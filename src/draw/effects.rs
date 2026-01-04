
use macroquad::prelude::*;
use crate::constants::*;


/// Draws a GIANT Jelly Circle with all the layered effects
pub fn draw_giant_jelly_circle(
    cx: f32,
    cy: f32,
    radius: f32,
    color: Color,
    is_ghost: bool,
) {
     let padding = JELLY_PADDING;
     
    // Ghost Logic
    if is_ghost {
         draw_circle_lines(cx, cy, radius, 3.0, Color::new(color.r, color.g, color.b, 0.25));
        return;
    }

    // --- Enhanced Wobble Physics (Copied & Adapted) ---
    let time = get_time();
    let wobble_speed = WOBBLE_SPEED;
    
    // Low frequency wobble for the giant mass
    let wobble_x = ((time * wobble_speed).sin() + (time * wobble_speed * 1.5).sin() * 0.3) as f32 * 2.0;
    let wobble_y = ((time * wobble_speed * 1.2).cos() + (time * wobble_speed * 2.0).cos() * 0.3) as f32 * 2.0;

    // Apply dampened wobble to center position
    let wx = cx + wobble_x;
    let wy = cy + wobble_y;

    // Breathing effect
    let breathe = ((time * 2.5).sin() * 2.0) as f32;

    // --- Layered Geometry Helper ---
    let draw_layer = |inset: f32, color_transform: &dyn Fn(Color) -> Color| {
        let current_radius = radius - padding - inset + breathe;
        
        // Safety check for negative radius
        if current_radius <= 0.0 { return; }

        let my_layer_color = color_transform(color);

        draw_circle(wx, wy, current_radius, my_layer_color);
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
    draw_layer(4.0, &|c| Color::new(c.r, c.g, c.b, 0.75));

    // --- 3. Inner Core (Denser Volume) ---
    draw_layer(12.0, &|c| Color::new(c.r * 0.8, c.g * 0.8, c.b * 0.8, 0.95));

    // --- 4. Glossy Highlights (Wetness) ---
    let shine_color = Color::new(1.0, 1.0, 1.0, 0.5);
    // Main big highlight
    draw_circle(
        wx - radius * 0.3,
        wy - radius * 0.3,
        radius * 0.25,
        shine_color,
    );
    // Smaller secondary
    draw_circle(
        wx + radius * 0.4,
        wy + radius * 0.4,
        radius * 0.1,
        Color::new(1.0, 1.0, 1.0, 0.8),
    );
}

/// Draws a GIANT Bomb Diamond (Rotated Square) with Lava effects
pub fn draw_giant_bomb_diamond(
    cx: f32,
    cy: f32,
    radius: f32, // This is roughly half-width
    color: Color,
    is_ghost: bool,
) {
    // Ghost Logic
    if is_ghost {
        // Draw Diamond Outline
        // (cx, cy-r), (cx+r, cy), (cx, cy+r), (cx-r, cy)
        let top = Vec2::new(cx, cy - radius);
        let right = Vec2::new(cx + radius, cy);
        let bot = Vec2::new(cx, cy + radius);
        let left = Vec2::new(cx - radius, cy);
        
        let c = Color::new(color.r, color.g, color.b, 0.25);
        draw_line(top.x, top.y, right.x, right.y, 2.0, c);
        draw_line(right.x, right.y, bot.x, bot.y, 2.0, c);
        draw_line(bot.x, bot.y, left.x, left.y, 2.0, c);
        draw_line(left.x, left.y, top.x, top.y, 2.0, c);
        return;
    }

    let time = get_time();
    
    // Pulsing size
    let pulse = (time * 8.0).sin() as f32 * 2.0;
    let r = radius + pulse;

    // Vertices
    let top = Vec2::new(cx, cy - r);
    let right = Vec2::new(cx + r, cy);
    let bot = Vec2::new(cx, cy + r);
    let left = Vec2::new(cx - r, cy);

    // 1. Base Lava Layer (Orange/Red gradient simulated by layers?)
    draw_triangle(top, right, bot, color);
    draw_triangle(bot, left, top, color);

    // 2. Inner Heat (Yellow/White center)
    let inner_r = r * 0.6;
    let i_top = Vec2::new(cx, cy - inner_r);
    let i_right = Vec2::new(cx + inner_r, cy);
    let i_bot = Vec2::new(cx, cy + inner_r);
    let i_left = Vec2::new(cx - inner_r, cy);
    
    let heat_col = Color::new(1.0, 0.6, 0.0, 1.0); // Bright Orange
    draw_triangle(i_top, i_right, i_bot, heat_col);
    draw_triangle(i_bot, i_left, i_top, heat_col);

    // 3. Cracks / Dark Spots (Noise)
    // Simple dark jagged lines or patches
    let dark_col = Color::new(0.4, 0.1, 0.0, 0.6);
    draw_circle(cx + r*0.3, cy - r*0.2, r*0.15, dark_col);
    draw_circle(cx - r*0.2, cy + r*0.3, r*0.1, dark_col);
    
    // 4. White Hot Core
    draw_circle(cx, cy, r * 0.2, Color::new(1.0, 1.0, 0.8, 1.0));
}



