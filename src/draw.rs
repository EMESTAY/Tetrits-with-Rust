use crate::bidule::Bidule;
use crate::constants::*;

use macroquad::prelude::*;
use std::f32::consts::PI;

/// Draws a piece centered within a given rectangle
pub fn draw_preview_piece(x: f32, y: f32, w: f32, h: f32, piece: &Bidule) {
    let scale = 1.0;
    let bs = BLOCK_SIZE * scale;

    // Calculate Bounding Box
    let positions = &piece.positions;
    if positions.is_empty() {
        return;
    }

    let min_x = positions.iter().map(|p| p.x).min().unwrap_or(0);
    let max_x = positions.iter().map(|p| p.x).max().unwrap_or(0);
    let min_y = positions.iter().map(|p| p.y).min().unwrap_or(0);
    let max_y = positions.iter().map(|p| p.y).max().unwrap_or(0);

    let piece_w = (max_x - min_x + 1) as f32 * bs;
    let piece_h = (max_y - min_y + 1) as f32 * bs;

    // Center the piece
    let offset_x = x + w / 2.0 - piece_w / 2.0 - (min_x as f32 * bs);
    let offset_y = y + h / 2.0 - piece_h / 2.0 - (min_y as f32 * bs);

    for p in positions.iter() {
        let bx = p.x;
        let by = p.y;

        let check = |dx: i32, dy: i32| -> Option<Color> {
            if positions
                .iter()
                .any(|pp| pp.x == bx + dx && pp.y == by + dy)
            {
                Some(piece.color)
            } else {
                None
            }
        };
        let neighbors = Connectivity {
            top: check(0, -1),
            bottom: check(0, 1),
            left: check(-1, 0),
            right: check(1, 0),
        };

        draw_jelly_block(
            offset_x + bx as f32 * bs,
            offset_y + by as f32 * bs,
            bs,
            piece.color,
            neighbors,
            false,
        );
    }
}

/// Helper to draw a rounded rectangle
pub fn draw_rounded_rect(x: f32, y: f32, w: f32, h: f32, r: f32, color: Color) {
    if r <= 0.0 {
        draw_rectangle(x, y, w, h, color);
        return;
    }
    let r = f32::min(r, f32::min(w, h) / 2.0);

    // 1. Center Body (Full Width, Vertical Middle)
    draw_rectangle(x, y + r, w, h - 2.0 * r, color);

    draw_rectangle(x + r, y, w - 2.0 * r, r, color);

    draw_rectangle(x + r, y + h - r, w - 2.0 * r, r, color);
    let draw_sector = |cx: f32, cy: f32, radius: f32, start_angle: f32, end_angle: f32| {
        let segments = 12;
        let step = (end_angle - start_angle) / segments as f32;

        for i in 0..segments {
            let a1 = start_angle + step * i as f32;
            let a2 = start_angle + step * (i + 1) as f32;

            draw_triangle(
                Vec2::new(cx, cy),
                Vec2::new(cx + a1.cos() * radius, cy + a1.sin() * radius),
                Vec2::new(cx + a2.cos() * radius, cy + a2.sin() * radius),
                color,
            );
        }
    };

    draw_sector(x + r, y + r, r, PI, PI * 1.5);
    draw_sector(x + w - r, y + r, r, PI * 1.5, PI * 2.0);
    draw_sector(x + w - r, y + h - r, r, 0.0, PI * 0.5);
    draw_sector(x + r, y + h - r, r, PI * 0.5, PI);
}

