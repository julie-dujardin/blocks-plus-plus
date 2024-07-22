use godot::builtin::Vector2;
use godot::classes::{CharacterBody2D, ICharacterBody2D, Input};
use godot::obj::{Base, WithBaseField};
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct BreakoutPlayer {
    #[export]
    speed: f32,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl BreakoutPlayer {}

#[godot_api]
impl ICharacterBody2D for BreakoutPlayer {
    fn init(base: Base<CharacterBody2D>) -> Self {
        BreakoutPlayer { speed: 350., base }
    }

    fn physics_process(&mut self, _delta: f64) {
        let input = Input::singleton();

        let mut velocity = Vector2::ZERO;

        if input.is_action_pressed("left".into()) {
            velocity.x = -self.speed;
        } else if input.is_action_pressed("right".into()) {
            velocity.x = self.speed;
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }
}
