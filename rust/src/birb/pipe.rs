use godot::classes::{AnimatableBody2D, IAnimatableBody2D};
use godot::prelude::*;

const FORWARDS_SPEED: f32 = 30.;

#[derive(GodotClass)]
#[class(base=AnimatableBody2D)]
pub struct Pipe {
    moving: bool,
    base: Base<AnimatableBody2D>,
}

#[godot_api]
impl Pipe {
    #[func]
    fn set_movement(&mut self, can_move: Variant) {
        self.moving = can_move.to::<bool>()
    }
}

#[godot_api]
impl IAnimatableBody2D for Pipe {
    fn init(base: Base<AnimatableBody2D>) -> Self {
        Pipe {
            moving: false,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if self.moving {}
    }
}
