use godot::builtin::Vector2;
use godot::classes::{CollisionShape2D, IStaticBody2D, StaticBody2D};
use godot::obj::{Base, WithBaseField};
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=StaticBody2D)]
pub struct Brick {
    base: Base<StaticBody2D>,
}

#[godot_api]
impl Brick {
    pub fn get_size(&self) -> Vector2 {
        self.base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D")
            .get_shape()
            .unwrap()
            .get_rect()
            .size
    }
}

#[godot_api]
impl IStaticBody2D for Brick {
    fn init(base: Base<StaticBody2D>) -> Self {
        Brick { base }
    }
}
