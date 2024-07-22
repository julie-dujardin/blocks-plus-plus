use crate::breakout::brick::Brick;
use godot::classes::{INode2D, Node2D};
use godot::obj::{Base, Gd};
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct BreakoutBoard {
    #[export]
    score: i64,
    game_over: bool,
    // lines: Vec<[Option<Gd<Brick>>; 10]>,

    base: Base<Node2D>,
}

#[godot_api]
impl BreakoutBoard {}

#[godot_api]
impl INode2D for BreakoutBoard {
    fn init(base: Base<Node2D>) -> Self {
        BreakoutBoard {
            score: 0,
            game_over: false,
            // lines: vec![],
            base,
        }
    }
}
