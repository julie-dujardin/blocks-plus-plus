use crate::breakout::breakout_board::BreakoutBoard;
use crate::constants::{COLOR_FAILURE, COLOR_FOREGROUND, COLOR_SUCCESS};
use crate::tetris::block::Block;
use crate::tetris::piece::Piece;
use godot::classes::{ColorRect, InputEvent, NinePatchRect, Timer};
use godot::engine::{AnimationPlayer, Line2D};
use godot::prelude::*;
use std::collections::HashSet;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct TetrisBoard {
    active_piece: Option<Gd<Piece>>,
    next_pieces: Vec<Gd<Piece>>,
    lines: Vec<[Option<Gd<Block>>; 10]>,
    game_playing: bool,
    score_timed_out: bool,

    base: Base<Node2D>,
}

#[godot_api]
impl TetrisBoard {
    #[signal]
    fn game_over();

    #[signal]
    fn scored();

    #[signal]
    fn score_timed_out();

    #[func]
    fn reset(&mut self) {
        let mut piece_down_timer = self.base().get_node_as::<Timer>("TimerPieceDown");
        piece_down_timer.stop();

        for line in self.lines.iter() {
            for cell_opt in line {
                match cell_opt {
                    Some(cell) => cell.clone().free(),
                    None => {}
                }
            }
        }
        self.lines.clear();
        for _ in 0..20 {
            self.push_new_line();
        }

        for piece in &mut self.next_pieces {
            if piece.is_instance_valid() {
                piece.clone().free();
            }
        }
        self.next_pieces.clear();
        if let Some(piece) = &mut self.active_piece {
            if piece.is_instance_valid() {
                piece.clone().free();
            }
        }
        self.active_piece = None;
        self.base().get_node_as::<ColorRect>("NextFail").hide();

        self.base().get_node_as::<Line2D>("LineScoreTimeout").show();

        self.base_mut().hide();
        self.reset_color();
    }

    #[func]
    fn reset_color(&mut self) {
        self.set_color(COLOR_FOREGROUND);
    }

    fn set_color(&mut self, color: Color) {
        self.base()
            .get_node_as::<NinePatchRect>("BorderBoard")
            .set_modulate(color);
        self.base()
            .get_node_as::<NinePatchRect>("BorderNext")
            .set_modulate(color);
    }

    #[func]
    fn on_parent_game_over(&mut self) {
        self.game_playing = false;
        self.base().get_node_as::<Timer>("TimerSuccess").stop();
        self.base()
            .get_node_as::<AnimationPlayer>("ScoreTimeoutPlayer")
            .pause();
    }

    fn handle_game_over(&mut self, no_piece_left: bool) {
        if self.game_playing {
            self.game_playing = false;
            self.base_mut().emit_signal("game_over".into(), &[]);

            if no_piece_left {
                self.base_mut()
                    .get_node_as::<NinePatchRect>("BorderNext")
                    .set_modulate(COLOR_FAILURE);
            } else {
                self.base_mut()
                    .get_node_as::<NinePatchRect>("BorderBoard")
                    .set_modulate(COLOR_FAILURE);
            }
        }
    }

    fn score_up(&mut self, count: usize) {
        self.base()
            .get_node_as::<NinePatchRect>("BorderBoard")
            .set_modulate(COLOR_SUCCESS);
        self.base().get_node_as::<Timer>("TimerSuccess").start();
        self.base_mut()
            .emit_signal("scored".into(), &[(count as i64 * 3).to_variant()]);

        if !self.score_timed_out {
            self.base()
                .get_node_as::<AnimationPlayer>("ScoreTimeoutPlayer")
                .seek(0.);
        }

        if !self.base().get_parent().unwrap().is_class("Window".into()) {
            let mut breakout_board = self
                .base()
                .get_parent()
                .unwrap()
                .get_node_as::<BreakoutBoard>("BreakoutBoard");
            breakout_board.show();
            let mut breakout_board_bind = breakout_board.bind_mut();
            breakout_board_bind.push_new_line(count as u64);
            breakout_board_bind.on_game_started();
        }
    }

    #[func]
    fn on_parent_score_timed_out(&mut self) {
        self.base().get_node_as::<Line2D>("LineScoreTimeout").hide();
    }

