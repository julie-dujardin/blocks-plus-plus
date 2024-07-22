use crate::breakout::brick::Brick;
use godot::builtin::Vector2;
use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::engine::StaticBody2D;
use godot::obj::{Base, WithBaseField};
use godot::prelude::{godot_api, GodotClass};

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

    #[signal]
    fn game_over();

    fn handle_game_over(&mut self) {
        self.base_mut().emit_signal("game_over".into(), &[]);
        self.current_velocity = Vector2::ZERO;
        self.base_mut().set_velocity(Vector2::ZERO);
        self.base_mut().set_position(Vector2::new(208., 232.))
    }

    pub fn set_movement(&mut self, can_move: bool) {
        if can_move {
            self.current_velocity = Vector2::new(self.start_speed / 2., -self.start_speed / 2.);
        }
        else {
            self.current_velocity = Vector2::ZERO;
        }
    }
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

    fn physics_process(&mut self, _delta: f64) {
        let current_velocity = self.current_velocity;
        self.base_mut().set_velocity(current_velocity);
        self.base_mut().move_and_slide();

        if self.base_mut().is_on_ceiling() || self.base_mut().is_on_floor() {
            self.current_velocity.y = -current_velocity.y;
        }
        if self.base_mut().is_on_wall() {
            self.current_velocity.x = -current_velocity.x;
        }

        for slide in 0..self.base().get_slide_collision_count() {
            let collision = self.base_mut().get_slide_collision(slide);
            let collider = collision.unwrap().get_collider().unwrap();

            if let Ok(brick) = collider.clone().try_cast::<Brick>() {
                brick.free();
                self.base_mut().emit_signal("broke_brick".into(), &[]);
            } else if let Ok(area) = collider.try_cast::<StaticBody2D>() {
                if area.get_name() == "Bottom".into() {
                    self.handle_game_over();
                }
            }
        }
    }
}
