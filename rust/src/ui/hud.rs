use godot::classes::{Button, Os};
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
        self.base().get_tree().unwrap().quit();
    }

    #[func]
    fn on_game_over_timer_timeout(&mut self) {
        self.base()
            .get_node_as::<Button>("ButtonPlay")
            .set_text("Replay".into());
        self.base_mut().show();
        self.base().get_node_as::<Button>("ButtonPlay").grab_focus();
    }
}

#[godot_api]
impl INode2D for Hud {
    fn init(base: Base<Node2D>) -> Self {
        Hud { base }
    }

    fn ready(&mut self) {
        if Os::singleton().get_name() == "Web".into() {
            self.base().get_node_as::<Button>("ButtonQuit").hide();
        }
        self.base().get_node_as::<Button>("ButtonPlay").grab_focus();
    }
}