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

use godot::prelude::*;

struct TetrisPlusPlus;

#[gdextension]
unsafe impl ExtensionLibrary for TetrisPlusPlus {
    fn on_level_init(_level: InitLevel) {
        println!("[Rust]      Init level {:?}", _level);
    }

    fn on_level_deinit(_level: InitLevel) {
        println!("[Rust]      Deinit level {:?}", _level);
    }
}
