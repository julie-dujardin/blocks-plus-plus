use crate::block::Block;
use crate::piece::Piece;
use crate::piece::Shape;
use godot::prelude::*;
use rand::prelude::IndexedRandom;
use rand::Rng;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct TetrisBoard {
    #[export]
    score: i64,
    active_piece: Option<Gd<Piece>>,
    next_pieces: Vec<Gd<Piece>>,
    lines: [[Option<Block>; 10]; 20],

    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for TetrisBoard {
    fn init(base: Base<Node2D>) -> Self {
        TetrisBoard {
            score: 0,
            active_piece: None,
            next_pieces: vec![],
            lines: core::array::from_fn(|i| core::array::from_fn(|i| None)),
            base,
        }
    }

    fn ready(&mut self) {
        if self.active_piece.is_none() {
            let piece_scene: Gd<PackedScene> = load("res://scenes/piece.tscn");
            let mut piece = piece_scene.instantiate_as::<Piece>();

            {
                let mut piece_bind = piece.bind_mut();
                let mut rng = rand::thread_rng();
                piece_bind.set_shape(
                    [
                        Shape::I,
                        Shape::O,
                        Shape::J,
                        Shape::L,
                        Shape::S,
                        Shape::Z,
                        Shape::T,
                    ]
                    .choose(&mut rng)
                    .unwrap()
                    .to_godot(),
                );
            }

            self.base_mut().add_child(piece.clone().upcast());
            self.active_piece = Some(piece);
        }
    }
}

#[godot_api]
impl TetrisBoard {}
