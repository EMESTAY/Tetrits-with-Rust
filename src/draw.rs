use crate::bidule::Bidule;
use crate::constants::*;
use macroquad::prelude::*;

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
    // 1. Center Body (Full Width, Vertical Middle)
    draw_rectangle(x, y + r, w, h - 2.0 * r, color);

    draw_rectangle(x + r, y, w - 2.0 * r, r, color);

    draw_rectangle(x + r, y + h - r, w - 2.0 * r, r, color);
    let draw_sector = |cx: f32, cy: f32, radius: f32, start_angle: f32, end_angle: f32| {
        let segments = 10;
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

    //TODO Du coup [pi, 3pi/2] grossomerdo [3.14, 4.71]
    draw_sector(
        x + r,
        y + r,
        r,
        std::f32::consts::PI,
        std::f32::consts::PI * 1.5,
    );

    draw_sector(
        x + w - r,
        y + r,
        r,
        std::f32::consts::PI * 1.5,
        std::f32::consts::PI * 2.0,
    );

    draw_sector(x + w - r, y + h - r, r, 0.0, std::f32::consts::PI * 0.5);

    draw_sector(
        x + r,
        y + h - r,
        r,
        std::f32::consts::PI * 0.5,
        std::f32::consts::PI,
    );
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

pub fn draw_sky_gradient() {
    let top = COLOR_SKY_TOP;
    let bot = COLOR_SKY_BOTTOM;

    // Draw vertical stripes to simulate gradient if texture not available
    let steps = 20;
    let h = screen_height() / steps as f32;
    for i in 0..steps {
        let t = i as f32 / steps as f32;
        let color = Color::new(
            top.r + (bot.r - top.r) * t,
            top.g + (bot.g - top.g) * t,
            top.b + (bot.b - top.b) * t,
            1.0,
        );
        draw_rectangle(0.0, i as f32 * h, screen_width(), h + 1.0, color);
    }
}

pub fn draw_clouds() {
    // Static clouds for now, can animate offset later
    let time = get_time();
    let cloud_color = Color::new(1.0, 1.0, 1.0, 0.6);

    // Helper to draw cloud clump
    let draw_cloud = |cx: f32, cy: f32, scale: f32| {
        draw_circle(cx, cy, 30.0 * scale, cloud_color);
        draw_circle(
            cx - 25.0 * scale,
            cy + 10.0 * scale,
            20.0 * scale,
            cloud_color,
        );
        draw_circle(
            cx + 25.0 * scale,
            cy + 5.0 * scale,
            22.0 * scale,
            cloud_color,
        );
    };

    // Draw a few clouds
    draw_cloud(
        100.0 + (time * 10.0) as f32 % (screen_width() + 200.0) - 100.0,
        100.0,
        1.5,
    );
    draw_cloud(
        400.0 + (time * 15.0) as f32 % (screen_width() + 200.0) - 100.0,
        300.0,
        1.0,
    );
    draw_cloud(
        800.0 + (time * 8.0) as f32 % (screen_width() + 200.0) - 100.0,
        150.0,
        1.2,
    );
}

/// Draws a "Jelly Frame" UI Panel
pub fn draw_panel(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    title: Option<&str>,
    font: Option<&Font>,
    theme_color: Color,
) {
    let _time = get_time();
    let border_thick = 8.0;

    // 1. Shadow/Outer Edge (Darker)
    draw_rounded_rect(
        x - border_thick - 2.0,
        y - border_thick - 2.0,
        w + (border_thick + 2.0) * 2.0,
        h + (border_thick + 2.0) * 2.0,
        UI_ROUNDING + 5.0,
        Color::new(0.0, 0.0, 0.0, 0.5),
    );

    // 2. Main Jelly Frame Body
    draw_rounded_rect(
        x - border_thick,
        y - border_thick,
        w + border_thick * 2.0,
        h + border_thick * 2.0,
        UI_ROUNDING + 2.0,
        theme_color,
    );

    // 3. Inner Stroke (Darker connection to glass)
    draw_rounded_rect(
        x - 2.0,
        y - 2.0,
        w + 4.0,
        h + 4.0,
        UI_ROUNDING,
        Color::new(0.0, 0.0, 0.0, 0.3),
    );

    // 4. Glass Background (Inner)
    let glass_color = Color::new(COLOR_UI_BG.r, COLOR_UI_BG.g, COLOR_UI_BG.b, 0.7);
    draw_rounded_rect(x, y, w, h, UI_ROUNDING, glass_color);

    // 5. Glossy Highlights (The "Jelly" Shine)

    // Top Edge Gloss (Subtle static/pulsing)
    let pulse = (get_time() * 2.0).sin() as f32 * 0.1 + 0.2;
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
        let title_y = y - 20.0; // Above the panel

        // Center text horizontally
        if let Some(f) = font {
            let dim = measure_text(text, Some(f), 40, 1.0);
            title_x = x + (w - dim.width) / 2.0;
        } else {
            title_x = x + 15.0; // Fallback
        }

        // Shadow
        if let Some(f) = font {
            let params = TextParams {
                font: Some(f),
                font_size: 40,
                color: Color::new(0.0, 0.0, 0.0, 0.5),
                ..Default::default()
            };
            draw_text_ex(text, title_x + 2.0, title_y + 2.0, params);

            // Colorize title
            // Use theme color but brighter
            let title_color = Color::new(
                f32::min(1.0, theme_color.r + 0.5),
                f32::min(1.0, theme_color.g + 0.5),
                f32::min(1.0, theme_color.b + 0.5),
                1.0,
            );

            let params_main = TextParams {
                font: Some(f),
                font_size: 40,
                color: title_color,
                ..Default::default()
            };
            draw_text_ex(text, title_x, title_y, params_main);
        } else {
            // Fallback
            draw_text(text, title_x, title_y, 30.0, WHITE);
        }
    }
}

