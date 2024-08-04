use crate::birb::birb_player::BirbPlayer;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct BirbBoard {
    base: Base<Node2D>,
}

#[godot_api]
impl BirbBoard {
    #[func]
    fn on_activated(&mut self) {
        self.base_mut().show();
        self.set_movement(true);
        self.base()
            .get_node_as::<BirbPlayer>("Birb")
            .set_position(Vector2::new(376., 120.));
    }

    #[func]
    fn on_parent_game_over(&mut self) {
        self.set_movement(false);
    }

    #[func]
    fn reset(&mut self) {
        self.base_mut().hide();
    }

    fn set_movement(&self, can_move: bool) {
        let mut player = self.base().get_node_as::<BirbPlayer>("Birb");
        let mut player_bind = player.bind_mut();
        player_bind.can_move = can_move;
    }
}

#[godot_api]
impl INode2D for BirbBoard {
    fn init(base: Base<Node2D>) -> Self {
        BirbBoard { base }
    }

    fn ready(&mut self) {
        if self.base().get_parent().unwrap().is_class("Window".into()) {
            // If this class is the root node, make it playable for testing
            self.on_activated();
        }
    }
}
