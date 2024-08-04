use crate::breakout::ball::Ball;
use crate::breakout::breakout_player::BreakoutPlayer;
use crate::breakout::brick::Brick;
use crate::constants::{COLOR_FAILURE, COLOR_FOREGROUND, COLOR_SUCCESS};
use godot::builtin::{Color, Variant, Vector2};
use godot::classes::{AnimationPlayer, INode2D, Line2D, Node2D, PackedScene, StaticBody2D, Timer};
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::*;

const BRICK_PER_LINE: usize = 10;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct BreakoutBoard {
    bricks: Vec<Gd<Brick>>,
    brick_size: Vector2,
    score_timed_out: bool,

    base: Base<Node2D>,
}

#[godot_api]
impl BreakoutBoard {
    #[signal]
    fn game_over();

    #[signal]
    fn scored();

    #[signal]
    fn next_game_activate();

    #[signal]
    fn score_timed_out();

    #[func]
    fn reset(&mut self) {
        let mut player = self.base().get_node_as::<BreakoutPlayer>("BreakoutPlayer");
        player.set_position(Vector2::new(192., 256.));

        let mut ball = self.base().get_node_as::<Ball>("Ball");
        let mut ball_bind = ball.bind_mut();
        ball_bind.reset();

        for brick in &self.bricks {
            brick.clone().free();
        }
        self.bricks.clear();

        self.base_mut().hide();
        self.reset_color();
    }

    #[func]
    fn reset_color(&mut self) {
        self.set_color(COLOR_FOREGROUND);
    }

    fn set_color(&mut self, color: Color) {
        self.base()
            .get_node_as::<StaticBody2D>("Walls")
            .set_modulate(color);
    }

    #[func]
    fn on_broke_brick(&mut self, brick_var: Variant, super_ball: Variant) {
        let mut brick_has_exploded = false;
        let mut brick = brick_var.to::<Gd<Brick>>();
        {
            let mut brick_bind = brick.bind_mut();
            if brick_bind.is_exploding {
                brick_has_exploded = true;
            } else {
                brick_bind.explode();
            }
        }
        if !brick_has_exploded {
            self.set_color(COLOR_SUCCESS);
            self.base().get_node_as::<Timer>("TimerSuccess").start();
            self.base_mut()
                .emit_signal("scored".into(), &[1.to_variant()]);
            self.bricks.retain(|x| *x != brick);

            if super_ball.to::<bool>() {
                self.base_mut()
                    .emit_signal("next_game_activate".into(), &[]);
            }
        }
        if !self.score_timed_out {
            self.base()
                .get_node_as::<AnimationPlayer>("ScoreTimeoutPlayer")
                .seek(0.);
        }
    }

    #[func]
    pub fn on_game_started(&mut self) {
        self.set_movement(true);
        self.base()
            .get_node_as::<AnimationPlayer>("ScoreTimeoutPlayer")
            .play_ex()
            .name("score_timeout".into())
            .done();
        self.base()
            .get_node_as::<AnimationPlayer>("ScoreTimeoutPlayer")
            .seek(0.);
    }

    #[func]
    fn on_parent_game_over(&mut self) {
        self.set_movement(false);
        self.base().get_node_as::<Timer>("TimerSuccess").stop();
        self.base()
            .get_node_as::<AnimationPlayer>("ScoreTimeoutPlayer")
            .pause();
    }

    #[func]
    fn on_game_over(&mut self) {
        self.set_color(COLOR_FAILURE);
        self.base_mut().emit_signal("game_over".into(), &[]);
    }

    fn set_movement(&self, can_move: bool) {
        let mut player = self.base().get_node_as::<BreakoutPlayer>("BreakoutPlayer");
        let mut player_bind = player.bind_mut();
        player_bind.can_move = can_move;

        let mut ball = self.base().get_node_as::<Ball>("Ball");
        let mut ball_bind = ball.bind_mut();
        ball_bind.set_movement(can_move);
    }

    #[func]
    pub fn push_new_line(&mut self, count: u64) {
        for _ in 0..count {
            for brick in &mut self.bricks {
                brick.move_local_y(self.brick_size.y + 10.);
            }

            let brick_scene: Gd<PackedScene> = load("res://scenes/breakout/brick.tscn");

            for i in 0..BRICK_PER_LINE {
                let mut brick = brick_scene.instantiate_as::<Brick>();

                self.brick_size = {
                    let brick_bind = brick.bind_mut();
                    brick_bind.get_size()
                };
                brick.set_position(Vector2::new((self.brick_size.x + 2.) * i as f32 + 2., 10.));
                self.base_mut().add_child(brick.clone().upcast());

                self.bricks.push(brick);
            }
        }
    }

    #[func]
    fn on_parent_score_timed_out(&mut self) {
        self.base()
            .get_node_as::<Line2D>("ScoreTimeoutLine").hide();
    }

    #[func]
    fn on_score_timed_out(&mut self, _anim_name: Variant) {
        self.base_mut().emit_signal("score_timed_out".into(), &[]);
        self.score_timed_out = true;
    }
}

#[godot_api]
impl INode2D for BreakoutBoard {
    fn init(base: Base<Node2D>) -> Self {
        BreakoutBoard {
            bricks: vec![],
            brick_size: Vector2::ZERO,
            score_timed_out: false,
            base,
        }
    }

    fn ready(&mut self) {
        if self.base().get_parent().unwrap().is_class("Window".into()) {
            // If this class is the root node, make it playable for testing
            self.base_mut().show();
            self.set_movement(true);
            self.push_new_line(3);
            self.base().get_node_as::<Timer>("TimerNewLine").start();
        } else {
            self.set_movement(false);
        }
    }
}