/// Helper to draw a rectangle with a hardware-accelerated vertex gradient
pub fn draw_mesh_gradient_rect(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    c1: Color,
    c2: Color,
    is_vertical: bool,
) {
    let (v1_col, v2_col, v3_col, v4_col) = if is_vertical {
        (c1, c1, c2, c2)
    } else {
        (c1, c2, c1, c2)
    };

    let vertices = [
        Vertex {
            position: Vec3::new(x, y, 0.0),
            uv: Vec2::ZERO,
            color: v1_col.into(),
            normal: Vec4::new(0.0, 0.0, 1.0, 0.0),
        },
        Vertex {
            position: Vec3::new(x + w, y, 0.0),
            uv: Vec2::ZERO,
            color: v2_col.into(),
            normal: Vec4::new(0.0, 0.0, 1.0, 0.0),
        },
        Vertex {
            position: Vec3::new(x, y + h, 0.0),
            uv: Vec2::ZERO,
            color: v3_col.into(),
            normal: Vec4::new(0.0, 0.0, 1.0, 0.0),
        },
        Vertex {
            position: Vec3::new(x + w, y + h, 0.0),
            uv: Vec2::ZERO,
            color: v4_col.into(),
            normal: Vec4::new(0.0, 0.0, 1.0, 0.0),
        },
    ];

    let indices = [0, 1, 2, 1, 2, 3];

    let mesh = Mesh {
        vertices: vertices.to_vec(),
        indices: indices.to_vec(),
        texture: None,
    };

    draw_mesh(&mesh);
}

#[derive(Clone, Copy)]
pub struct Connectivity {
    pub top: Option<Color>,
    pub right: Option<Color>,
    pub bottom: Option<Color>,
    pub left: Option<Color>,
}

