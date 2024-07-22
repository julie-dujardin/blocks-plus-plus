use godot::builtin::Vector2;
use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::obj::{Base, WithBaseField};
use godot::prelude::{godot_api, GodotClass};
use crate::breakout::brick::Brick;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Ball {
    #[export]
    start_speed: f32,
    current_velocity: Vector2,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl Ball {
    #[signal]
    fn broke_brick();
}

#[godot_api]
impl ICharacterBody2D for Ball {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Ball {
            start_speed: 200.,
            current_velocity: Vector2::ZERO,
            base,
        }
    }

    fn ready(&mut self) {
        self.current_velocity = Vector2::new(self.start_speed / 2., -self.start_speed / 2.);
    }

    fn physics_process(&mut self, _delta: f64) {
        let current_velocity = self.current_velocity;
        self.base_mut().set_velocity(current_velocity);
        self.base_mut().move_and_slide();

        for slide in 0..self.base().get_slide_collision_count() {
            let collision = self.base_mut().get_slide_collision(slide);
            let collider = collision.unwrap().get_collider().unwrap().try_cast::<Brick>();
            if let Ok(brick) = collider {
                {
                    brick.free();
                    if self.base().is_on_wall(){
                        self.current_velocity.x = - current_velocity.x;
                    }
                    else {
                        self.current_velocity.y = - current_velocity.y;
                    }
                    self.base_mut().emit_signal("broke_brick".into(), &[]);
                }
            };
        }

        if self.base_mut().is_on_ceiling() || self.base_mut().is_on_floor() {
            self.current_velocity.y = - current_velocity.y;
        }
        if self.base_mut().is_on_wall() {
            self.current_velocity.x = - current_velocity.x;
        }
        if self.base_mut().is_on_floor() {
            // TODO check if the floor is lava -> game over
        }
    }
}
