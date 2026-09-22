// SPDX-FileCopyrightText: 2026 Alex Brandt
//
// SPDX-License-Identifier: MIT

//! Zellij plugin for pairing with Claude Code.

use zellij_tile::prelude::*;

#[derive(Default)]
struct State;

register_plugin!(State);

impl ZellijPlugin for State {
    fn render(&mut self, _rows: usize, _cols: usize) {
        println!("Hello, world!");
    }
}
