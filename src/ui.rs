use macroquad::prelude::*;
use crate::constants::*;
use crate::rect_utils::draw_rounded_rect;
use crate::game::Game;

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

    // --- 6. Pulsing Rim Highlight ---
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

pub fn draw_start_screen(game: &Game) {
    let screen_w = screen_width();
    let screen_h = screen_height();
    let cx = screen_w / 2.0;

    // Background Overlay
    draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));

    // --- Title ---
    let font_ref = game.font.as_ref();
    let title_text = "RUST TETRIS";
    let title_size = 100.0;
    
    // Pulse animation for title
    let time = get_time();
    let pulse = (time * 2.0).sin() as f32 * 0.05 + 1.0;
    
    // Draw Title (Centered)
    if let Some(f) = font_ref {
        let dim = measure_text(title_text, Some(f), (title_size * pulse) as u16, 1.0);
        let tx = cx - dim.width / 2.0;
        let ty = screen_h * 0.3;
        
        // Shadow/Glow
        draw_text_ex(title_text, tx + 5.0, ty + 5.0, TextParams {
            font: Some(f),
            font_size: (title_size * pulse) as u16,
            color: Color::new(0.0, 0.0, 1.0, 0.5),
            ..Default::default()
        });
        
        // Main Text (Gradient-ish effect by drawing twice?)
        draw_text_ex(title_text, tx, ty, TextParams {
            font: Some(f),
            font_size: (title_size * pulse) as u16,
            color: WHITE,
            ..Default::default()
        });
    }

    // --- Menu Options ---
    let options = vec![
        "START GAME", 
        if game.is_music_playing { "OPTIONS: MUSIC ON" } else { "OPTIONS: MUSIC OFF" },
        "EXIT"
    ];
    
    let start_y = screen_h * 0.5;
    let spacing = 60.0;

    for (i, opt) in options.iter().enumerate() {
        let is_selected = i == game.menu_selection;
        
        let color = if is_selected { GOLD } else { LIGHTGRAY };
        let size = if is_selected { 50 } else { 40 };
        
        let y = start_y + i as f32 * spacing;

        if let Some(f) = font_ref {
            let dim = measure_text(opt, Some(f), size, 1.0);
            let x = cx - dim.width / 2.0;

            // Arrow for selection
            if is_selected {
                let arrow_off = (time * 10.0).sin() as f32 * 5.0;
                draw_text_ex(">", x - 30.0 + arrow_off, y, TextParams {
                    font: Some(f),
                    font_size: size,
                    color: GOLD,
                    ..Default::default()
                });
            }

            draw_text_ex(opt, x, y, TextParams {
                font: Some(f),
                font_size: size,
                color,
                ..Default::default()
            });
        }
    }

    // Footer
    let footer = "Use ARROW KEYS and ENTER";
    draw_text(footer, 20.0, screen_h - 20.0, 20.0, GRAY);
}

pub fn draw_game_over(game: &Game) {
    let screen_w = screen_width();
    let screen_h = screen_height();
    
    draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.85));

    let text = "GAME OVER";
    let font_size = 100.0;
    
    if let Some(f) = game.font.as_ref() {
        let dim = measure_text(text, Some(f), font_size as u16, 1.0);
        let x = (screen_w - dim.width) / 2.0;
        let y = (screen_h - dim.height) / 2.0;
        
        draw_text_ex(text, x, y, TextParams {
            font: Some(f),
            font_size: font_size as u16,
            color: RED,
            ..Default::default()
        });
        
        let sub = format!("Score: {}", game.score);
        let dim_s = measure_text(&sub, Some(f), 40, 1.0);
        
        draw_text_ex(&sub, (screen_w - dim_s.width) / 2.0, y + 80.0, TextParams {
            font: Some(f),
            font_size: 40,
            color: WHITE,
            ..Default::default()
        });

        let restart = "Press R to Restart";
        let dim_r = measure_text(restart, Some(f), 30, 1.0);
        draw_text_ex(restart, (screen_w - dim_r.width) / 2.0, y + 140.0, TextParams {
            font: Some(f),
            font_size: 30,
            color: GRAY,
            ..Default::default()
        });
    }
}

/// Helper for basic styled text
pub fn draw_text_styled(text: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(text, x, y + 2.0, size, BLACK); // Shadow
    draw_text(text, x, y, size, color);
}

