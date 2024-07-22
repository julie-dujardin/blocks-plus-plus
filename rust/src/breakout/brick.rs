use godot::classes::{IStaticBody2D, StaticBody2D};
use godot::obj::Base;
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=StaticBody2D)]
pub struct Brick {
    base: Base<StaticBody2D>,
}

#[godot_api]
impl Brick {}

#[godot_api]
impl IStaticBody2D for Brick {
    fn init(base: Base<StaticBody2D>) -> Self {
        Brick { base }
    }
}
