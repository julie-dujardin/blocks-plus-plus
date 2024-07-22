use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::obj::Base;
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Ball {
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl Ball {}

#[godot_api]
impl ICharacterBody2D for Ball {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Ball { base }
    }
}
