use godot::classes::tween::TweenProcessMode;
use godot::classes::{AnimatableBody2D, IAnimatableBody2D, Tween};
use godot::prelude::*;

const FORWARDS_SPEED: f32 = -20.;

#[derive(GodotClass)]
#[class(base=AnimatableBody2D)]
pub struct Pipe {
    tween: Option<Gd<Tween>>,
    base: Base<AnimatableBody2D>,
}

#[godot_api]
impl Pipe {
    #[func]
    fn set_movement(&mut self, can_move: Variant) {
        let current_position = self.base().get_position();
        let self_clone = self.base_mut().clone().upcast();

        if self.tween == None {
            let tween_opt = self.base_mut().create_tween();
            if let Some(mut tween) = tween_opt {
                tween.set_process_mode(TweenProcessMode::PHYSICS);
                tween.tween_property(
                    self_clone,
                    "position".into(),
                    Vector2::new(current_position.x - 1300., current_position.y).to_variant(),
                    60.,
                );
                self.tween = Some(tween);
            }
        }

        if let Some(ref mut tween) = &mut self.tween {
            if !can_move.to::<bool>() {
                tween.stop();
            }
        }
    }
}

#[godot_api]
impl IAnimatableBody2D for Pipe {
    fn init(base: Base<AnimatableBody2D>) -> Self {
        Pipe {
            tween: None,
            base,
        }
    }
}
