use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct TetrisBoard {
    #[export]
    score: i64,

    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for TetrisBoard {
    fn init(base: Base<Node2D>) -> Self {
        TetrisBoard { score: 0, base }
    }
}
