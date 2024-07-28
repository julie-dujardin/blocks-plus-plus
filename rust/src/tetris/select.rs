use crate::constants::{COLOR_FOREGROUND, COLOR_SUCCESS};
use crate::tetris::piece::Piece;
use crate::tetris::tetris_board::TetrisBoard;
use godot::classes::{InputEvent, Sprite2D, Timer};
use godot::prelude::*;
use rand::prelude::IndexedRandom;
use std::f32::consts::PI;

const SELECT_COUNT: usize = 3;

#[derive(Clone, Copy, PartialEq)]
enum InputOptions {
    Up,
    Left,
    Right,
}

const INPUT_CHOICES: [InputOptions; 3] =
    [InputOptions::Up, InputOptions::Left, InputOptions::Right];

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Select {
    piece: Gd<Piece>,
    sequence: Vec<InputOptions>,
    prompts: Vec<Gd<Sprite2D>>,
    curr_check_index: usize,
    game_over: bool,

    base: Base<Node2D>,
}

#[godot_api]
impl Select {
    #[func]
    fn success_reset(&mut self) {
        self.reset_color();

        self.sequence = Select::random_sequence();
        self.piece = Piece::spawn_random();
        self.ready();
    }

    #[func]
    fn reset(&mut self) {
        self.base_mut().hide();
        self.game_over = false;
        self.reset_color();
        self.sequence = Select::random_sequence();
        self.piece.clone().free();
        self.piece = Piece::spawn_random();
        self.ready();
    }

    #[func]
    fn handle_game_over(&mut self) {
        self.game_over = true;

        self.base_mut().get_node_as::<Timer>("TimerSuccess").stop();
    }

    fn check_input(&mut self, input: InputOptions) {
        if input == self.sequence[self.curr_check_index] {
            self.prompts[self.curr_check_index].set_modulate(COLOR_SUCCESS);
            self.curr_check_index += 1;
            if self.curr_check_index >= SELECT_COUNT {
                self.success();
            }
        } else {
            self.reset_color();
        }
    }

    fn reset_color(&mut self) {
        self.curr_check_index = 0;
        for prompt in &mut self.prompts {
            prompt.set_modulate(COLOR_FOREGROUND);
        }
    }

    fn success(&mut self) {
        if !self.game_over {
            let mut tetris_board = self
                .base()
                .get_parent()
                .unwrap()
                .get_node_as::<TetrisBoard>("Tetris");
            tetris_board.show();
            let mut tetris_board = tetris_board.bind_mut();
            tetris_board.add_next_piece(self.piece.clone());

            self.base_mut().get_node_as::<Timer>("TimerSuccess").start();
        }
    }

    fn random_sequence() -> Vec<InputOptions> {
        let mut rng = rand::thread_rng();

        (0..SELECT_COUNT)
            .map(|_| *INPUT_CHOICES.choose(&mut rng).unwrap())
            .collect()
    }
}

#[godot_api]
impl INode2D for Select {
    fn init(base: Base<Node2D>) -> Self {
        Select {
            piece: Piece::spawn_random(),
            sequence: Select::random_sequence(),
            prompts: vec![],
            curr_check_index: 0,
            game_over: false,
            base,
        }
    }

    fn ready(&mut self) {
        for (i, expected_input) in self.sequence.iter().enumerate() {
            let mut select_node = self
                .base()
                .get_node_as::<Sprite2D>(format!("SelectOption{}", i.to_string().as_str()));

            match expected_input {
                InputOptions::Up => select_node.set_rotation(-PI / 2.),
                InputOptions::Left => select_node.set_rotation(-PI),
                InputOptions::Right => select_node.set_rotation(0.),
            }

            self.prompts.push(select_node);
        }

        {
            let piece = &mut self.piece.clone();
            self.base_mut().add_child(piece.clone().upcast());
        };
        {
            let mut piece_bind = self.piece.bind_mut();
            piece_bind.position_for_ui();
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.base().is_visible() && !self.game_over && self.curr_check_index < SELECT_COUNT {
            if event.is_action_pressed("up".into()) {
                self.check_input(InputOptions::Up);
            } else if event.is_action_pressed("left".into()) {
                self.check_input(InputOptions::Left);
            } else if event.is_action_pressed("right".into()) {
                self.check_input(InputOptions::Right);
            } else if event.is_action_pressed("down".into()) {
                self.reset_color();
            }
        }
    }
}
