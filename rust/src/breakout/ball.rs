use crate::breakout::brick::Brick;
use crate::constants::{COLOR_FOREGROUND, COLOR_SUCCESS};
use godot::builtin::Vector2;
use godot::classes::{CharacterBody2D, ICharacterBody2D, StaticBody2D};
use godot::obj::{Base, WithBaseField};
use godot::prelude::{godot_api, GodotClass, ToGodot};
use rand::Rng;

const BONUS_CHANCE: f64 = 0.05;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Ball {
    #[export]
    start_speed: f32,
    current_velocity: Vector2,
    pub bonus_active: bool,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl Ball {
    #[signal]
    fn broke_brick();

    #[signal]
    fn game_over();

    fn handle_game_over(&mut self) {
        self.base_mut().emit_signal("game_over".into(), &[]);
        self.set_movement(false);
    }

    pub fn reset(&mut self) {
        self.set_movement(false);
        self.base_mut().set_position(Vector2::new(208., 232.))
    }

    pub fn set_movement(&mut self, can_move: bool) {
        if can_move {
            self.current_velocity = Vector2::new(self.start_speed / 2., -self.start_speed / 2.);
        } else {
            self.current_velocity = Vector2::ZERO;
            self.base_mut().set_velocity(Vector2::ZERO);
        }
    }

    fn set_bonus(&mut self, active: bool) {
        self.bonus_active = active;
        self.base_mut().set_modulate(if active {
            COLOR_SUCCESS
        } else {
            COLOR_FOREGROUND
        });
    }
}

#[godot_api]
impl ICharacterBody2D for Ball {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Ball {
            start_speed: 200.,
            current_velocity: Vector2::ZERO,
            bonus_active: false,
            base,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        if self.current_velocity != Vector2::ZERO {
            let current_velocity = self.current_velocity;
            self.base_mut().set_velocity(current_velocity);
            self.base_mut().move_and_slide();

            if self.base_mut().is_on_ceiling() || self.base_mut().is_on_floor() {
                self.current_velocity.y = -current_velocity.y;
            }
            if self.base_mut().is_on_wall() {
                self.current_velocity.x = -current_velocity.x;
            }

            let mut rng = rand::thread_rng();
            for slide in 0..self.base().get_slide_collision_count() {
                let collision_opt = self.base_mut().get_slide_collision(slide);
                if let Some(collision) = collision_opt {
                    let collider_opt = collision.get_collider();
                    if let Some(collider) = collider_opt {
                        if let Ok(brick) = collider.clone().try_cast::<Brick>() {
                            let bonus_active = self.bonus_active;
                            self.base_mut().emit_signal(
                                "broke_brick".into(),
                                &[brick.to_variant(), bonus_active.to_variant()],
                            );
                            self.set_bonus(false);
                        } else if let Ok(area) = collider.try_cast::<StaticBody2D>() {
                            if area.get_name() == "Bottom".into() {
                                self.handle_game_over();
                            }
                        }
                    }
                }

                if !self.bonus_active && rng.gen::<f64>() < BONUS_CHANCE {
                    self.set_bonus(true);
                }
            }
        }
    }
}
