pub mod blocks;
pub mod effects;

use macroquad::prelude::*;

// Shared Structs
#[derive(Clone, Copy)]
pub struct Connectivity {
    pub top: Option<Color>,
    pub right: Option<Color>,
    pub bottom: Option<Color>,
    pub left: Option<Color>,
}
