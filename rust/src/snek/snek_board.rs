use crate::snek::segment::Segment;
use godot::classes::{InputEvent, Timer};
use godot::prelude::*;
use phf::phf_map;

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

    base: Base<Node2D>,
}

#[godot_api]
impl SnekBoard {
    #[func]
    fn start_game(&mut self) {
        self.head_position = Vector2::new(5., 5.);
        self.base().get_node_as::<Timer>("TimerMove").start();
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
        self.add_segment();
        self.segments.pop().unwrap().free();
    }

    fn add_segment(&mut self) {
        let segment_scene: Gd<PackedScene> = load("res://scenes/snek/segment.tscn");
        let mut segment = segment_scene.instantiate_as::<Segment>();
        segment.set_position(self.head_position * SEGMENT_SIZE);
        self.base_mut().add_child(segment.clone().upcast());
        self.segments.insert(0, segment);
    }
}

#[godot_api]
impl INode2D for SnekBoard {
    fn init(base: Base<Node2D>) -> Self {
        SnekBoard {
            direction: Vector2::RIGHT,
            segments: Vec::new(),
            head_position: Vector2::new(5., 5.),
            base,
        }
    }

    fn ready(&mut self) {
        self.add_segment();

        if self.base().get_parent().unwrap().is_class("Window".into()) {
            // If this class is the root node, make it playable for testing
            self.start_game();
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
