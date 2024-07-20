use godot::classes::InputEvent;
use godot::prelude::*;
use std::f32::consts::PI;

use crate::block::Block;

#[derive(GodotConvert, Var, Export)]
#[godot(via=GString)]
enum Shape {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Piece {
    #[export]
    shape: Shape,
    blocks: VariantArray,
    block_size: Vector2,
    center_block_position: Vector2,
    rotation: real,

    shape_blocks: Dictionary,
    shape_bounds: Dictionary,

    base: Base<Node2D>,
}

#[godot_api]
impl Piece {
    fn get_bounds(&self, position: Vector2) -> (Vector2, Vector2) {
        let bounds = self
            .shape_bounds
            .get(self.shape.to_godot())
            .unwrap()
            .to::<VariantArray>();
        let bounding_rectangle = bounds.at(0).to::<Vector2>();
        let center_offset = bounds.at(1).to::<Vector2>();

        let bottom_right_rotated =
            (center_offset + bounding_rectangle).rotated(self.rotation) + position;
        let top_left_rotated = center_offset.rotated(self.rotation) + position;
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

    fn mov(&mut self, direction: Vector2) {
        let new_position = self.center_block_position + direction;

        let (top_left, bottom_right) = self.get_bounds(new_position);
        if top_left.x >= 0. && bottom_right.x < 10. {
            self.change_position(new_position)
        }
    }

    fn down(&mut self) -> bool {
        let new_position = self.center_block_position + Vector2::DOWN;

        let (_, bottom_right) = self.get_bounds(new_position);
        if bottom_right.y < 20. {
            self.change_position(new_position);
            return true
        }
        false
    }

    fn drop(&mut self) {
        while self.down() {}
    }

    fn rotate(&mut self, clockwise: bool) {
        let additional_rotation = match self.shape {
            Shape::O => 0.,  // o should not be rotated
            // Those pieces only have 2 different rotations
            Shape::S => if self.rotation == 0. {PI / 2.} else {-PI / 2.},
            Shape::Z => if self.rotation == 0. {PI / 2.} else {-PI / 2.},
            Shape::I => if self.rotation == 0. {PI / 2.} else {-PI / 2.},
            // Other pieces can be rotated 4 ways
            _ => PI / 2.,
        };
        self.rotation = (self.rotation + additional_rotation) % (PI * 2.) * if clockwise {1.} else {-1.};
        for block in self.blocks.iter_shared() {
            let mut block_ref = block.to::<Gd<Block>>();
            let mut block = block_ref.bind_mut();
            let new_cell = block.board_offset.rotated(additional_rotation);
            block.board_offset = new_cell;
            block.update_position();
        }
    }
}

#[godot_api]
impl INode2D for Piece {
    fn init(base: Base<Node2D>) -> Self {
        Piece {
            blocks: varray![],
            shape: Shape::O, // TODO override this after instantiation but before add_child
            center_block_position: Vector2::new(5., 2.),
            rotation: 0.,
            block_size: Vector2::ZERO, // Gets set in self.ready()
            shape_blocks: dict![
                // Shapes rotate around their (0, 0) block
                "I": varray![Vector2::new(-1., 0.), Vector2::new(0., 0.), Vector2::new(1., 0.), Vector2::new(2., 0.)],
                "O": varray![Vector2::new(0., 0.), Vector2::new(1., 0.), Vector2::new(0., 1.), Vector2::new(1., 1.)],
                "T": varray![Vector2::new(-1., 0.), Vector2::new(0., 0.), Vector2::new(1., 0.), Vector2::new(0., -1.)],
                "J": varray![Vector2::new(0., -1.), Vector2::new(0., 0.), Vector2::new(0., 1.), Vector2::new(-1., 1.)],
                "L": varray![Vector2::new(0., -1.), Vector2::new(0., 0.), Vector2::new(0., 1.), Vector2::new(1., 1.)],
                "S": varray![Vector2::new(1., 0.), Vector2::new(0., 0.), Vector2::new(0., 1.), Vector2::new(-1., 1.)],
                "Z": varray![Vector2::new(-1., 0.), Vector2::new(0., 0.), Vector2::new(0., 1.), Vector2::new(1., 1.)],
            ],
            shape_bounds: dict![
                // for each shape, its (width, height), (x/y offset of upper left bounding rectangle relative to center)
                "I": varray![Vector2::new(3., 0.), Vector2::new(-1., 0.)],
                "O": varray![Vector2::new(1., 1.), Vector2::new(0., 0.)],
                "T": varray![Vector2::new(2., 1.), Vector2::new(-1., -1.)],  // The (-1., -1.) point is not a block, but it is part of the bounding rectangle
                "J": varray![Vector2::new(1., 2.), Vector2::new(-1., -1.)],
                "L": varray![Vector2::new(1., 2.), Vector2::new(0., -1.)],
                "S": varray![Vector2::new(2., 1.), Vector2::new(-1., 0.)],
                "Z": varray![Vector2::new(2., 1.), Vector2::new(-1., 0.)],
            ],
            base,
        }
    }

    fn ready(&mut self) {
        let block_scene: Gd<PackedScene> = load("res://scenes/block.tscn");

        for block_offset in self
            .shape_blocks
            .get(self.shape.to_godot())
            .unwrap()
            .to::<VariantArray>()
            .iter_shared()
        {
            let mut block = block_scene.instantiate_as::<Block>();
            self.block_size = block.call("get_size".into(), &[]).to::<Vector2>();
            let piece_position = self.center_block_position * self.block_size;
            self.base_mut().set_position(piece_position);

            {
                let mut block_ref = block.bind_mut();
                block_ref.board_offset = block_offset.to::<Vector2>();
            }

            self.blocks.push(block.to_variant());
            self.base_mut().add_child(block.clone().upcast());
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("down".into()) {
            self.drop();
        } else if event.is_action_pressed("up".into()) {
            self.rotate(true);
        } else if event.is_action_pressed("left".into()) {
            self.mov(Vector2::LEFT);
        } else if event.is_action_pressed("right".into()) {
            self.mov(Vector2::RIGHT);
        }
    }
}
