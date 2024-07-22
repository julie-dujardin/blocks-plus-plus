use godot::prelude::*;
use crate::select::Select;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct MainBoard {
    base: Base<Node2D>,
}

#[godot_api]
impl MainBoard {
    #[func]
    fn start_game(&mut self) {
        self.base_mut()
            .get_node_as::<Select>("Select0").show();
        self.base_mut()
            .get_node_as::<Select>("Select1").show();
        self.base_mut()
            .get_node_as::<Select>("Select2").show();
        self.base_mut()
            .get_node_as::<Select>("Select3").show();
    }
}

#[godot_api]
impl INode2D for MainBoard {
    fn init(base: Base<Node2D>) -> Self {
        MainBoard { base }
    }
}