/// Draws text with a "Jelly" wobble effect (updated for Font support)
pub fn draw_text_special(
    text: &str,
    x: f32,
    y: f32,
    font_size: f32,
    color: Color,
    font: Option<&Font>,
) {
    let time = get_time();
    let chars: Vec<char> = text.chars().collect();
    let mut current_x = x;

    // Basic approximate width if font is not loaded, but with font we can measure?
    // Macroquad's measure_text is global.

    for (i, c) in chars.iter().enumerate() {
        let phase = i as f64 * 0.5;
        let offset_y = ((time * 4.0 + phase).sin() * 3.0) as f32;
        let scale_pulse = 1.0 + ((time * 3.0 + phase).cos() * 0.1) as f32;

        let s = c.to_string();
        let cur_size = (font_size * scale_pulse) as u16;

        if let Some(f) = font {
            let params = TextParams {
                font: Some(f),
                font_size: cur_size,
                color: Color::new(0.0, 0.0, 0.0, 0.5),
                ..Default::default()
            };
            draw_text_ex(&s, current_x + 2.0, y + offset_y + 2.0, params);

            let params_main = TextParams {
                font: Some(f),
                font_size: cur_size,
                color: color,
                ..Default::default()
            };
            draw_text_ex(&s, current_x, y + offset_y, params_main);

            // measure advance
            let dim = measure_text(&s, Some(f), cur_size, 1.0);
            current_x += dim.width;
        } else {
            draw_text(&s, current_x, y + offset_y, font_size * scale_pulse, color);
            current_x += font_size * 0.6;
        }
    }
}

use crate::game::Game;

/// Helper for basic styled text
pub fn draw_text_styled(text: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(text, x + 2.0, y + 2.0, size, COLOR_TEXT_SHADOW);
    draw_text(text, x, y, size, color);
}

/// Main drawing function for the game
pub fn draw_game(game: &Game) {
    // 1. Background (Sky + Clouds)
    draw_sky_gradient();
    draw_clouds();

    // Layout Constants
    let board_w = GRID_WIDTH as f32 * BLOCK_SIZE;
    let board_h = GRID_HEIGHT as f32 * BLOCK_SIZE;
    let spacing = 60.0;
    let side_panel_w = 240.0;
    let score_h = 80.0;

    let total_w = side_panel_w + spacing + board_w + spacing + side_panel_w;
    let total_content_h = score_h + 50.0 + board_h;

    let offset_x = (screen_width() - total_w) / 2.0;
    let offset_y = (screen_height() - total_content_h) / 2.0;

    let next_x = offset_x;
    let grid_x = next_x + side_panel_w + spacing;
    let hold_x = grid_x + board_w + spacing;

    let score_y = offset_y;
    let grid_y = score_y + score_h + 50.0;

    let font_ref = game.font.as_ref();

    // 0. Score Panel
    let score_color = Color::new(0.0, 0.5, 0.9, 1.0);
    draw_panel(
        grid_x,
        score_y,
        board_w,
        score_h,
        Some("SCORE"),
        font_ref,
        score_color,
    );

    if let Some(f) = font_ref {
        let score_text = format!("{:06}", game.score);
        let dim = measure_text(&score_text, Some(f), 30, 1.0);
        let score_x = grid_x + (board_w - dim.width) / 2.0;
        let center_y = score_y + score_h / 2.0 + dim.height / 3.0;

        let params = TextParams {
            font: Some(f),
            font_size: 30,
            color: WHITE,
            ..Default::default()
        };
        draw_text_ex(&score_text, score_x, center_y, params);
    } else {
        draw_text_styled(
            &format!("{:06}", game.score),
            grid_x + 20.0,
            score_y + 50.0,
            30.0,
            WHITE,
        );
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

    // 3. Grid
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

    // Draw Effects
    for e in &game.effects {
        e.draw();
    }

    // Draw Particles
    for p in &game.particles {
        p.draw(grid_x, grid_y);
    }

    if game.game_over {
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.7),
        );
        draw_text_styled(
            "GAME OVER",
            screen_width() / 2.0 - 150.0,
            screen_height() / 2.0,
            80.0,
            RED,
        );
        draw_text_styled(
            "Press R to Reset",
            screen_width() / 2.0 - 100.0,
            screen_height() / 2.0 + 80.0,
            30.0,
            WHITE,
        );
    }
}
