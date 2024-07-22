use godot::classes::{Area2D, CollisionShape2D, IArea2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Block {
    base: Base<Area2D>,
    pub board_offset: Vector2,
}

#[godot_api]
impl IArea2D for Block {
    fn init(base: Base<Area2D>) -> Self {
        Block {
            board_offset: Vector2::ZERO,
            base,
        }
    }

    fn ready(&mut self) {
        self.update_position();
    }
}

#[godot_api]
impl Block {
    pub fn update_position(&mut self) {
        let position = self.board_offset * self.get_size();
        self.base_mut().set_position(position);
    }

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