/// Draws an individual "Jelly" block with connected textures
pub fn draw_jelly_block(
    x: f32,
    y: f32,
    size: f32,
    color: Color,
    neighbors: Connectivity,
    is_ghost: bool,
) {
    let padding = 1.0;

    // Ghost Logic
    if is_ghost {
        draw_rounded_rect(
            x + padding,
            y + padding,
            size - padding * 2.0,
            size - padding * 2.0,
            BLOCK_ROUNDING,
            Color::new(color.r, color.g, color.b, 0.3),
        );
        return;
    }

    // --- Enhanced Wobble Physics ---
    let time = get_time();
    let wobble_speed = 3.0; // Slightly slower, heavier wobble
                            // Reduce spatial frequency to sync adjacent blocks (BLOCK_SIZE is ~30.0)
                            // 0.05 * 30.0 = 1.5 radians (approx 90 deg phase shift per block)
                            // Let's go even lower for tighter sync: 0.02 * 30.0 = 0.6 radians
    let phase = (x * 0.02 + y * 0.02) as f64;

    // Compound sine wave for organic "jiggle"
    let wobble_x = ((time * wobble_speed + phase).sin()
        + (time * wobble_speed * 1.5 + phase * 2.0).sin() * 0.3) as f32
        * 1.5;
    let wobble_y = ((time * wobble_speed * 1.2 + phase).cos()
        + (time * wobble_speed * 2.0 + phase).cos() * 0.3) as f32
        * 1.5;

    // Apply dampened wobble to position
    let wx = x + wobble_x * 0.4;
    let wy = y + wobble_y * 0.4;

    // Breathing effect
    let breathe = ((time * 2.5 + phase).sin() * 1.0) as f32;

    // --- Layered Geometry Helper ---
    // We need to draw the same connected shape multiple times with different sizes/colors
    // radius_mod can be negative to shrink (inset) or positive to grow
    // --- Layered Geometry Helper ---
    let draw_layer = |inset: f32, color_transform: &dyn Fn(Color) -> Color| {
        let current_size = size - (padding + inset) * 2.0 + breathe;
        let current_r = f32::max(1.0, BLOCK_ROUNDING - inset * 0.5);

        let cx = wx + padding + inset - breathe * 0.5;
        let cy = wy + padding + inset - breathe * 0.5;

        // Base rounded rect (The "Node") - Always our color
        let my_layer_color = color_transform(color);
        draw_rounded_rect(
            cx,
            cy,
            current_size,
            current_size,
            current_r,
            my_layer_color,
        );

        // Connectors (The "Bridges")
        let bridge_overlap = 3.0;

        // Top Connector
        if let Some(nc) = neighbors.top {
            let neighbor_layer_color = color_transform(nc);
            let h = (cy - y) + bridge_overlap;

            if nc == color {
                draw_rectangle(cx, y, current_size, h, my_layer_color);
            } else {
                // PREMIUM: Smooth Hardware Gradient from Neighbor (Top) to Us (Bottom)
                draw_mesh_gradient_rect(
                    cx,
                    y,
                    current_size,
                    h,
                    neighbor_layer_color,
                    my_layer_color,
                    true,
                );
            }
        }
        // Bottom Connector
        if let Some(nc) = neighbors.bottom {
            let neighbor_layer_color = color_transform(nc);
            let start_y = cy + current_size - bridge_overlap;
            let h = (y + size) - start_y;

            if nc == color {
                draw_rectangle(cx, start_y, current_size, h, my_layer_color);
            } else {
                // PREMIUM: Smooth Hardware Gradient from Us (Top) to Neighbor (Bottom)
                draw_mesh_gradient_rect(
                    cx,
                    start_y,
                    current_size,
                    h,
                    my_layer_color,
                    neighbor_layer_color,
                    true,
                );
            }
        }
        // Left Connector
        if let Some(nc) = neighbors.left {
            let neighbor_layer_color = color_transform(nc);
            let w = (cx - x) + bridge_overlap;

            if nc == color {
                draw_rectangle(x, cy, w, current_size, my_layer_color);
            } else {
                // PREMIUM: Smooth Hardware Gradient from Neighbor (Left) to Us (Right)
                draw_mesh_gradient_rect(
                    x,
                    cy,
                    w,
                    current_size,
                    neighbor_layer_color,
                    my_layer_color,
                    false,
                );
            }
        }
        // Right Connector
        if let Some(nc) = neighbors.right {
            let neighbor_layer_color = color_transform(nc);
            let start_x = cx + current_size - bridge_overlap;
            let w = (x + size) - start_x;

            if nc == color {
                draw_rectangle(start_x, cy, w, current_size, my_layer_color);
            } else {
                // PREMIUM: Smooth Hardware Gradient from Us (Left) to Neighbor (Right)
                draw_mesh_gradient_rect(
                    start_x,
                    cy,
                    w,
                    current_size,
                    my_layer_color,
                    neighbor_layer_color,
                    false,
                );
            }
        }

        // Corner Fills - simplified to our color for performance, as they are small
        let corner_col = my_layer_color;
        if neighbors.top.is_some() && neighbors.left.is_some() {
            let w = (cx - x) + bridge_overlap;
            let h = (cy - y) + bridge_overlap;
            draw_rectangle(x, y, w, h, corner_col);
        }
        if neighbors.top.is_some() && neighbors.right.is_some() {
            let start_x = cx + current_size - bridge_overlap;
            let w = (x + size) - start_x;
            let h = (cy - y) + bridge_overlap;
            draw_rectangle(start_x, y, w, h, corner_col);
        }
        if neighbors.bottom.is_some() && neighbors.left.is_some() {
            let w = (cx - x) + bridge_overlap;
            let start_y = cy + current_size - bridge_overlap;
            let h = (y + size) - start_y;
            draw_rectangle(x, start_y, w, h, corner_col);
        }
        if neighbors.bottom.is_some() && neighbors.right.is_some() {
            let start_x = cx + current_size - bridge_overlap;
            let w = (x + size) - start_x;
            let start_y = cy + current_size - bridge_overlap;
            let h = (y + size) - start_y;
            draw_rectangle(start_x, start_y, w, h, corner_col);
        }
    };

    // --- 1. Rim Layer (Outer Glow) ---
    draw_layer(0.0, &|c| {
        Color::new(
            f32::min(1.0, c.r + 0.2),
            f32::min(1.0, c.g + 0.2),
            f32::min(1.0, c.b + 0.2),
            0.9,
        )
    });

    // --- 2. Body Layer (Translucent Jelly) ---
    draw_layer(2.0, &|c| Color::new(c.r, c.g, c.b, 0.75));

    // --- 3. Inner Core (Denser Volume) ---
    draw_layer(6.0, &|c| Color::new(c.r * 0.8, c.g * 0.8, c.b * 0.8, 0.95));

    // --- 3.5 Color Bleed Mixing ---
    // [Logic Removed]

    // --- 4. Glossy Highlights (Wetness) ---
    // These do not need to connect (lights don't merge the same way, usually)
    // But for a gummy look, maybe they should just be on the 'ridges'.
    // Let's keep the localized bubble highlights for now.

    let shine_color = Color::new(1.0, 1.0, 1.0, 0.5);
    // Main blob highlight (Top Left)
    if neighbors.top.is_none() && neighbors.left.is_none() {
        draw_circle(
            wx + padding + size * 0.25,
            wy + padding + size * 0.25,
            size * 0.15,
            shine_color,
        );
        // Streaks
        draw_rectangle(
            wx + padding + size * 0.15,
            wy + padding + size * 0.15,
            size * 0.1,
            size * 0.3,
            shine_color,
        );
    }

    // Secondary "wet" dot (Bottom Right)
    // Only if exposed
    if neighbors.bottom.is_none() && neighbors.right.is_none() {
        let wet_color = Color::new(1.0, 1.0, 1.0, 0.8);
        draw_circle(
            wx + padding + size * 0.75,
            wy + padding + size * 0.75,
            size * 0.08,
            wet_color,
        );
    }
}

