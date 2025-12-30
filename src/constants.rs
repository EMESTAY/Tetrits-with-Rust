use macroquad::prelude::Color;

pub const BLOCK_SIZE: f32 = 35.0;
pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 20;

/// Helper to create color from hex
pub const fn hex_color(r: u8, g: u8, b: u8) -> Color {
    Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

// Vivid "Jelly" Palette
// Sky: 87CEEB (Light Sky Blue) -> Gradient in draw
pub const COLOR_SKY_TOP: Color = hex_color(0x4F, 0xAC, 0xFE);
pub const COLOR_SKY_BOTTOM: Color = hex_color(0x00, 0xF2, 0xFE);

// UI
pub const COLOR_UI_BG: Color = hex_color(0x00, 0x24, 0x47); // Deep Blue for UI panels

pub const COLOR_TEXT_SHADOW: Color = Color::new(0.0, 0.0, 0.0, 0.5);

pub const COLOR_TEXT: Color = Color::new(1.0, 1.0, 1.0, 1.0);

// Blocks (Vivid, Juicy colors)
pub const COLOR_GREEN: Color = hex_color(0x71, 0xF5, 0x5F); // Lime Green
pub const COLOR_PURPLE: Color = hex_color(0x9D, 0x51, 0xF3); // Rich Purple
pub const COLOR_ORANGE: Color = hex_color(0xFF, 0x9F, 0x1C); // Bright Orange
pub const COLOR_RED: Color = hex_color(0xFF, 0x4E, 0x50); // Coral Red
pub const COLOR_BLUE: Color = hex_color(0x2E, 0x86, 0xAB); // Strong Blue
pub const COLOR_YELLOW: Color = hex_color(0xFF, 0xD4, 0x00); // Golden Yellow
pub const COLOR_CYAN: Color = hex_color(0x00, 0xE0, 0xFF); // Cyan

// Mapping colors to pieces
pub const COLOR_S: Color = COLOR_GREEN;
pub const COLOR_Z: Color = COLOR_RED;
pub const COLOR_L: Color = COLOR_ORANGE;
pub const COLOR_J: Color = COLOR_BLUE;
pub const COLOR_I: Color = COLOR_CYAN;
pub const COLOR_T: Color = COLOR_PURPLE;
pub const COLOR_O: Color = COLOR_YELLOW;

pub const BLOCK_ROUNDING: f32 = 8.0; // Roundness of blocks
pub const UI_ROUNDING: f32 = 15.0; // Roundness of UI panels
