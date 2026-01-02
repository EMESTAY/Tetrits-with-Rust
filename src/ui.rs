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