    #[func]
    fn on_score_timed_out(&mut self, _anim_name: Variant) {
        self.base_mut().emit_signal("score_timed_out".into(), &[]);
        self.score_timed_out = true;
    }

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
        let mut too_high = false;
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
                    too_high = true;
                }
            }

            for height in check_heights {
                if self.lines[height].iter().filter(|c| c.is_some()).count() == 10 {
                    removed_lines += 1;
                    lowest_removed_height = if height < lowest_removed_height {
                        height
                    } else {
                        lowest_removed_height
                    };
                }
            }
        };
        if too_high {
            self.handle_game_over(false);
        }

        if removed_lines > 0 {
            self.score_up(removed_lines);
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
    }

    #[func]
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

    pub fn push_random_piece(&mut self) {
        godot_print!("pushing new piece");
        let piece = Piece::spawn_random();
        self.base_mut().add_child(piece.clone().upcast());
        self.add_next_piece(piece);
    }

    pub fn add_next_piece(&mut self, mut piece: Gd<Piece>) {
        if !self.game_playing {
            self.game_playing = true;
            self.base()
                .get_node_as::<AnimationPlayer>("ScoreTimeoutPlayer")
                .play_ex()
                .name("score_timeout".into())
                .done();
            self.base()
                .get_node_as::<AnimationPlayer>("ScoreTimeoutPlayer")
                .seek(0.);
        }

        let mut piece_down_timer = self.base().get_node_as::<Timer>("TimerPieceDown");
        piece_down_timer.start();

        if !self.next_pieces.is_empty() {
            piece.free();
            return;
        }
        piece.clone().reparent(self.base_mut().to_godot().upcast());
        let background_position = self
            .base()
            .get_node_as::<NinePatchRect>("BorderNext")
            .get_position()
            + Vector2::new(1., 25.);
        piece.set_position(background_position);
        {
            let mut piece_bind = piece.bind_mut();
            piece_bind.position_for_ui();
        }
        self.next_pieces.push(piece);
        if self.active_piece.is_none() {
            self.spawn_new_piece();

            if self.base().get_parent().unwrap().is_class("Window".into()) {
                // If this class is the root node, keep spawning new pieces
                self.push_random_piece();
            }
        }
    }

    pub fn spawn_new_piece(&mut self) {
        if self.game_playing {
            let piece_opt = self.next_pieces.pop();

            match piece_opt {
                None => {
                    if self.active_piece.is_none() {
                        self.base().get_node_as::<ColorRect>("NextFail").show();
                        self.handle_game_over(true);
                    }
                }
                Some(mut piece) => {
                    {
                        let mut piece_bind = piece.bind_mut();
                        piece_bind.center_block_position = Vector2::new(5., 2.);
                        piece_bind.update_position();
                    }

                    self.active_piece = Some(piece);
                }
            }
        }
    }

    fn push_new_line(&mut self) {
        self.lines.push(core::array::from_fn(|_| None));
    }

    #[func]
    fn down_piece(&mut self) -> bool {
        if !self.game_playing {
            return false;
        }

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

            if self.base().get_parent().unwrap().is_class("Window".into()) {
                // If this class is the root node, keep spawning new pieces
                self.push_random_piece();
            }
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

    fn update_piece_side_lines(&mut self) {
        let mut line_left = self.base().get_node_as::<Line2D>("LineLeft");
        let mut line_right = self.base().get_node_as::<Line2D>("LineRight");

        if let Some(piece) = &mut self.active_piece {
            let piece_bind = piece.bind();
            let (top_left, bottom_right) = piece_bind.get_bounds();
            let block_width = piece_bind.block_size.x;
            line_left.set_position(Vector2::new(top_left.x * block_width, 0.));
            line_right.set_position(Vector2::new((bottom_right.x + 1.) * block_width, 0.));
            line_left.set_visible(true);
            line_right.set_visible(true);
        } else {
            line_left.set_visible(false);
            line_right.set_visible(false);
        }
    }
}

#[godot_api]
impl INode2D for TetrisBoard {
    fn init(base: Base<Node2D>) -> Self {
        let mut tb = TetrisBoard {
            active_piece: None,
            next_pieces: vec![],
            lines: vec![],
            game_playing: false,
            score_timed_out: false,
            base,
        };

        for _ in 0..20 {
            tb.push_new_line();
        }

        tb
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.active_piece.is_some() && self.game_playing {
            if event.is_action_pressed("down".into()) {
                self.drop_piece();
            } else if event.is_action_pressed("up".into()) {
                self.rotate_piece(true);
            } else if event.is_action_pressed("left".into()) {
                self.left_piece();
            } else if event.is_action_pressed("right".into()) {
                self.right_piece();
            }
            self.update_piece_side_lines()
        }
    }

    fn ready(&mut self) {
        if self.base().get_parent().unwrap().is_class("Window".into()) {
            // If this class is the root node, make it playable for testing
            self.base_mut().show();
            self.push_random_piece();
        }
    }
}
