use crate::breakout::breakout_player::BreakoutPlayer;
use crate::breakout::brick::Brick;
use godot::builtin::Vector2;
use godot::classes::{INode2D, Node2D, PackedScene};
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{godot_api, load, GodotClass};
use crate::breakout::ball::Ball;

const BRICK_PER_LINE: usize = 10;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct BreakoutBoard {
    #[export]
    score: i64,
    lines: Vec<[Option<Gd<Brick>>; BRICK_PER_LINE]>,

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

        self.base_mut().hide();
    }

    #[func]
    fn on_broke_brick(&mut self) {
        self.score += 1;
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
        // TODO move existing lines down

        let brick_scene: Gd<PackedScene> = load("res://scenes/breakout/brick.tscn");
        let new_line = core::array::from_fn(|i| {
            let mut brick = brick_scene.instantiate_as::<Brick>();

            let brick_size = {
                let brick_bind = brick.bind_mut();
                brick_bind.get_size()
            };
            brick.set_position(Vector2::new((brick_size.x + 2.) * i as f32 + 2., 10.));
            self.base_mut().add_child(brick.clone().upcast());

            Some(brick)
        });

        self.lines.insert(0, new_line);
    }
}

#[godot_api]
impl INode2D for BreakoutBoard {
    fn init(base: Base<Node2D>) -> Self {
        BreakoutBoard {
            score: 0,
            lines: vec![],
            base,
        }
    }

    fn ready(&mut self) {
        self.set_movement(false);
    }
}
