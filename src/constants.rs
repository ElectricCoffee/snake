#![allow(unused)]
pub const SPRITE_WIDTH: f32 = 8.;
pub const SPRITE_HEIGHT: f32 = 8.;

pub const ARENA_WIDTH: f32 = SPRITE_WIDTH * 32.;
pub const ARENA_HEIGHT: f32 = SPRITE_HEIGHT * 32.;

pub const ARENA_CENTER_X: f32 = ARENA_WIDTH / 2.;
pub const ARENA_CENTER_Y: f32 = ARENA_HEIGHT / 2.;

pub const BLACK: [f32; 4] = [0., 0., 0., 1.];

pub mod sprites {
    pub const HEAD_IDX: usize = 0;
    pub const BODY_IDX: usize = 1;
    pub const TAIL_IDX: usize = 2;
    pub const POP_IDX: usize = 3;
    pub const CHERRY_IDX: usize = 6; // Why I chose this position I don't know...
    pub const CLOCK: [usize; 8] = [8, 9, 10, 11, 12, 13, 14, 15];
}
