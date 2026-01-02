use crate::bidule::Bidule;
use crate::constants::*;
use crate::rect_utils::draw_rounded_rect;
use crate::ui::*;

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
            0,
        );
    }
}

// draw_rounded_rect and _draw_mesh_gradient_rect moved to rect_utils.rs

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
    bubble_seed: usize,
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

// draw_panel moved to ui.rs

use crate::game::{Game, GameState};

/// Main drawing function for the game
pub fn draw_game(game: &Game) {
    // 1. Background (Nature Theme)
    game.background.draw();

    // 0. Game States (Start / Game Over / Playing)
    match game.state {
        GameState::Start => {
            crate::ui::draw_start_screen(game);
        }
        GameState::Playing | GameState::GameOver | GameState::ChooseBonus => {
            draw_play_scene(game);
            
            if game.state == GameState::ChooseBonus {
                crate::ui::draw_bonus_selection(game);
            }
            if game.state == GameState::GameOver {
                crate::ui::draw_game_over(game);
            }
        }
    }
}

fn draw_play_scene(game: &Game) {
    // Layout Constants
    let board_w = GRID_WIDTH as f32 * BLOCK_SIZE;
    let board_h = GRID_HEIGHT as f32 * BLOCK_SIZE;
    let spacing = 80.0;
    let side_panel_w = 260.0;

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

    // --- 1. Level Panel (Top Left) ---
    // User requested "level window a bit smaller on top"
    let level_panel_h = 100.0;
    let level_color = Color::new(0.0, 0.5, 0.9, 1.0);
    
    draw_panel(
        next_x,
        stats_y,
        side_panel_w,
        level_panel_h,
        Some("LEVEL"),
        font_ref,
        level_color,
    );

    if let Some(f) = font_ref {
        let pulse_scale = 1.0 + (game.ui_pulse * 0.2);
        let lvl_text = format!("{}", game.level);
        
        // Center the level number big
        let dim = measure_text(&lvl_text, Some(f), 60, 1.0);
        let tx = next_x + (side_panel_w - dim.width * pulse_scale) / 2.0;
        let ty = stats_y + 70.0;
        
        draw_text_ex(&lvl_text, tx, ty, TextParams {
            font: Some(f),
            font_size: (60.0 * pulse_scale) as u16,
            color: WHITE,
            ..Default::default()
        });
    }

    // --- 2. Next Piece Panel (Left, below Level) ---
    // User requested "winsows need to be spaced on from an other"
    let spacing_vertical = 100.0; // Increased from 50 to 100
    let next_panel_y = stats_y + level_panel_h + spacing_vertical;
    let next_panel_h = 250.0; 
    
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

    // --- 3. Hold Panel (Top Right) ---
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

    // --- 4. Score Panel (Right, below Hold) ---
    // User requested "score put it on the right under hold"
    let score_panel_y = stats_y + hold_panel_h + spacing_vertical;
    let score_panel_h = 150.0; 
    
    draw_panel(
        hold_x,
        score_panel_y,
        side_panel_w,
        score_panel_h,
        Some("SCORE"),
        font_ref,
        GOLD,
    );

    if let Some(f) = font_ref {
        let pulse_scale = 1.0 + (game.ui_pulse * 0.2);
        let score_text = format!("{}", game.score);
        let lines_text = format!("LINES: {}", game.lines_cleared_total);

        // Score Big
        let dim = measure_text(&score_text, Some(f), 50, 1.0);
        let tx = hold_x + (side_panel_w - dim.width * pulse_scale) / 2.0;
        
        draw_text_ex(&score_text, tx, score_panel_y + 80.0, TextParams {
            font: Some(f),
            font_size: (50.0 * pulse_scale) as u16,
            color: WHITE,
            ..Default::default()
        });

        // Lines Small below
        let dim_l = measure_text(&lines_text, Some(f), 30, 1.0);
        let lx = hold_x + (side_panel_w - dim_l.width) / 2.0;
         draw_text_ex(&lines_text, lx, score_panel_y + 120.0, TextParams {
            font: Some(f),
            font_size: 30,
            color: Color::new(0.8, 0.8, 0.8, 1.0),
            ..Default::default()
        });
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
            if let Some(cell) = &game.grid.cells[y][x] {
                let check_neighbor = |nx: i32, ny: i32| -> Option<Color> {
                    if nx < 0 || nx >= GRID_WIDTH as i32 || ny < 0 || ny >= GRID_HEIGHT as i32 {
                        return None;
                    }
                    game.grid.cells[ny as usize][nx as usize].as_ref().map(|c| c.color)
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
                    cell.color,
                    neighbors,
                    false,
                    cell.bubble_seed,
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
                0,
            );
        }
    }

    // Draw Current Piece
    for (i, p) in game.current_piece.positions.iter().enumerate() {
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
                game.current_piece.seeds[i],
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
        crate::ui::draw_game_over(game);
    }
}

// draw_start_screen removed

// draw_game_over removed

// draw_utils removed
