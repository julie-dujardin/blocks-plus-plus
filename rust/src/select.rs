use crate::piece::{Piece, Shape};
use godot::classes::{InputEvent, Sprite2D};
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

    base: Base<Node2D>,
}

#[godot_api]
impl Select {
    fn check_input(&mut self, input: InputOptions) {
        if input == self.sequence[self.curr_check_index] {
            self.prompts[self.curr_check_index]
                .set_modulate(Color::from_rgb(1., 1., 1.));
            self.curr_check_index += 1;
            if self.curr_check_index >= SELECT_COUNT {
                self.success();
            }
        }
        else {
            self.reset();
        }
    }

    fn reset(&mut self) {
        if self.curr_check_index > 0 {
            self.curr_check_index = 0;
            for mut prompt in &mut self.prompts {
                prompt.set_modulate(Color::from_rgb(105./255., 105./255., 105./255.));
            }
        }
    }

    fn success(&mut self) {
        self.reset();
        // TODO move piece
        self.piece.clone().free();

        self.sequence = Select::random_sequence();
        self.piece = Piece::spawn_random();
        self.ready();
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
        self.piece.set_position(Vector2::new(42., 2.));
        {
            let shape = {
                let piece_bind = &self.piece.bind();
                piece_bind.shape.clone()
            };
            match shape {
                Shape::I => {
                    self.piece.set_position(Vector2::new(42., 22.));
                }
                Shape::T => {
                    self.piece.set_position(Vector2::new(42., 22.));
                }
                Shape::J => {
                    self.piece.set_rotation(PI / 2.);
                    self.piece.set_position(Vector2::new(62., 22.));
                }
                Shape::L => {
                    self.piece.set_rotation(PI / 2.);
                    self.piece.set_position(Vector2::new(62., 2.));
                }
                _ => {}
            }
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("up".into()) {
            self.check_input(InputOptions::Up);
        } else if event.is_action_pressed("left".into()) {
            self.check_input(InputOptions::Left);
        } else if event.is_action_pressed("right".into()) {
            self.check_input(InputOptions::Right);
        } else if event.is_action_pressed("down".into()) {
            self.reset();
        }
    }
}
