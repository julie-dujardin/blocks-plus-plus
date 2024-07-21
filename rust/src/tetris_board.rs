use crate::block::Block;
use crate::piece::Piece;
use godot::classes::InputEvent;
use godot::engine::ColorRect;
use godot::prelude::*;
use std::collections::HashSet;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct TetrisBoard {
    #[export]
    score: i64,
    active_piece: Option<Gd<Piece>>,
    next_pieces: Vec<Gd<Piece>>,
    lines: Vec<[Option<Gd<Block>>; 10]>,

    base: Base<Node2D>,
}

#[godot_api]
impl TetrisBoard {
    fn check_collision_with_lines(&mut self) -> bool {
        if let Some(piece) = &mut self.active_piece {
            let piece_bind = piece.bind_mut();

            for block in piece_bind.blocks.iter_shared() {
                let block_ref = block.to::<Gd<Block>>();

                let (height, x) = {
                    let block = block_ref.bind();
                    (
                        19 - (block.board_offset.y + piece_bind.center_block_position.y).round()
                            as usize,
                        (block.board_offset.x + piece_bind.center_block_position.x).round()
                            as usize,
                    )
                };

                if self.lines.get(height).is_some() && self.lines[height][x].is_some() {
                    return true;
                }
            }
        };
        false
    }

    fn add_current_piece_to_line(&mut self) {
        let mut removed_lines = 0;
        let mut check_heights = HashSet::new();
        let mut lowest_removed_height = 21;
        if let Some(piece) = &mut self.active_piece {
            let piece_bind = piece.bind_mut();
            for block in piece_bind.blocks.iter_shared() {
                let block_ref = block.to::<Gd<Block>>();

                let (height, x) = {
                    let block = block_ref.bind();
                    (
                        19 - (block.board_offset.y + piece_bind.center_block_position.y).round()
                            as usize,
                        (block.board_offset.x + piece_bind.center_block_position.x).round()
                            as usize,
                    )
                };
                check_heights.insert(height);
                self.lines[height][x] = Some(block_ref);

                if height >= 16 {
                    // TODO handle game over
                }
            }

            for height in check_heights {
                if self.lines[height].iter().filter(|c| c.is_some()).count() == 10 {
                    self.score += 1;
                    removed_lines += 1;
                    lowest_removed_height = if height < lowest_removed_height {
                        height
                    } else {
                        lowest_removed_height
                    };
                }
            }
        };

        if removed_lines > 0 {
            for _ in lowest_removed_height..lowest_removed_height + removed_lines {
                for cell_opt in &mut self.lines[lowest_removed_height] {
                    let cell = cell_opt.as_mut().unwrap();
                    cell.clone().free();
                }
                self.lines.remove(lowest_removed_height);
                self.push_new_line();
            }

            for height in lowest_removed_height..20 {
                for i in 0..10 {
                    if let Some(block_ref) = &mut self.lines[height][i] {
                        let mut block = block_ref.bind_mut();
                        block.board_offset = Vector2::new(
                            block.board_offset.x,
                            block.board_offset.y + removed_lines as f32,
                        );
                        block.update_position();
                    }
                }
            }
        }

        self.active_piece = None;
        self.godot_print_lines();
    }

    fn godot_print_lines(&self) {
        godot_print!("current lines:");
        for line in self.lines.iter().rev() {
            let mut line_str = String::new();
            for cell in line {
                line_str.push(match cell {
                    Some(_) => '█',
                    None => ' ',
                })
            }
            godot_print!("|{}|", line_str);
        }
    }

    pub fn add_next_piece(&mut self, mut piece: Gd<Piece>) {
        if self.next_pieces.len() > 0 {
            piece.free();
            return;
        }
        piece.clone().reparent(self.base_mut().to_godot().upcast());
        let background_position = self
            .base()
            .get_node_as::<ColorRect>("BorderNext")
            .get_position() + Vector2::new(1., 25.);
        piece.set_position(background_position);
        {
            let mut piece_bind = piece.bind_mut();
            piece_bind.position_for_ui();
        }
        self.next_pieces.push(piece);
        if self.active_piece.is_none() {
            self.spawn_new_piece();
        }
    }

    fn spawn_new_piece(&mut self) {
        let piece_opt = self.next_pieces.pop();
        match piece_opt {
            None => {
                // TODO game over
            }
            Some(mut piece) => {
                {
                    let mut piece_bind = piece.bind_mut();
                    piece_bind.center_block_position = Vector2::new(5., 2.);
                    piece_bind.update_position();
                }

                self.base_mut().add_child(piece.clone().upcast());
                self.active_piece = Some(piece);
            }
        }
    }

    fn push_new_line(&mut self) {
        self.lines.push(core::array::from_fn(|_| None));
    }

    #[func]
    fn down_piece(&mut self) -> bool {
        let mut reached_bottom = false;
        if let Some(piece) = &mut self.active_piece {
            let mut piece_bind = piece.bind_mut();
            reached_bottom = !piece_bind.down()
        };

        if self.check_collision_with_lines() {
            if let Some(piece) = &mut self.active_piece {
                let mut piece_bind = piece.bind_mut();
                piece_bind.mov(Vector2::UP);
            }
            reached_bottom = true;
        }
        if reached_bottom {
            self.add_current_piece_to_line();
            self.spawn_new_piece();
            return false;
        }
        true
    }

    fn drop_piece(&mut self) {
        while self.down_piece() {}
    }

    fn rotate_piece(&mut self, clockwise: bool) {
        if let Some(piece) = &mut self.active_piece {
            let mut piece_bind = piece.bind_mut();
            piece_bind.rotate(clockwise);
        }
        if self.check_collision_with_lines() {
            self.rotate_piece(false);
        }
    }

    fn left_piece(&mut self) {
        if let Some(piece) = &mut self.active_piece {
            let mut piece_bind = piece.bind_mut();
            piece_bind.mov(Vector2::LEFT);
        }
        if self.check_collision_with_lines() {
            self.right_piece();
        }
    }

    fn right_piece(&mut self) {
        if let Some(piece) = &mut self.active_piece {
            let mut piece_bind = piece.bind_mut();
            piece_bind.mov(Vector2::RIGHT);
        }
        if self.check_collision_with_lines() {
            self.left_piece();
        }
    }
}

#[godot_api]
impl INode2D for TetrisBoard {
    fn init(base: Base<Node2D>) -> Self {
        let mut tb = TetrisBoard {
            score: 0,
            active_piece: None,
            next_pieces: vec![],
            lines: vec![],
            base,
        };

        for _ in 0..20 {
            tb.push_new_line();
        }

        tb
    }

    fn ready(&mut self) {
        self.spawn_new_piece();
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.active_piece.is_some() {
            {
                if event.is_action_pressed("down".into()) {
                    self.drop_piece();
                } else if event.is_action_pressed("up".into()) {
                    self.rotate_piece(true);
                } else if event.is_action_pressed("left".into()) {
                    self.left_piece();
                } else if event.is_action_pressed("right".into()) {
                    self.right_piece();
                }
            }
        }
    }
}