/// Draws a "Jelly Frame" UI Panel with premium effects
pub fn draw_panel(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    title: Option<&str>,
    font: Option<&Font>,
    theme_color: Color,
) {
    let time = get_time();
    let border_thick = 8.0;

    // --- 1. Procedural Glow (Aura) ---
    // Multiple layers of expanding translucent rectangles for a soft glow
    for i in 1..=4 {
        let alpha = 0.15 / i as f32;
        let expand = i as f32 * 4.0;
        let glow_col = Color::new(theme_color.r, theme_color.g, theme_color.b, alpha);
        draw_rounded_rect(
            x - border_thick - expand,
            y - border_thick - expand,
            w + (border_thick + expand) * 2.0,
            h + (border_thick + expand) * 2.0,
            UI_ROUNDING + expand,
            glow_col,
        );
    }

    // --- 2. Shadow/Outer Edge ---
    draw_rounded_rect(
        x - border_thick - 2.0,
        y - border_thick - 2.0,
        w + (border_thick + 2.0) * 2.0,
        h + (border_thick + 2.0) * 2.0,
        UI_ROUNDING + 5.0,
        Color::new(0.0, 0.0, 0.1, 0.6),
    );

    // --- 3. Main Jelly Frame Body ---
    draw_rounded_rect(
        x - border_thick,
        y - border_thick,
        w + border_thick * 2.0,
        h + border_thick * 2.0,
        UI_ROUNDING + 2.0,
        theme_color,
    );

    // --- 4. Inner Bevel/Stroke ---
    draw_rounded_rect(
        x - 2.0,
        y - 2.0,
        w + 4.0,
        h + 4.0,
        UI_ROUNDING,
        Color::new(0.0, 0.0, 0.0, 0.4),
    );

    // --- 5. Glass Background ---
    let glass_color = Color::new(COLOR_UI_BG.r, COLOR_UI_BG.g, COLOR_UI_BG.b, 0.85);
    draw_rounded_rect(x, y, w, h, UI_ROUNDING, glass_color);

    // --- 6. Animated Glass Reflection (Sweep) ---
    // A light streak that "sweeps" across the panel periodically
    let sweep_speed = 0.4;
    let sweep_width = 80.0;
    let sweep_t = (time * sweep_speed) % 2.5; // Offset to add pause
    let sweep_x = x - sweep_width + sweep_t as f32 * (w + sweep_width);

    if sweep_t < 1.0 {
        // Only draw when within bounds (normalized sweep)
        let alpha = (1.0 - (sweep_t - 0.5).abs() * 2.0).max(0.0) as f32 * 0.15;
        // Draw diagonal streak clipped to rectangle (simplified: vertical streak)
        let rect_x = f32::max(x, sweep_x);
        let rect_w = f32::min(x + w, sweep_x + sweep_width) - rect_x;

        if rect_w > 0.0 {
            draw_rectangle(
                rect_x,
                y,
                rect_w,
                h,
                Color::new(1.0, 1.0, 1.0, alpha),
            );
        }
    }

    // --- 7. Pulsing Rim Highlight ---
    let pulse = (time * 2.5).sin() as f32 * 0.1 + 0.25;
    draw_rounded_rect(
        x - border_thick + 5.0,
        y - border_thick + 2.0,
        w + border_thick * 2.0 - 10.0,
        4.0,
        4.0,
        Color::new(1.0, 1.0, 1.0, pulse),
    );

    if let Some(text) = title {
        let title_x;
        let title_y = y - 25.0;

        if let Some(f) = font {
            let dim = measure_text(text, Some(f), 35, 1.0);
            title_x = x + (w - dim.width) / 2.0;
        } else {
            title_x = x + 15.0;
        }

        if let Some(f) = font {
            // Shadow
            draw_text_ex(text, title_x + 2.0, title_y + 2.0, TextParams {
                font: Some(f),
                font_size: 35,
                color: Color::new(0.0, 0.0, 0.1, 0.8),
                ..Default::default()
            });

            // Bright Title
            let title_color = Color::new(
                f32::min(1.0, theme_color.r + 0.6),
                f32::min(1.0, theme_color.g + 0.6),
                f32::min(1.0, theme_color.b + 0.6),
                1.0,
            );

            draw_text_ex(text, title_x, title_y, TextParams {
                font: Some(f),
                font_size: 35,
                color: title_color,
                ..Default::default()
            });
        } else {
            draw_text(text, title_x, title_y, 30.0, WHITE);
        }
    }
}

