use godot::classes::{IStaticBody2D, StaticBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=StaticBody2D)]
pub struct Segment {
    base: Base<StaticBody2D>,
}

#[godot_api]
impl Segment {}

#[godot_api]
impl IStaticBody2D for Segment {
    fn init(base: Base<StaticBody2D>) -> Self {
        Segment { base }
    }
}
