use godot::classes::{Button, InputEvent, OptionButton, Os};
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
        let difficulty = self
            .base()
            .get_node_as::<OptionButton>("ButtonDifficulty")
            .get_selected_id();
        self.base_mut()
            .emit_signal("start_game".into(), &[difficulty.to_variant()]);
        self.base_mut().hide();
    }

    #[func]
    fn on_quit_button_pressed(&mut self) {
        self.base().get_tree().unwrap().quit();
    }

    #[func]
    fn reset(&mut self) {
        self.base()
            .get_node_as::<Button>("ButtonPlay")
            .set_text("Replay".into());
        self.base_mut().show();
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

    fn input(&mut self, event: Gd<InputEvent>) {
        if !self.base().get_node_as::<Button>("ButtonPlay").has_focus()
            && !self.base().get_node_as::<Button>("ButtonQuit").has_focus()
            && !self
                .base()
                .get_node_as::<Button>("ButtonDifficulty")
                .has_focus()
        {
            for action in ["up", "left", "down", "right"] {
                if event.is_action_pressed(action.into()) {
                    self.base().get_node_as::<Button>("ButtonPlay").grab_focus();
                    return;
                }
            }
        }
    }
}