use crate::game::{Game, GameState};

/// Main drawing function for the game
pub fn draw_game(game: &Game) {
    // 1. Background (Nature Theme)
    game.background.draw();

    // 0. Game States (Start / Game Over / Playing)
    match game.state {
        GameState::Start => {
            draw_start_screen(game);
        }
        GameState::Playing | GameState::GameOver => {
            draw_play_scene(game);
        }
    }
}

fn draw_play_scene(game: &Game) {
    // Layout Constants
    let board_w = GRID_WIDTH as f32 * BLOCK_SIZE;
    let board_h = GRID_HEIGHT as f32 * BLOCK_SIZE;
    let spacing = 80.0;
    let side_panel_w = 260.0;
    let stats_h = 160.0;

    let total_w = side_panel_w + spacing + board_w + spacing + side_panel_w;
    let total_content_h = board_h;

    // Apply Screen Shake
    let shake_x = (fastrand::f32() - 0.5) * game.screen_shake;
    let shake_y = (fastrand::f32() - 0.5) * game.screen_shake;

    let offset_x = (screen_width() - total_w) / 2.0 + shake_x;
    let offset_y = (screen_height() - total_content_h) / 2.0 + shake_y;

    let next_x = offset_x;
    let grid_x = next_x + side_panel_w + spacing;
    let hold_x = grid_x + board_w + spacing;

    let grid_y = offset_y;
    let stats_y = grid_y;

    let font_ref = game.font.as_ref();

    // --- 1. Stats Panel (Left) ---
    let stats_color = Color::new(0.0, 0.5, 0.9, 1.0);
    draw_panel(
        next_x,
        stats_y,
        side_panel_w,
        stats_h,
        Some("STATUS"),
        font_ref,
        stats_color,
    );

    if let Some(f) = font_ref {
        let pulse_scale = 1.0 + (game.ui_pulse * 0.2);
        let score_text = format!("{:06}", game.score);
        let lvl_text = format!("LEVEL {}", game.level);
        let lines_text = format!("LINES {}", game.lines_cleared_total);

        let draw_stat = |txt: &str, dy: f32, size: u16, col: Color| {
            let dim = measure_text(txt, Some(f), size, 1.0);
            let tx = next_x + (side_panel_w - dim.width * pulse_scale) / 2.0;
            draw_text_ex(txt, tx, stats_y + dy, TextParams {
                font: Some(f),
                font_size: (size as f32 * pulse_scale) as u16,
                color: col,
                ..Default::default()
            });
        };

        draw_stat(&score_text, 50.0, 40, GOLD);
        draw_stat(&lvl_text, 100.0, 30, WHITE);
        draw_stat(&lines_text, 140.0, 25, Color::new(0.7, 0.9, 1.0, 1.0));
    }

    // --- 2. Next Piece Panel (Left, below Stats) ---
    let next_panel_y = stats_y + stats_h + 40.0;
    let next_panel_h = 180.0;
    draw_panel(
        next_x,
        next_panel_y,
        side_panel_w,
        next_panel_h,
        Some("NEXT"),
        font_ref,
        Color::new(0.0, 0.7, 0.3, 1.0),
    );
    if let Some(next_piece) = game.next_pieces.first() {
        draw_preview_piece(next_x, next_panel_y, side_panel_w, next_panel_h, next_piece);
    }

    // --- 3. Hold Panel (Right) ---
    let hold_panel_h = 180.0;
    draw_panel(
        hold_x,
        stats_y,
        side_panel_w,
        hold_panel_h,
        Some("HOLD"),
        font_ref,
        Color::new(0.7, 0.2, 0.8, 1.0),
    );
    if let Some(hold_piece) = &game.hold_piece {
        draw_preview_piece(hold_x, stats_y, side_panel_w, hold_panel_h, hold_piece);
    }

    // 0.5 Music Button (Bottom Left Icon)
    let btn_size = 50.0;
    let btn_x = 50.0;
    let btn_y = screen_height() - 100.0;

    let icon_color = if game.is_music_playing {
        Color::new(0.2, 0.8, 0.2, 1.0) // Green
    } else {
        Color::new(0.8, 0.2, 0.2, 1.0) // Red
    };

    // Background
    draw_panel(btn_x, btn_y, btn_size, btn_size, None, None, icon_color);

    // Speaker Icon Shape
    let cx = btn_x + btn_size / 2.0;
    let cy = btn_y + btn_size / 2.0;

    // Speaker Body
    let body_color = WHITE;
    draw_rectangle(btn_x + 10.0, cy - 8.0, 6.0, 16.0, body_color);
    draw_triangle(
        Vec2::new(btn_x + 16.0, cy - 8.0),
        Vec2::new(btn_x + 16.0, cy + 8.0),
        Vec2::new(btn_x + 30.0, cy + 14.0),
        body_color,
    );
    draw_triangle(
        Vec2::new(btn_x + 16.0, cy - 8.0),
        Vec2::new(btn_x + 30.0, cy + 14.0),
        Vec2::new(btn_x + 30.0, cy - 14.0),
        body_color,
    );

    // Sound Waves (if ON) or Cross (if OFF)?
    // User requested: "red sound icon for off and a green sound icon fo when on"
    // I made the background red/green. I will add sound waves if ON.
    if game.is_music_playing {
        let wave_c = WHITE;
        // Small wave
        draw_poly_lines(cx + 5.0, cy, 3, 8.0, 30.0, 2.0, wave_c);
        // Macroquad doesn't have easy arc primitive, we can approximate or just use lines like `)`
        // Hacky curve: 3 segments
        draw_line(btn_x + 34.0, cy - 5.0, btn_x + 36.0, cy, 2.0, wave_c);
        draw_line(btn_x + 36.0, cy, btn_x + 34.0, cy + 5.0, 2.0, wave_c);

        draw_line(btn_x + 38.0, cy - 10.0, btn_x + 42.0, cy, 2.0, wave_c);
        draw_line(btn_x + 42.0, cy, btn_x + 38.0, cy + 10.0, 2.0, wave_c);
    } else {
        // X mark for OFF
        draw_line(btn_x + 35.0, cy - 5.0, btn_x + 45.0, cy + 5.0, 3.0, WHITE);
        draw_line(btn_x + 45.0, cy - 5.0, btn_x + 35.0, cy + 5.0, 3.0, WHITE);
    }

    // 1. Next Pieces
    let next_color = Color::new(0.0, 0.6, 1.0, 1.0);
    let side_panel_h = 220.0;
    let side_panel_y = grid_y - 30.0;

    draw_panel(
        next_x,
        side_panel_y,
        side_panel_w,
        side_panel_h,
        Some("NEXT"),
        font_ref,
        next_color,
    );

    if let Some(next_piece) = game.next_pieces.first() {
        draw_preview_piece(next_x, side_panel_y, side_panel_w, side_panel_h, next_piece);
    }

    // 2. Hold Piece
    let hold_color = Color::new(0.6, 0.0, 1.0, 1.0);
    draw_panel(
        hold_x,
        side_panel_y,
        side_panel_w,
        side_panel_h,
        Some("HOLD"),
        font_ref,
        hold_color,
    );

    if let Some(hold_piece) = &game.hold_piece {
        draw_preview_piece(hold_x, side_panel_y, side_panel_w, side_panel_h, hold_piece);
    }

    // --- 4. Grid Panel ---
    let grid_border_color = Color::new(0.4, 0.9, 0.1, 1.0);
    draw_panel(
        grid_x,
        grid_y,
        board_w,
        board_h,
        None,
        font_ref,
        grid_border_color,
    );

    // Grid Lines
    for x in 1..GRID_WIDTH {
        draw_line(
            grid_x + x as f32 * BLOCK_SIZE,
            grid_y,
            grid_x + x as f32 * BLOCK_SIZE,
            grid_y + board_h,
            1.0,
            Color::new(1.0, 1.0, 1.0, 0.1),
        );
    }
    for y in 1..GRID_HEIGHT {
        draw_line(
            grid_x,
            grid_y + y as f32 * BLOCK_SIZE,
            grid_x + board_w,
            grid_y + y as f32 * BLOCK_SIZE,
            1.0,
            Color::new(1.0, 1.0, 1.0, 0.1),
        );
    }

    // Draw Grid Blocks
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if let Some(color) = game.grid.cells[y][x] {
                let check_neighbor = |nx: i32, ny: i32| -> Option<Color> {
                    if nx < 0 || nx >= GRID_WIDTH as i32 || ny < 0 || ny >= GRID_HEIGHT as i32 {
                        return None;
                    }
                    game.grid.cells[ny as usize][nx as usize]
                };

                let neighbors = Connectivity {
                    top: check_neighbor(x as i32, y as i32 - 1),
                    right: check_neighbor(x as i32 + 1, y as i32),
                    bottom: check_neighbor(x as i32, y as i32 + 1),
                    left: check_neighbor(x as i32 - 1, y as i32),
                };

                draw_jelly_block(
                    grid_x + x as f32 * BLOCK_SIZE,
                    grid_y + y as f32 * BLOCK_SIZE,
                    BLOCK_SIZE,
                    color,
                    neighbors,
                    false,
                );
            }
        }
    }

    // Helper to get connectivity for active piece
    let get_piece_conn = |pos: crate::bidule::Point,
                          all: &[crate::bidule::Point; 4],
                          my_color: Color|
     -> Connectivity {
        let mut conn = Connectivity {
            top: None,
            right: None,
            bottom: None,
            left: None,
        };
        for other in all.iter() {
            if other.x == pos.x && other.y == pos.y - 1 {
                conn.top = Some(my_color);
            }
            if other.x == pos.x + 1 && other.y == pos.y {
                conn.right = Some(my_color);
            }
            if other.x == pos.x && other.y == pos.y + 1 {
                conn.bottom = Some(my_color);
            }
            if other.x == pos.x - 1 && other.y == pos.y {
                conn.left = Some(my_color);
            }
        }
        conn
    };

    // Draw Ghost Piece
    let ghost = game.get_ghost_position(); // We might need to make get_ghost_position pub
    for p in game.current_piece.positions.iter() {
        let x = ghost.x + p.x;
        let y = ghost.y + p.y;
        if y >= 0 {
            let neighbors =
                get_piece_conn(*p, &game.current_piece.positions, game.current_piece.color);
            draw_jelly_block(
                grid_x + x as f32 * BLOCK_SIZE,
                grid_y + y as f32 * BLOCK_SIZE,
                BLOCK_SIZE,
                game.current_piece.color,
                neighbors,
                true,
            );
        }
    }

    // Draw Current Piece
    for p in game.current_piece.positions.iter() {
        let x = game.current_piece.pos.x + p.x;
        let y = game.current_piece.pos.y + p.y;
        if y >= 0 {
            let neighbors =
                get_piece_conn(*p, &game.current_piece.positions, game.current_piece.color);
            draw_jelly_block(
                grid_x + x as f32 * BLOCK_SIZE,
                grid_y + y as f32 * BLOCK_SIZE,
                BLOCK_SIZE,
                game.current_piece.color,
                neighbors,
                false,
            );
        }
    }

    // Draw Effects & Particles (Relative to grid) ---
    // Actually effects are screen-space in current game.rs implementation, let's keep them as is.

    // Draw Effects
    for e in &game.effects {
        e.draw();
    }

    // Draw Particles
    for p in &game.particles {
        p.draw(grid_x, grid_y);
    }

    // Overlay Game Over
    if game.state == GameState::GameOver {
        draw_game_over(game);
    }
}