pub fn draw_bonus_selection(game: &Game) {
    // Darken background
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, 0.8),
    );

    let screen_w = screen_width();
    let screen_h = screen_height();
    let cx = screen_w / 2.0;

    // Title
    if let Some(f) = game.font.as_ref() {
        let title = "LEVEL UP! CHOOSE A BONUS";
        let dim = measure_text(title, Some(f), 60, 1.0);
        draw_text_styled(
            title,
            cx - dim.width / 2.0,
            screen_h * 0.15,
            60.0,
            GOLD,
        );
    }

    // Grid Layout Constants
    let card_w = 250.0;
    let card_h = 350.0;
    let gap = 30.0;
    let grid_cols = 3;
    let font_ref = game.font.as_ref();
    let time = get_time();
    
    // Calculate Grid Dimensions
    let total_bonuses = game.bonus_options.len();
    let total_rows = (total_bonuses + grid_cols - 1) / grid_cols;
    
    let grid_w = (grid_cols as f32).min(total_bonuses as f32) * card_w + ((grid_cols as f32).min(total_bonuses as f32) - 1.0).max(0.0) * gap;
    let grid_h = (total_rows as f32) * card_h + ((total_rows as f32) - 1.0).max(0.0) * gap;

    let start_x = (screen_w - grid_w) / 2.0;
    let start_y = (screen_h - grid_h) / 2.0 + 50.0; // Offset for title

    for (i, bonus) in game.bonus_options.iter().enumerate() {
        let is_selected = i == game.bonus_selection_idx;
        
        let col = i % grid_cols;
        let row = i / grid_cols;

        // Hover/Selection effect
        let scale = if is_selected { 
            1.05 + (time * 5.0).sin() as f32 * 0.02 
        } else { 
            1.0 
        };
        
        let cw = card_w * scale;
        let ch = card_h * scale;
        
        // Position Card in Grid
        let base_x = start_x + col as f32 * (card_w + gap);
        let base_y = start_y + row as f32 * (card_h + gap);
        
        // Center scaled card on its slot
        let x = base_x - (cw - card_w) / 2.0;
        let y = base_y - (ch - card_h) / 2.0;

        // Card Background
        let card_color = if is_selected {
            GRAY
        } else {
            Color::new(0.2, 0.2, 0.2, 1.0)
        };
        
        // Border based on Rarity
        let mut border_color = bonus.color;
        let mut border_thick = 1.0;
        
        match bonus.rarity {
            crate::bonuses::Rarity::Rare => {
                border_color = GOLD;
                border_thick = 3.0;
            },
            crate::bonuses::Rarity::Legendary => {
                // Rainbow pulse for Legendary
                let t = time * 3.0;
                border_color = Color::new(
                    (t.sin() * 0.5 + 0.5) as f32,
                    ((t + 2.0).sin() * 0.5 + 0.5) as f32,
                    ((t + 4.0).sin() * 0.5 + 0.5) as f32,
                    1.0
                );
                border_thick = 5.0;
            },
            _ => { if is_selected { border_thick = 2.0; } }
        }
        
        // Draw Border
        draw_rectangle(x - border_thick, y - border_thick, cw + border_thick*2.0, ch + border_thick*2.0, border_color);
        draw_rectangle(x, y, cw, ch, card_color);

        // Icon
        if let Some(f) = font_ref {
            // Rarity Label
            let (rarity_txt, r_col) = match bonus.rarity {
                crate::bonuses::Rarity::Legendary => ("LEGENDARY", MAGENTA),
                crate::bonuses::Rarity::Rare => ("RARE", GOLD),
                _ => ("", WHITE),
            };
            
            if !rarity_txt.is_empty() {
                let rdim = measure_text(rarity_txt, Some(f), 20, 1.0);
                draw_text_ex(rarity_txt, x + (cw - rdim.width)/2.0, y + 30.0, TextParams {
                    font: Some(f),
                    font_size: 20,
                    color: r_col,
                    ..Default::default()
                });
            }

            // Draw Icon Centered
            let icon_size = 80.0;
            let idim = measure_text(bonus.icon, Some(f), icon_size as u16, 1.0);
            draw_text_ex(bonus.icon, x + (cw - idim.width) / 2.0, y + 100.0, TextParams {
                font: Some(f),
                font_size: icon_size as u16,
                color: WHITE,
                ..Default::default()
            });

            // Name
            let name_size = 30.0;
            let ndim = measure_text(bonus.name, Some(f), name_size as u16, 1.0);
            draw_text_styled(
                bonus.name, 
                x + (cw - ndim.width) / 2.0, 
                y + 160.0, 
                name_size, 
                bonus.color
            );

            // Description (Wrapped manually)
            let desc_size = 20.0;
            let words: Vec<&str> = bonus.description.split_whitespace().collect();
            let mut line = String::new();
            let mut ly = y + 220.0;
            
            for word in words {
                let test_line = if line.is_empty() {
                    word.to_string()
                } else {
                    format!("{} {}", line, word)
                };
                
                let dim = measure_text(&test_line, Some(f), desc_size as u16, 1.0);
                if dim.width > cw - 20.0 {
                    // Draw current line
                    let ldim = measure_text(&line, Some(f), desc_size as u16, 1.0);
                    draw_text_ex(&line, x + (cw - ldim.width) / 2.0, ly, TextParams {
                        font: Some(f),
                        font_size: desc_size as u16,
                        color: LIGHTGRAY,
                        ..Default::default()
                    });
                    line = word.to_string();
                    ly += 25.0;
                } else {
                    line = test_line;
                }
            }
            if !line.is_empty() {
                 let ldim = measure_text(&line, Some(f), desc_size as u16, 1.0);
                 draw_text_ex(&line, x + (cw - ldim.width) / 2.0, ly, TextParams {
                    font: Some(f),
                    font_size: desc_size as u16,
                    color: LIGHTGRAY,
                    ..Default::default()
                });
            }
        }
    }
    
    // Instructions
    let instr = "Select with Arrows, Confirm with Enter";
    if let Some(f) = game.font.as_ref() {
        let dim = measure_text(instr, Some(f), 30, 1.0);
         draw_text_styled(
            instr,
            (screen_w - dim.width) / 2.0,
            screen_h - 50.0,
            30.0,
            WHITE
        );
    }
}
