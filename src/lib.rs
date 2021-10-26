#![allow(dead_code)]

pub mod enemy;
pub mod game;
pub mod map;
pub mod player;
pub mod qtree;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub const PI: f64 = std::f64::consts::PI;
pub const RAYS: usize = 360;
