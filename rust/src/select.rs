use crate::piece::Piece;
use godot::classes::InputEvent;
use godot::engine::Sprite2D;
use godot::prelude::*;
use rand::prelude::IndexedRandom;
use std::f32::consts::PI;

const SELECT_COUNT: usize = 3;

#[derive(Clone, PartialEq)]
enum InputOptions {
    UP,
    LEFT,
    RIGHT,
}

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Select {
    piece: Gd<Piece>,
    sequence: Vec<InputOptions>,
    select_options: Vec<Gd<Sprite2D>>,
    curr_check_index: usize,

    base: Base<Node2D>,
}

#[godot_api]
impl Select {
    fn check_input(&mut self, input: InputOptions) {
        if input == self.sequence[self.curr_check_index] {}
    }
}

#[godot_api]
impl INode2D for Select {
    fn init(base: Base<Node2D>) -> Self {
        let mut rng = rand::thread_rng();

        let mut sl = Select {
            piece: Piece::spawn_random(),
            sequence: [InputOptions::UP, InputOptions::LEFT, InputOptions::RIGHT]
                .choose_multiple(&mut rng, SELECT_COUNT)
                .cloned()
                .collect(),
            select_options: vec![],
            curr_check_index: 0,
            base,
        };

        // for (i, expected_input) in sl.sequence.iter().enumerate() {
        //     let mut select_node = sl
        //         .base()
        //         .get_node_as::<Sprite2D>(format!("SelectOption{}", i.to_string().as_str()));
        //     match expected_input {
        //         InputOptions::UP => select_node.rotate(-PI / 2.),
        //         InputOptions::LEFT => select_node.rotate(-PI),
        //         _ => {}
        //     }
        //     sl.select_options.push(select_node);
        // }

        sl
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("up".into()) {
            self.check_input(InputOptions::UP);
        } else if event.is_action_pressed("left".into()) {
            self.check_input(InputOptions::LEFT);
        } else if event.is_action_pressed("right".into()) {
            self.check_input(InputOptions::RIGHT);
        }
    }
}
