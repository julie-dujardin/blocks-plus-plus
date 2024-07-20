use godot::classes::{Area2D, CollisionShape2D, IArea2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Block {
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for Block {
    fn init(base: Base<Area2D>) -> Self {
        Block { base }
    }
}

#[godot_api]
impl Block {
    #[func]
    pub fn get_size(&self) -> Vector2 {
        self.base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D")
            .get_shape()
            .unwrap()
            .get_rect()
            .size
    }
}