fn draw_start_screen(game: &Game) {
    let sw = screen_width();
    let sh = screen_height();
    let font_ref = game.font.as_ref();
    let time = get_time();

    // --- 1. Floating Background Pieces ---
    // Draw 10 random pieces slowly drifting
    for i in 0..10 {
        let x = (i as f32 * 200.0 + time as f32 * 20.0) % sw;
        let y = (i as f32 * 100.0 + (time as f32 * 0.5).sin() * 50.0) % sh;
        let color = Color::new(1.0, 1.0, 1.0, 0.05);
        draw_circle(x, y, 40.0, color);
    }

    // Darken background
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.5));

    let panel_w = 700.0;
    let panel_h = 450.0;
    let px = (sw - panel_w) / 2.0;
    let py = (sh - panel_h) / 2.0;

    // Breathing Logo effect
    let breathe = (time * 2.0).sin() as f32 * 5.0;
    draw_panel(px, py + breathe, panel_w, panel_h, Some("RUST TETRIS JELLY"), font_ref, COLOR_PURPLE);

    if let Some(f) = font_ref {
        let msg = "PRESS [SPACE] TO START";
        let dim = measure_text(msg, Some(f), 50, 1.0);
        let tx = px + (panel_w - dim.width) / 2.0;
        let ty = py + panel_h / 2.0 + 30.0;

        let p = (time * 3.0).sin() as f32 * 0.2 + 0.8;
        let col = Color::new(p, p, 1.0, 1.0);

        draw_text_ex(msg, tx, ty, TextParams {
            font: Some(f),
            font_size: 50,
            color: col,
            ..Default::default()
        });

        let sub_msg = "⬅️ ➡️ to move | ⬆️ to rotate | SPACE to drop | C to hold";
        let sdim = measure_text(sub_msg, Some(f), 24, 1.0);
        draw_text_ex(sub_msg, px + (panel_w - sdim.width) / 2.0, py + panel_h - 50.0, TextParams {
            font: Some(f),
            font_size: 24,
            color: Color::new(0.8, 0.8, 0.8, 1.0),
            ..Default::default()
        });
    }
}

