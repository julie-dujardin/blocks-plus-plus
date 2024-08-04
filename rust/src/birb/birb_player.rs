use godot::classes::{CharacterBody2D, ICharacterBody2D, InputEvent};
use godot::prelude::*;

const GRAVITY: f32 = 60.;
const JUMP_VELOCITY: Vector2 = Vector2::new(30., -30.);

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct BirbPlayer {
    velocity: Vector2,
    pub can_move: bool,
    jumped: bool,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl BirbPlayer {}

#[godot_api]
impl ICharacterBody2D for BirbPlayer {
    fn init(base: Base<CharacterBody2D>) -> Self {
        BirbPlayer {
            velocity: Vector2::ZERO,
            can_move: false,
            jumped: false,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if self.can_move {
            if self.velocity != Vector2::ZERO {
                self.velocity.y += GRAVITY * delta as f32;
            }

            if self.jumped {
                self.jumped = false;
                self.velocity = JUMP_VELOCITY;
            }

            let new_velocity = self.velocity;
            self.base_mut().set_velocity(new_velocity);
            self.base_mut().set_rotation(new_velocity.angle());
            self.base_mut().move_and_slide();
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.can_move && event.is_action_pressed("up".into()) {
            self.jumped = true;
        }
    }
}
