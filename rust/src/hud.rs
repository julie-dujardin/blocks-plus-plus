use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Hud {
    base: Base<Node2D>,
}

#[godot_api]
impl Hud {
    #[signal]
    fn start_game();

    #[func]
    fn on_start_button_pressed(&mut self) {
        self.base_mut().emit_signal("start_game".into(), &[]);
        self.base_mut().hide();
    }

    #[func]
    fn on_quit_button_pressed(&mut self) {
        // TODO quit game
    }

    #[func]
    fn on_game_over_timer_timeout(&mut self) {
        // TODO
    }
}

#[godot_api]
impl INode2D for Hud {
    fn init(base: Base<Node2D>) -> Self {
        Hud { base }
    }
}
