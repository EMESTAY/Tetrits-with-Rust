use macroquad::prelude::*;
use crate::constants::*;
use crate::rect_utils::draw_rounded_rect;
use super::Connectivity;

/// Draws an individual "Jelly" block with connected textures
pub fn draw_jelly_block(
    x: f32,
    y: f32,
    size: f32,
    color: Color,
    neighbors: Connectivity,
    _is_ghost: bool,
    bubble_seed: usize,
) {
    let padding = JELLY_PADDING;

    // Ghost Logic
    // --- Enhanced Wobble Physics ---
    let time = get_time();
    let wobble_speed = WOBBLE_SPEED;
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
    let draw_layer = |inset: f32, color_transform: &dyn Fn(Color) -> Color| {
        let current_size = size - (padding + inset) * 2.0 + breathe;
        let current_r = f32::max(1.0, BLOCK_ROUNDING - inset * 0.5);

        // GHOST RENDERING (PREVIEW SHADOW)
        if _is_ghost {
            // "Just the contour of the piece" - User Request
            // We draw lines ONLY where there is NO neighbor.
            // This creates a continuous outline for the whole shape.
            
            let c = Color::new(color.r, color.g, color.b, 0.4); // Visible outline color
            let thickness = 2.0;
            
            let gx = wx + padding + inset;
            let gy = wy + padding + inset;
            let gsize = current_size;

            // Top
            if neighbors.top.is_none() {
                draw_line(gx, gy, gx + gsize, gy, thickness, c);
            }
            // Right
            if neighbors.right.is_none() {
                draw_line(gx + gsize, gy, gx + gsize, gy + gsize, thickness, c);
            }
            // Bottom
            if neighbors.bottom.is_none() {
                draw_line(gx, gy + gsize, gx + gsize, gy + gsize, thickness, c);
            }
            // Left
            if neighbors.left.is_none() {
                draw_line(gx, gy, gx, gy + gsize, thickness, c);
            }
            
            return;
        }

        let cx = wx + padding + inset - breathe * 0.5;
        let cy = wy + padding + inset - breathe * 0.5;

        let my_layer_color = color_transform(color);

        // --- DYNAMIC SHAPE RENDERING ---
        
        // Anvil Check: Dark Gray (#4d4d59 -> 0.3, 0.3, 0.35 roughly)
        // RGB: Anvil is (0.3, 0.3, 0.35)
        let is_anvil = (color.r - 0.3).abs() < 0.05 && (color.g - 0.3).abs() < 0.05 && (color.b - 0.35).abs() < 0.05;

        // Bomb Check: Orange (#f35b04)
        // RGB: (243/255, 91/255, 4/255) -> (0.953, 0.357, 0.016)
        let is_bomb = color.r > 0.9 && color.g < 0.4 && color.b < 0.1;


        
        if is_anvil {
             // --- HEAVY METAL RENDERING (Unified Hard Look) ---
             // Gunmetal Base
             let base_col = COLOR_ANVIL_BASE;
             let highlight_col = COLOR_ANVIL_HIGHLIGHT; // Brushed Steel Highlight
             let shadow_col = COLOR_ANVIL_SHADOW;    // Deep Shadow
             let border_col = BLACK;

             let mx = x + padding + inset;
             let my = y + padding + inset;
             let msize = size - (padding + inset) * 2.0;

             // 1. Draw Base (Solid)
             draw_rectangle(mx, my, msize, msize, base_col);
             
             // 2. Add Metallic Noise / Texture (Simple)
             // Use bubble_seed for consistent noise
             let noise_val = (bubble_seed % 100) as f32 / 100.0;
             if noise_val > 0.5 {
                 draw_rectangle(mx + msize*0.2, my + msize*0.2, msize*0.6, msize*0.6, Color::new(0.3, 0.35, 0.4, 0.3));
             }

             // 3. Thick Borders on EXPOSED SIDES (Silhouette)
             let thickness = 4.0;
             // Top
             if neighbors.top.is_none() {
                 draw_line(mx, my, mx + msize, my, thickness, highlight_col); // Top edge is highlight
             }
             // Left
             if neighbors.left.is_none() {
                 draw_line(mx, my, mx, my + msize, thickness, highlight_col); // Left edge is highlight
             }
             // Bottom
             if neighbors.bottom.is_none() {
                 draw_line(mx, my + msize, mx + msize, my + msize, thickness, border_col); // Bottom is dark
             }
             // Right
             if neighbors.right.is_none() {
                 draw_line(mx + msize, my, mx + msize, my + msize, thickness, border_col); // Right is dark
             }

             // Inner Plate Indent
             let plate_margin = 6.0;
             draw_rectangle_lines(mx + plate_margin, my + plate_margin, msize - plate_margin*2.0, msize - plate_margin*2.0, 2.0, shadow_col);
             
             // Central Bolt
             let bolt_r = msize * 0.15;
             draw_circle(mx + msize/2.0, my + msize/2.0, bolt_r, Color::new(0.7, 0.7, 0.75, 1.0)); // Silver
             draw_circle(mx + msize/2.0, my + msize/2.0, bolt_r * 0.5, Color::new(0.4, 0.4, 0.45, 1.0)); // Bolt head indent

             return;
             
        } else if is_bomb {
            // --- LAVA BLOCK RENDERING ---
            let time = get_time();
            
            // Pulsing Red/Orange base
            let pulse = (time * 5.0).sin() * 0.1;
            let lava_col = Color::new(
                color.r, 
                color.g + pulse as f32, 
                color.b, 
                1.0
            );
            
            draw_rectangle(cx, cy, current_size, current_size, lava_col);
            
            // "Cracks" or Heat spots (Yellow/White)
            let spots = bubble_seed; // Recycle seed
            if spots % 3 == 0 {
                 draw_circle(cx + current_size*0.3, cy + current_size*0.3, current_size*0.2, YELLOW);
            }
            if spots % 5 == 0 {
                 draw_circle(cx + current_size*0.7, cy + current_size*0.6, current_size*0.15, RED);
            }
            
        } else {
            // Standard Rounded Rect for all block-based rendering
            draw_rounded_rect(
                cx,
                cy,
                current_size,
                current_size,
                current_r,
                my_layer_color,
            );
        }

        // --- TRAPPED BUBBLES (Organic/Aerated look) ---
        if bubble_seed != 0 {
             let time = get_time() as f32;
             let bubble_layer_color = Color::new(1.0, 1.0, 1.0, 0.15); // Faint white
             
             // Calculate center for safe positioning
             let center_x = cx + current_size / 2.0;
             let center_y = cy + current_size / 2.0;
             let max_offset = current_size / 4.0; // Keep bubbles well within center
     
             // Bubble 1
             let b1_raw = (bubble_seed % 10) as f32;
             let b1_off_x = (b1_raw - 5.0) / 5.0 * max_offset;
             let b1_off_y = ((bubble_seed % 7) as f32 - 3.5) / 3.5 * max_offset;
             
             draw_circle(
                 center_x + b1_off_x + (time * 2.0).sin() * 1.0, 
                 center_y + b1_off_y + (time * 1.5).cos() * 1.0, 
                 3.0, 
                 bubble_layer_color
             );
     
             // Bubble 2
             let b2_off_x = -b1_off_x * 0.6; // Mirror and reduce
             let b2_off_y = -b1_off_y * 0.6;
             draw_circle(
                 center_x + b2_off_x + (time * 1.7).cos() * 1.0, 
                 center_y + b2_off_y + (time * 2.2).sin() * 1.0, 
                 2.0, 
                 bubble_layer_color
             );
     
             // Bubble 3 (Tiny)
             draw_circle(
                 center_x + b1_off_y * 0.5, 
                 center_y + b2_off_x * 0.5, 
                 1.5, 
                 bubble_layer_color
             );
        }

        // Connectors (The "Bridges")
        let bridge_overlap = 3.0;

        // Top Connector
        if let Some(_nc) = neighbors.top {
            let h = (cy - y) + bridge_overlap;
            draw_rectangle(cx, y, current_size, h, my_layer_color);
        }
        // Bottom Connector
        if let Some(_nc) = neighbors.bottom {
            let start_y = cy + current_size - bridge_overlap;
            let h = (y + size) - start_y;
            draw_rectangle(cx, start_y, current_size, h, my_layer_color);
        }
        // Left Connector
        if let Some(_nc) = neighbors.left {
            let w = (cx - x) + bridge_overlap;
            draw_rectangle(x, cy, w, current_size, my_layer_color);
        }
        // Right Connector
        if let Some(_nc) = neighbors.right {
            let start_x = cx + current_size - bridge_overlap;
            let w = (x + size) - start_x;
            draw_rectangle(start_x, cy, w, current_size, my_layer_color);
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

    // --- 4. Glossy Highlights (Wetness) ---
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
