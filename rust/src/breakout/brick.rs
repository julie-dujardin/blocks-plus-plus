use godot::builtin::Vector2;
use godot::classes::{
    CollisionShape2D, ColorRect, GpuParticles2D, IStaticBody2D, LightOccluder2D, StaticBody2D,
    Timer,
};
use godot::obj::{Base, WithBaseField};
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=StaticBody2D)]
pub struct Brick {
    pub is_exploding: bool,

    base: Base<StaticBody2D>,
}

#[godot_api]
impl Brick {
    #[func]
    fn explosion_finished(&mut self) {
        self.base_mut().queue_free();
    }

    pub fn get_size(&self) -> Vector2 {
        self.base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D")
            .get_shape()
            .unwrap()
            .get_rect()
            .size
    }

    pub fn explode(&mut self) {
        self.is_exploding = true;
        self.base()
            .get_node_as::<GpuParticles2D>("ExplosionParticles")
            .set_emitting(true);
        self.base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D")
            .free();
        self.base()
            .get_node_as::<LightOccluder2D>("LightOccluder2D")
            .free();
        self.base().get_node_as::<ColorRect>("ColorRect").free();
        self.base().get_node_as::<Timer>("TimerFree").start();
    }
}

#[godot_api]
impl IStaticBody2D for Brick {
    fn init(base: Base<StaticBody2D>) -> Self {
        Brick {
            is_exploding: false,
            base,
        }
    }
}
