use crate::birb::birb_player::BirbPlayer;
use crate::birb::pipe::Pipe;
use crate::constants::{COLOR_FAILURE, COLOR_FOREGROUND};
use godot::classes::InputEvent;
use godot::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

const PIPE_SPAWN_START: f32 = 576.;
const PIPE_SPAWN_END: f32 = 1152.;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct BirbBoard {
    can_move: bool,
    base: Base<Node2D>,
}

#[godot_api]
impl BirbBoard {
    #[signal]
    fn game_over();

    #[signal]
    fn set_pipe_movement();

    #[func]
    fn on_activated(&mut self) {
        self.base_mut().show();
        self.set_movement(true);
        self.base()
            .get_node_as::<BirbPlayer>("Birb")
            .set_position(Vector2::new(376., 120.));
        self.can_move = true
    }

    #[func]
    fn on_parent_game_over(&mut self) {
        self.set_movement(false);
        self.can_move = false;
        self.base_mut()
            .emit_signal("set_pipe_movement".into(), &[false.to_variant()]);
    }

    #[func]
    fn on_birb_collided(&mut self) {
        self.base_mut().emit_signal("game_over".into(), &[]);
        self.set_color(COLOR_FAILURE);

        if self.base().get_parent().unwrap().is_class("Window".into()) {
            // If this class is the root node, make it playable for testing
            self.on_parent_game_over();
        }
    }

    #[func]
    fn reset(&mut self) {
        self.base_mut().hide();
        self.set_color(COLOR_FOREGROUND);
    }

    fn set_movement(&self, can_move: bool) {
        let mut player = self.base().get_node_as::<BirbPlayer>("Birb");
        let mut player_bind = player.bind_mut();
        player_bind.can_move = can_move;
    }

    fn spawn_pipes(&mut self, x_offset: f32) {
        let mut rng = rand::thread_rng();
        let top_y = rng.gen_range(10..(208 - (10 + 48))) as f32;
        let pipe_scene: Gd<PackedScene> = load("res://scenes/birb/pipe.tscn");

        let mut up_pipe = pipe_scene.instantiate_as::<Pipe>();
        up_pipe.set_position(Vector2::new(PIPE_SPAWN_START + x_offset, top_y));
        self.base_mut()
            .connect("set_pipe_movement".into(), up_pipe.callable("set_movement"));
        self.base_mut().add_child(up_pipe.upcast());

        let mut down_pipe = pipe_scene.instantiate_as::<Pipe>();
        down_pipe.set_position(Vector2::new(PIPE_SPAWN_START + x_offset, top_y + 48.));
        down_pipe.set_rotation(PI);
        self.base_mut().connect(
            "set_pipe_movement".into(),
            down_pipe.callable("set_movement"),
        );
        self.base_mut().add_child(down_pipe.upcast());
    }

    fn set_color(&mut self, color: Color) {
        self.base_mut().set_modulate(color);
    }
}

#[godot_api]
impl INode2D for BirbBoard {
    fn init(base: Base<Node2D>) -> Self {
        BirbBoard {
            can_move: false,
            base,
        }
    }

    fn ready(&mut self) {
        for offset in 0..7 {
            self.spawn_pipes((offset * 128) as f32);
        }
        if self.base().get_parent().unwrap().is_class("Window".into()) {
            // If this class is the root node, make it playable for testing
            self.on_activated();
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.can_move && event.is_action_pressed("up".into()) {
            self.base_mut()
                .emit_signal("set_pipe_movement".into(), &[true.to_variant()]);
        }
    }
}
