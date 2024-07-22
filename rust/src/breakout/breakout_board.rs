use crate::breakout::ball::Ball;
use crate::breakout::breakout_player::BreakoutPlayer;
use crate::breakout::brick::Brick;
use godot::builtin::{Variant, Vector2};
use godot::classes::{INode2D, Node2D, PackedScene};
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{godot_api, load, GodotClass};

const BRICK_PER_LINE: usize = 10;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct BreakoutBoard {
    #[export]
    score: i64,
    bricks: Vec<Gd<Brick>>,
    brick_size: Vector2,

    base: Base<Node2D>,
}

#[godot_api]
impl BreakoutBoard {
    #[signal]
    fn game_over();

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
    }

    #[func]
    fn on_broke_brick(&mut self, brick_var: Variant) {
        self.score += 1;
        let brick = brick_var.to::<Gd<Brick>>();
        self.bricks.retain(|x| *x != brick);
    }

    #[func]
    pub fn on_game_started(&mut self) {
        self.set_movement(true);
    }

    #[func]
    fn on_parent_game_over(&mut self) {
        self.set_movement(false);
    }

    #[func]
    fn on_game_over(&mut self) {
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

    pub fn push_new_line(&mut self) {
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

#[godot_api]
impl INode2D for BreakoutBoard {
    fn init(base: Base<Node2D>) -> Self {
        BreakoutBoard {
            score: 0,
            bricks: vec![],
            brick_size: Vector2::ZERO,
            base,
        }
    }

    fn ready(&mut self) {
        self.set_movement(false);
    }
}
