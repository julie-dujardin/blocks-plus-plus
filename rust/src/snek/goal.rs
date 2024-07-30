use godot::classes::{Area2D, IArea2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Goal {
    base: Base<Area2D>,
}

#[godot_api]
impl Goal {
    #[signal]
    fn destroyed();

    #[func]
    fn on_area_entered(&mut self, area: Gd<Area2D>) {
        if area.is_class("Goal".into()) {
            self.base_mut().queue_free();
        }
    }
}

#[godot_api]
impl IArea2D for Goal {
    fn init(base: Base<Area2D>) -> Self {
        Goal { base }
    }
}
