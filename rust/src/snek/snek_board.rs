use crate::constants::COLOR_SUCCESS;
use crate::snek::segment::Segment;
use godot::classes::{InputEvent, Timer};
use godot::prelude::*;
use phf::phf_map;
use rand::Rng;

const DIRECTIONS: phf::Map<&str, Vector2> = phf_map! {
    "up" => Vector2::UP,
    "left" => Vector2::LEFT,
    "down" => Vector2::DOWN,
    "right" => Vector2::RIGHT,
};

const SEGMENT_SIZE: Vector2 = Vector2::new(8., 8.);

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct SnekBoard {
    direction: Vector2,
    segments: Vec<Gd<Segment>>,
    head_position: Vector2,
    size: Vector2,

    base: Base<Node2D>,
}

#[godot_api]
impl SnekBoard {
    #[func]
    fn start_game(&mut self) {
        self.head_position = Vector2::new(5., 5.);
        self.add_segment();
        self.base().get_node_as::<Timer>("TimerMove").start();
        self.base_mut().show();
    }

    #[func]
    fn on_game_over(&self) {
        self.base().get_node_as::<Timer>("TimerMove").stop();
    }

    #[func]
    fn reset(&mut self) {
        while let Some(segment) = self.segments.pop() {
            segment.free();
        }
    }

    #[func]
    fn moved(&mut self) {
        self.head_position += self.direction;
        // Keep the snek in bounds w/ tp
        self.head_position = Vector2::new(
            if self.head_position.x < 0. {
                self.size.x - 1.
            } else {
                self.head_position.x % self.size.x
            },
            if self.head_position.y < 0. {
                self.size.y - 1.
            } else {
                self.head_position.y % self.size.y
            },
        );
        self.add_segment();
        self.segments.pop().unwrap().free();
    }

    #[func]
    fn add_goal(&mut self) {
        let mut rng = rand::thread_rng();
        let spawn_location = Vector2::new(
            rng.gen_range(0..self.size.x as u64) as f32,
            rng.gen_range(0..self.size.y as u64) as f32,
        );

        let segment_scene: Gd<PackedScene> = load("res://scenes/snek/segment.tscn");
        let mut goal = segment_scene.instantiate_as::<Segment>();
        goal.set_modulate(COLOR_SUCCESS);
        goal.set_position(spawn_location * SEGMENT_SIZE);
        self.base_mut().add_child(goal.upcast());
    }

    fn add_segment(&mut self) {
        let segment_scene: Gd<PackedScene> = load("res://scenes/snek/segment.tscn");
        let mut segment = segment_scene.instantiate_as::<Segment>();
        segment.set_position(self.head_position * SEGMENT_SIZE);
        self.base_mut().add_child(segment.clone().upcast());
        self.segments.insert(0, segment);
    }
}

// TODO: handle collisions, remove goal & score up

#[godot_api]
impl INode2D for SnekBoard {
    fn init(base: Base<Node2D>) -> Self {
        SnekBoard {
            direction: Vector2::RIGHT,
            segments: Vec::new(),
            head_position: Vector2::new(5., 5.),
            size: Vector2::new(20., 20.),
            base,
        }
    }

    fn ready(&mut self) {
        if self.base().get_parent().unwrap().is_class("Window".into()) {
            // If this class is the root node, make it playable for testing
            self.head_position = Vector2::new(3., 5.);
            self.add_segment();
            self.head_position = Vector2::new(4., 5.);
            self.add_segment();

            self.start_game();
            self.base().get_node_as::<Timer>("TimerGoal").start();
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        for (&key, direction) in DIRECTIONS.entries() {
            if event.is_action_pressed(key.into()) && self.direction + *direction != Vector2::ZERO {
                self.direction = *direction;
                self.moved();
                self.base().get_node_as::<Timer>("TimerMove").start();
            }
        }
    }
}
