use crate::breakout::brick::Brick;
use godot::builtin::Vector2;
use godot::classes::{INode2D, Node2D, PackedScene};
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{godot_api, load, GodotClass};

const BRICK_PER_LINE: usize = 10;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct BreakoutBoard {
    #[export]
    score: i64,
    game_over: bool,
    lines: Vec<[Option<Gd<Brick>>; BRICK_PER_LINE]>,

    base: Base<Node2D>,
}

#[godot_api]
impl BreakoutBoard {
    #[func]
    fn on_broke_brick(&mut self) {
        self.score += 1;
    }

    fn push_new_line(&mut self) {
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
            game_over: false,
            lines: vec![],
            base,
        }
    }

    fn ready(&mut self) {
        self.push_new_line();
    }
}
