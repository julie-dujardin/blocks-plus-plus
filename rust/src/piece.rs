use godot::classes::InputEvent;
use godot::prelude::*;

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

    shape_blocks: Dictionary,
    shape_bounds: Dictionary,

    base: Base<Node2D>,
}

#[godot_api]
impl Piece {
    fn mov(&mut self, direction: Vector2) {}

    fn drop(&mut self) {}

    fn rotate(&mut self) {
        let start = varray![Vector2::new(0., -1.), Vector2::new(0., 0.), Vector2::new(0., 1.), Vector2::new(1., 1.)];
        let rotate = varray![Vector2::new(1., 0.), Vector2::new(0., 0.), Vector2::new(-1., 0.), Vector2::new(-1., 1.)];

        for (i, block_offset) in self
            .shape_blocks
            .get(self.shape.to_godot())
            .unwrap()
            .to::<VariantArray>()
            .iter_shared().enumerate()
        {
            let block = self.blocks.at(i);
        }
    }
}

#[godot_api]
impl INode2D for Piece {
    fn init(base: Base<Node2D>) -> Self {
        Piece {
            blocks: varray![],
            shape: Shape::O, // TODO override this after instantiation but before add_child
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
                "I": varray![Vector2::new(4., 1.), Vector2::new(-1., 0.)],
                "O": varray![Vector2::new(2., 2.), Vector2::new(0., 0.)],
                "T": varray![Vector2::new(3., 2.), Vector2::new(-1., -1.)],  // The (-1., -1.) point is not a block, but it is part of the bounding rectangle
                "J": varray![Vector2::new(2., 3.), Vector2::new(-1., -1.)],
                "L": varray![Vector2::new(2., 3.), Vector2::new(0., -1.)],
                "S": varray![Vector2::new(3., 2.), Vector2::new(-1., 0.)],
                "Z": varray![Vector2::new(3., 2.), Vector2::new(-1., 0.)],
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

            let block_size = block.call("get_size".into(), &[]).to::<Vector2>();
            let block_position =
                block_offset.to::<Vector2>() * block_size + Vector2::new(200., 200.);
            godot_print!("{}", block_position);
            block.set_position(block_position);

            self.blocks.push(block.to_variant());
            self.base_mut().add_child(block.clone().upcast());
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("down".into()) {
            self.drop();
        } else if event.is_action_pressed("up".into()) {
            self.rotate();
        } else if event.is_action_pressed("left".into()) {
            self.mov(Vector2::LEFT);
        } else if event.is_action_pressed("right".into()) {
            self.mov(Vector2::RIGHT);
        }
    }
}
