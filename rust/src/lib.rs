/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
mod birb;
mod breakout;
mod constants;
mod main_board;
mod snek;
mod tetris;
mod ui;

use crate::ui::state::GameState;
use godot::classes::Engine;
use godot::prelude::*;

struct TetrisPlusPlus;

#[gdextension]
unsafe impl ExtensionLibrary for TetrisPlusPlus {
    fn on_level_init(_level: InitLevel) {
        println!("[Rust]      Init level {:?}", _level);

        Engine::singleton().register_singleton(
            StringName::from("GameState"),
            GameState::new_alloc().upcast(),
        );
    }

    fn on_level_deinit(_level: InitLevel) {
        println!("[Rust]      Deinit level {:?}", _level);

        let mut engine = Engine::singleton();
        let singleton_name = StringName::from("GameState");
        let singleton = engine
            .get_singleton(singleton_name.clone())
            .expect("cannot retrieve the singleton");
        engine.unregister_singleton(singleton_name);
        if singleton.is_instance_valid() {
            singleton.free();
        }
    }
}
