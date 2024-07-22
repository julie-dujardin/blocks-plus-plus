use crate::tetris::select::Select;
use godot::classes::{Label, Timer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct MainBoard {
    base: Base<Node2D>,
}

#[godot_api]
impl MainBoard {
    #[func]
    fn start_game(&mut self) {
        self.base_mut().get_node_as::<Select>("Select0").show();
        self.base_mut().get_node_as::<Select>("Select1").show();
        self.base_mut().get_node_as::<Select>("Select2").show();
        self.base_mut().get_node_as::<Select>("Select3").show();
        self.base_mut().get_node_as::<Label>("LabelGameOver").hide();
    }

    #[func]
    fn on_game_over(&mut self) {
        self.base_mut().get_node_as::<Label>("LabelGameOver").show();
        self.base_mut()
            .get_node_as::<Timer>("TimerGameOver")
            .start();
    }

    #[func]
    fn on_game_over_timer_timeout(&mut self) {
        self.base_mut().get_node_as::<Select>("Select0").hide();
        self.base_mut().get_node_as::<Select>("Select1").hide();
        self.base_mut().get_node_as::<Select>("Select2").hide();
        self.base_mut().get_node_as::<Select>("Select3").hide();
        self.base_mut().get_node_as::<Select>("Tetris").hide();
    }
}

#[godot_api]
impl INode2D for MainBoard {
    fn init(base: Base<Node2D>) -> Self {
        MainBoard { base }
    }
}
