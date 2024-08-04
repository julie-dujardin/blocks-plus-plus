use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct BirbBoard {
    base: Base<Node2D>,
}

#[godot_api]
impl BirbBoard {
    #[func]
    fn on_activated(&mut self) {
        self.base_mut().show();
    }
}

#[godot_api]
impl INode2D for BirbBoard {
    fn init(base: Base<Node2D>) -> Self {
        BirbBoard {
            base,
        }
    }
}
