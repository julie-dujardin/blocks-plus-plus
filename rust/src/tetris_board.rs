use crate::block::Block;
use crate::piece::Piece;
use crate::piece::Shape;
use godot::classes::InputEvent;
use godot::prelude::*;
use rand::prelude::IndexedRandom;
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
    fn add_current_piece_to_line(&mut self) {
        if let Some(piece) = &mut self.active_piece {
            let piece_bind = piece.bind_mut();
            let mut check_heights = HashSet::new();
            for block in piece_bind.blocks.iter_shared() {
                let block_ref = block.to::<Gd<Block>>();

                let (height, x) = {
                    let block = block_ref.bind();
                    (
                        19 - (block.board_offset.y + piece_bind.center_block_position.y).round()
                            as usize,
                        (block.board_offset.x + piece_bind.center_block_position.x).round(),
                    )
                };
                check_heights.insert(height);
                self.lines[height][x as usize] = Some(block_ref);
            }

            // TODO check height too high (>=16) => game over
            // TODO check full line & delete & add new
        };

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

    fn spawn_new_piece(&mut self) {
        // TODO use next piece

        let piece_scene: Gd<PackedScene> = load("res://scenes/piece.tscn");
        let mut piece = piece_scene.instantiate_as::<Piece>();

        {
            let mut piece_bind = piece.bind_mut();
            let mut rng = rand::thread_rng();
            piece_bind.set_shape(
                [
                    Shape::I,
                    Shape::O,
                    Shape::J,
                    Shape::L,
                    Shape::S,
                    Shape::Z,
                    Shape::T,
                ]
                .choose(&mut rng)
                .unwrap()
                .to_godot(),
            );
        }

        self.base_mut().add_child(piece.clone().upcast());
        self.active_piece = Some(piece);
    }

    fn push_new_line(&mut self) {
        self.lines.push(core::array::from_fn(|_| None));
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
        let mut insert_piece = false;
        if let Some(piece) = &mut self.active_piece {
            {
                let mut piece_bind = piece.bind_mut();

                if event.is_action_pressed("down".into()) {
                    piece_bind.drp();
                    insert_piece = true;
                } else if event.is_action_pressed("up".into()) {
                    piece_bind.rotate(true);
                } else if event.is_action_pressed("left".into()) {
                    piece_bind.mov(Vector2::LEFT);
                } else if event.is_action_pressed("right".into()) {
                    piece_bind.mov(Vector2::RIGHT);
                }
            }
        }
        if insert_piece {
            self.add_current_piece_to_line();
            self.spawn_new_piece();
        }
    }
}
