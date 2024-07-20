use godot::prelude::*;
use crate::block::Block;
use crate::piece::Piece;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct TetrisBoard {
    #[export]
    score: i64,
    lines: [[Option<Block>; 10]; 20],

    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for TetrisBoard {
    fn init(base: Base<Node2D>) -> Self {
        TetrisBoard {
            score: 0,
            lines: core::array::from_fn(|i| core::array::from_fn(|i| None)),
            base
        }
    }
}

#[godot_api]
impl TetrisBoard {

}