use crate::constants::{COLOR_FAILURE, COLOR_FOREGROUND, COLOR_SUCCESS};
use crate::snek::goal::Goal;
use crate::snek::segment::Segment;
use godot::classes::{InputEvent, NinePatchRect, Timer};
use godot::engine::ColorRect;
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
    goals: Vec<Gd<Goal>>,
    head_position: Vector2,
    size: Vector2,
    just_scored: bool,
    can_move: bool,

    base: Base<Node2D>,
}

#[godot_api]
impl SnekBoard {
    #[signal]
    fn game_over();

    #[signal]
    fn scored();

    #[func]
    fn on_previous_scored_up(&mut self, _count: Variant) {
        if self.can_move {
            self.add_goal();
        } else {
            self.start_game();
        }
    }

    #[func]
    fn start_game(&mut self) {
        self.head_position = Vector2::new(5., 5.);
        self.add_segment();
        self.base().get_node_as::<Timer>("TimerMove").start();
        self.base_mut().show();
        self.can_move = true;
    }

    #[func]
    fn on_parent_game_over(&mut self) {
        self.base().get_node_as::<Timer>("TimerMove").stop();
        self.base().get_node_as::<Timer>("TimerGoal").stop();
        self.can_move = false;
    }

    #[func]
    fn on_game_over(&mut self) {
        self.set_color(COLOR_FAILURE);
        if self.base().get_parent().unwrap().is_class("Window".into()) {
            // If this class is the root node, make it playable for testing
            self.on_parent_game_over();
        } else {
            self.base_mut().emit_signal("game_over".into(), &[]);
        }
    }

    #[func]
    fn reset(&mut self) {
        while let Some(segment) = self.segments.pop() {
            segment.free();
        }
        while let Some(goal) = self.goals.pop() {
            // TODO remove goal from goals on hit
            if goal.is_instance_valid() {
                goal.free();
            }
        }
        self.base_mut().hide();
        self.set_color(COLOR_FOREGROUND);
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
        if self.just_scored {
            self.just_scored = false;
        } else {
            self.segments.pop().unwrap().free();
        }
    }

    #[func]
    fn add_goal(&mut self) {
        let mut rng = rand::thread_rng();
        let spawn_location = Vector2::new(
            rng.gen_range(0..self.size.x as u64) as f32,
            rng.gen_range(0..self.size.y as u64) as f32,
        );

        let goal_scene: Gd<PackedScene> = load("res://scenes/snek/Goal.tscn");
        let mut goal = goal_scene.instantiate_as::<Goal>();
        goal.set_modulate(COLOR_SUCCESS);
        goal.set_position(spawn_location * SEGMENT_SIZE);
        self.base_mut().add_child(goal.clone().upcast());
        self.goals.push(goal);
    }

    #[func]
    fn score_up(&mut self) {
        self.base_mut()
            .emit_signal("scored".into(), &[10.to_variant()]);
        self.just_scored = true;
        self.set_color(COLOR_SUCCESS);
        self.base_mut()
            .get_node_as::<Timer>("TimerGoalTimeout")
            .start();
    }

    fn add_segment(&mut self) {
        let segment_scene: Gd<PackedScene> = load("res://scenes/snek/segment.tscn");
        let mut segment = segment_scene.instantiate_as::<Segment>();
        segment.set_position(self.head_position * SEGMENT_SIZE);
        segment.connect("score_up".into(), self.base().callable("score_up"));
        segment.connect("game_over".into(), self.base().callable("on_game_over"));
        self.base_mut().add_child(segment.clone().upcast());
        self.segments.insert(0, segment);
    }

    #[func]
    fn reset_color(&mut self) {
        self.set_color(COLOR_FOREGROUND);
    }

    fn set_color(&mut self, color: Color) {
        self.base_mut()
            .get_node_as::<NinePatchRect>("Border")
            .set_modulate(color);
    }
}

#[godot_api]
impl INode2D for SnekBoard {
    fn init(base: Base<Node2D>) -> Self {
        SnekBoard {
            direction: Vector2::RIGHT,
            segments: Vec::new(),
            goals: Vec::new(),
            head_position: Vector2::new(5., 5.),
            size: Vector2::new(30., 36.),
            just_scored: false,
            can_move: false,
            base,
        }
    }

    fn ready(&mut self) {
        if self.base().get_parent().unwrap().is_class("Window".into()) {
            // If this class is the root node, make it playable for testing
            self.head_position = Vector2::new(2., 5.);
            self.add_segment();
            self.head_position = Vector2::new(3., 5.);
            self.add_segment();
            self.head_position = Vector2::new(4., 5.);
            self.add_segment();

            self.start_game();
            self.base().get_node_as::<Timer>("TimerGoal").start();
            self.base().get_node_as::<ColorRect>("Background").show();
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.can_move {
            for (&key, direction) in DIRECTIONS.entries() {
                if event.is_action_pressed(key.into())
                    && self.direction + *direction != Vector2::ZERO
                {
                    self.direction = *direction;
                    self.moved();
                    self.base().get_node_as::<Timer>("TimerMove").start();
                }
            }
        }
    }
}
