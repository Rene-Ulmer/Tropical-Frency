// We aren't using the standard library to save some precious memory ;)
#![no_std]
#![feature(
    alloc,
    alloc_error_handler,
    core_intrinsics,
    panic_implementation,
    lang_items
)]
#![allow(dead_code)]

extern crate alloc;
extern crate wee_alloc;

// Use other modules.
mod camera;
mod collision;
mod color;
mod common;
mod effects;
mod enemy;
mod entity;
mod filter;
mod game;
mod gfx;
mod handler;
mod input;
mod level;
mod map;
mod math;
mod misc;
mod movement;
mod particle;
mod pathfinding;
mod player;
mod powerup;
mod prelude;
mod projectile;
mod random;
mod refcell;
mod resources;
mod sound;
mod structures;
mod trigger;
mod weapon;

use common::ScreenPosition;
use game::Game;

// Re-export core methods.
pub use prelude::{alloc, dealloc, oom, panic};

// Re-export handlers.
pub use handler::{kd, ku, md, mm, mu};

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Canvas size.
const WIDTH: usize = 360;
const HEIGHT: usize = 400;

/*******************************************************************/
// Boilerplate code done, actual code following
/*******************************************************************/
// Lots of includes that would be in std::prelude. Oh man, no_std :/
use alloc::slice;
use gfx::Canvas;

/// Contains the current state of the input peripherals.
#[derive(Clone)]
pub struct InputState {
    key_down: [bool; 256],
    mouse_down: [bool; 3],
    mouse_pos: (ScreenPosition),
}

static mut INPUT_STATE: InputState = InputState {
    key_down: [false; 256],
    mouse_down: [false; 3],
    mouse_pos: ScreenPosition(0, 0),
};

static mut LAST_INPUT_STATE: InputState = InputState {
    key_down: [false; 256],
    mouse_down: [false; 3],
    mouse_pos: ScreenPosition(0, 0),
};

static mut GAME: Option<Game> = None;

/*
extern "C" {
    fn R(n_colors: u32);
}
*/

#[no_mangle]
pub extern "C" fn T(buffer: *mut u8, width: usize, height: usize, time: f32) {
    if unsafe { GAME.is_none() } {
        unsafe {
            GAME = Some(Game::new());
        }
    }
    // time is in milliseconds.
    let mut canvas = Canvas::new(
        unsafe { slice::from_raw_parts_mut(buffer, width * height * 4) },
        (width, height),
    );

    canvas.clear(color::DARK_GRAY);

    unsafe {
        if let Some(ref mut game) = GAME {
            game.update(&INPUT_STATE, time);
            game.draw(&mut canvas);
        }
    }

    //unsafe { R(canvas.used_colors() as u32) };
    unsafe {
        LAST_INPUT_STATE = INPUT_STATE.clone();
    }
}
