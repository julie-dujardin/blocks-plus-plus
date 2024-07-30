use godot::classes::{Area2D, IArea2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Segment {
    base: Base<Area2D>,
}

#[godot_api]
impl Segment {
    #[signal]
    fn score_up();

    #[signal]
    fn game_over();

    #[func]
    fn on_area_entered(&mut self, area: Gd<Area2D>) {
        if area.is_class("Goal".into()) {
            self.base_mut().emit_signal("score_up".into(), &[]);
            area.free();
        } else {
            self.base_mut().emit_signal("game_over".into(), &[]);
        }
    }
}

#[godot_api]
impl IArea2D for Segment {
    fn init(base: Base<Area2D>) -> Self {
        Segment { base }
    }
}