fn draw_game_over(game: &Game) {
    let sw = screen_width();
    let sh = screen_height();
    let font_ref = game.font.as_ref();

    // Thick dark overlay
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.1, 0.7));

    let panel_w = 600.0;
    let panel_h = 350.0;
    let px = (sw - panel_w) / 2.0;
    let py = (sh - panel_h) / 2.0;

    draw_panel(px, py, panel_w, panel_h, Some("GAME OVER"), font_ref, COLOR_RED);

    if let Some(f) = font_ref {
        let score_txt = format!("FINAL SCORE: {}", game.score);
        let dim = measure_text(&score_txt, Some(f), 45, 1.0);
        draw_text_ex(&score_txt, px + (panel_w - dim.width) / 2.0, py + panel_h / 2.0, TextParams {
            font: Some(f),
            font_size: 45,
            color: GOLD,
            ..Default::default()
        });

        let restart_txt = "PRESS [R] TO RESTART";
        let rdim = measure_text(restart_txt, Some(f), 30, 1.0);
        draw_text_ex(restart_txt, px + (panel_w - rdim.width) / 2.0, py + panel_h - 60.0, TextParams {
            font: Some(f),
            font_size: 30,
            color: WHITE,
            ..Default::default()
        });
    }
}

/// Helper for basic styled text
pub fn draw_text_styled(text: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(text, x + 2.0, y + 2.0, size, COLOR_TEXT_SHADOW);
    draw_text(text, x, y, size, color);
}
