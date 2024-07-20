use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct PieceSelect {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for PieceSelect {
    fn init(base: Base<Node2D>) -> Self {
        PieceSelect { base }
    }
}
