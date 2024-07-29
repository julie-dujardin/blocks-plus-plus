use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct SnekBoard {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for SnekBoard {
    fn init(base: Base<Node2D>) -> Self {
        SnekBoard { base }
    }
}
