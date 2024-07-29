use crate::tetris::block::Block;
use godot::prelude::*;
use phf::phf_map;
use rand::prelude::IndexedRandom;
use std::f32::consts::PI;

#[derive(Clone, GodotConvert, Var, Export)]
#[godot(via=GString)]
pub enum Shape {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

static SHAPES: [Shape; 7] = [
    Shape::I,
    Shape::O,
    Shape::J,
    Shape::L,
    Shape::S,
    Shape::Z,
    Shape::T,
];

const SHAPE_BLOCKS: phf::Map<&str, [Vector2; 4]> = phf_map! {
    "I" => [
        Vector2::new(-1.0, 0.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(2.0, 0.0)
    ],
    "O" => [
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0)
    ],
    "T" => [
        Vector2::new(-1.0, 0.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(0.0, -1.0)
    ],
    "J" => [
        Vector2::new(0.0, -1.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(-1.0, 1.0)
    ],
    "L" => [
        Vector2::new(0.0, -1.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0)
    ],
    "S" => [
        Vector2::new(1.0, 0.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(-1.0, 1.0)
    ],
    "Z" => [
        Vector2::new(-1.0, 0.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0)
    ],
};
const SHAPE_BOUNDS: phf::Map<&str, [Vector2; 2]> = phf_map! {
    "I" => [
        Vector2::new(3.0, 0.0),
        Vector2::new(-1.0, 0.0)
    ],
    "O" => [
        Vector2::new(1.0, 1.0),
        Vector2::new(0.0, 0.0)
    ],
    "T" => [
        Vector2::new(2.0, 1.0),
        Vector2::new(-1.0, -1.0)
    ],
    "J" => [
        Vector2::new(1.0, 2.0),
        Vector2::new(-1.0, -1.0)
    ],
    "L" => [
        Vector2::new(1.0, 2.0),
        Vector2::new(0.0, -1.0)
    ],
    "S" => [
        Vector2::new(2.0, 1.0),
        Vector2::new(-1.0, 0.0)
    ],
    "Z" => [
        Vector2::new(2.0, 1.0),
        Vector2::new(-1.0, 0.0)
    ],
};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Piece {
    #[export]
    pub shape: Shape,
    pub blocks: VariantArray,
    block_size: Vector2,
    pub center_block_position: Vector2,
    rotation: real,

    base: Base<Node2D>,
}

#[godot_api]
impl Piece {
    pub fn spawn_random() -> Gd<Piece> {
        let piece_scene: Gd<PackedScene> = load("res://scenes/tetris/piece.tscn");
        let mut piece = piece_scene.instantiate_as::<Piece>();

        {
            let mut piece_bind = piece.bind_mut();
            let mut rng = rand::thread_rng();
            piece_bind.set_shape(SHAPES.choose(&mut rng).unwrap().to_godot());
        }

        piece
    }

    fn get_bounds(&self, position: Vector2, rotation: real) -> (Vector2, Vector2) {
        let bounds = SHAPE_BOUNDS
            .get(self.shape.to_godot().to_string().as_str())
            .unwrap();
        let bounding_rectangle = bounds[0];
        let center_offset = bounds[1];

        let bottom_right_rotated =
            (center_offset + bounding_rectangle).rotated(rotation) + position;
        let top_left_rotated = center_offset.rotated(rotation) + position;
        let top_left = Vector2::new(
            bottom_right_rotated.x.min(top_left_rotated.x).round(),
            bottom_right_rotated.y.min(top_left_rotated.y).round(),
        );
        let bottom_right = Vector2::new(
            bottom_right_rotated.x.max(top_left_rotated.x).round(),
            bottom_right_rotated.y.max(top_left_rotated.y).round(),
        );
        (top_left, bottom_right)
    }

    fn change_position(&mut self, position: Vector2) {
        self.center_block_position = position;

        let position = self.block_size * self.center_block_position;
        self.base_mut().set_position(position);
    }

    pub fn mov(&mut self, direction: Vector2) {
        let new_position = self.center_block_position + direction;

        let (top_left, bottom_right) = self.get_bounds(new_position, self.rotation);
        if top_left.x >= 0. && bottom_right.x < 10. {
            self.change_position(new_position)
        }
    }

    pub fn down(&mut self) -> bool {
        let new_position = self.center_block_position + Vector2::DOWN;

        let (_, bottom_right) = self.get_bounds(new_position, self.rotation);
        if bottom_right.y < 20. {
            self.change_position(new_position);
            return true;
        }
        false
    }

    pub fn rotate(&mut self, clockwise: bool) {
        let additional_rotation = match self.shape {
            Shape::O => 0., // o should not be rotated
            // Those pieces only have 2 different rotations
            Shape::S => {
                if self.rotation == 0. {
                    PI / 2.
                } else {
                    -PI / 2.
                }
            }
            Shape::Z => {
                if self.rotation == 0. {
                    PI / 2.
                } else {
                    -PI / 2.
                }
            }
            Shape::I => {
                if self.rotation == 0. {
                    PI / 2.
                } else {
                    -PI / 2.
                }
            }
            // Other pieces can be rotated 4 ways
            _ => PI / 2.,
        };
        let new_rotation =
            (self.rotation + additional_rotation) % (PI * 2.) * if clockwise { 1. } else { -1. };

        let (top_left, bottom_right) = self.get_bounds(self.center_block_position, new_rotation);
        if top_left.x >= 0. && bottom_right.x < 10. && bottom_right.y < 20. {
            self.rotation = new_rotation;
            for block in self.blocks.iter_shared() {
                let mut block_ref = block.to::<Gd<Block>>();
                let mut block = block_ref.bind_mut();
                let new_cell = block.board_offset.rotated(additional_rotation);
                block.board_offset = new_cell;
                block.update_position();
            }
        }
    }

    pub fn position_for_ui(&mut self) {
        let base_offset = self.base().get_position() + Vector2::new(42., 7.);
        match self.shape {
            Shape::I => {
                self.base_mut()
                    .set_position(base_offset + Vector2::new(0., 10.));
            }
            Shape::T => {
                self.base_mut()
                    .set_position(base_offset + Vector2::new(10., 20.));
            }
            Shape::J => {
                self.base_mut().set_rotation(PI / 2.);
                self.base_mut()
                    .set_position(base_offset + Vector2::new(30., 20.));
            }
            Shape::L => {
                self.base_mut().set_rotation(PI / 2.);
                self.base_mut()
                    .set_position(base_offset + Vector2::new(30., 0.));
            }
            Shape::S => {
                self.base_mut()
                    .set_position(base_offset + Vector2::new(10., 0.));
            }
            Shape::Z => {
                self.base_mut()
                    .set_position(base_offset + Vector2::new(10., 0.));
            }
            _ => {
                self.base_mut().set_position(base_offset);
            }
        }
    }

    pub fn update_position(&mut self) {
        let piece_position = self.center_block_position * self.block_size;
        self.base_mut().set_rotation(0.);
        self.base_mut().set_position(piece_position);
    }
}

#[godot_api]
impl INode2D for Piece {
    fn init(base: Base<Node2D>) -> Self {
        Piece {
            blocks: varray![],
            shape: Shape::O, // override this after instantiation but before add_child
            center_block_position: Vector2::new(0., 0.),
            rotation: 0.,
            block_size: Vector2::ZERO, // Gets set in self.ready()
            base,
        }
    }

    fn ready(&mut self) {
        let block_scene: Gd<PackedScene> = load("res://scenes/tetris/block.tscn");

        for block_offset in SHAPE_BLOCKS
            .get(self.shape.to_godot().to_string().as_str())
            .unwrap()
        {
            let mut block = block_scene.instantiate_as::<Block>();

            {
                let mut block_bind = block.bind_mut();
                self.block_size = block_bind.get_size();
                block_bind.board_offset = *block_offset;
            }

            self.blocks.push(block.to_variant());
            self.base_mut().add_child(block.clone().upcast());
        }
        self.update_position();
    }
}
