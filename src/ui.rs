use macroquad::prelude::*;
use crate::constants::*;
use crate::rect_utils::draw_rounded_rect;
use crate::game::{Game, GameState};
use crate::bidule::Bidule;
use crate::draw::Connectivity;
use crate::draw::blocks::draw_jelly_block;
use crate::draw::effects::{draw_giant_jelly_circle, draw_giant_bomb_diamond};

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
    let border_thick = UI_BORDER_THICKNESS;

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

    // --- SPECIAL PREVIEW: GIANT JELLY CIRCLE ---
    if piece.kind == crate::bidule::BiduleType::Jelly {
        let cx = x + w / 2.0;
        let cy = y + h / 2.0;
        let radius = 45.0; // 1.5 blocks radius = 3 blocks diameter
        
        draw_giant_jelly_circle(cx, cy, radius, piece.color, false);
        return;
    }

    // --- SPECIAL PREVIEW: GIANT BOMB DIAMOND ---
    if piece.kind == crate::bidule::BiduleType::Bomb {
        let cx = x + w / 2.0;
        let cy = y + h / 2.0;
        let radius = 50.0; // Slightly larger for diamond points
        draw_giant_bomb_diamond(cx, cy, radius, piece.color, false);
        return;
    }



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

pub fn draw_play_scene(game: &Game) {
    // Layout Constants
    let board_w = GRID_WIDTH as f32 * BLOCK_SIZE;
    let board_h = GRID_HEIGHT as f32 * BLOCK_SIZE;
    let spacing = UI_SPACING;
    let side_panel_w = UI_SIDE_PANEL_WIDTH;

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
    let level_panel_h = UI_LEVEL_PANEL_HEIGHT;
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
        
        // Shadow
        draw_text_ex(&lvl_text, tx + 2.0, ty + 2.0, TextParams {
            font: Some(f),
            font_size: (60.0 * pulse_scale) as u16,
            color: COLOR_TEXT_SHADOW,
            ..Default::default()
        });
        // Text
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
        
        // Shadow
        draw_text_ex(&score_text, tx + 2.0, score_panel_y + 80.0 + 2.0, TextParams {
            font: Some(f),
            font_size: (50.0 * pulse_scale) as u16,
            color: COLOR_TEXT_SHADOW,
            ..Default::default()
        });
        // Text
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



    // --- 5. Active Relics / Bonuses (Icons below Score) ---
    // User request: "display a thumb in the background"
    let mut relic_y = score_panel_y + score_panel_h + 20.0;
    
    // Group bonuses by type to show counts (Stacks)
    let mut counts = std::collections::HashMap::new();
    for b in &game.active_bonuses {
        *counts.entry(b.kind).or_insert(0) += 1;
    }

    if let Some(f) = font_ref {
        for (kind, count) in counts {
            let (icon, color) = match kind {
                crate::bonuses::BonusType::TimeAnchor => ("⚓", GOLD),
                crate::bonuses::BonusType::GoldenPickaxe => ("⛏️", GOLD),
                crate::bonuses::BonusType::LifeInsurance => ("💖", PINK),
                crate::bonuses::BonusType::Chill => ("❄️", SKYBLUE),
                _ => continue, // Don't show icons for pieces that are instantaneous or handled otherwise
            };

            let text = if count > 1 {
                format!("{} x{}", icon, count)
            } else {
                icon.to_string()
            };
            
            // Draw Icon Row
            draw_text_ex(&text, hold_x, relic_y + 40.0, TextParams {
                font: Some(f),
                font_size: 40,
                color: color,
                ..Default::default()
            });
            
            relic_y += 50.0;
        }
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

    // Grid Lines (Scaled)
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
                    game.grid.cells[ny as usize][nx as usize].as_ref().and_then(|c| {
                        // Anvil Check: Don't connect to Anvils (return None if Anvil)
                        // Anvil Color: (0.3, 0.3, 0.35)
                        let is_anvil = (c.color.r - 0.3).abs() < 0.05 
                                    && (c.color.g - 0.3).abs() < 0.05 
                                    && (c.color.b - 0.35).abs() < 0.05;
                        if is_anvil {
                            None
                        } else {
                            Some(c.color)
                        }
                    })
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
                          all: &[crate::bidule::Point],
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
    let ghost = game.get_ghost_position();
    if game.current_piece.kind == crate::bidule::BiduleType::Jelly {
         // Ghost Circle
         let mut min_x = f32::MAX;
         let mut max_x = f32::MIN;
         let mut min_y = f32::MAX;
         let mut max_y = f32::MIN;
         
         for p in &game.current_piece.positions {
             let x = (ghost.x + p.x) as f32 * BLOCK_SIZE;
             let y = (ghost.y + p.y) as f32 * BLOCK_SIZE;
             if x < min_x { min_x = x; }
             if x + BLOCK_SIZE > max_x { max_x = x + BLOCK_SIZE; }
             if y < min_y { min_y = y; }
             if y + BLOCK_SIZE > max_y { max_y = y + BLOCK_SIZE; }
         }
         
         let cx = grid_x + (min_x + max_x) / 2.0;
         let cy = grid_y + (min_y + max_y) / 2.0;
         let radius = (max_x - min_x).max(max_y - min_y) / 2.0;
         
         let col = game.current_piece.color;
         // Draw layered ghost
         draw_giant_jelly_circle(cx, cy, radius, col, true);
         


    } else {
        // Standard Ghost Preview (Blocks)
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
    }

    // Draw Current Piece
    if game.current_piece.kind == crate::bidule::BiduleType::Jelly {
         // ... (Jelly Logic) ...
         // Calculate center of mass in pixels
         let mut min_x = f32::MAX;
         let mut max_x = f32::MIN;
         let mut min_y = f32::MAX;
         let mut max_y = f32::MIN;
         
         for p in &game.current_piece.positions {
             let x = (game.current_piece.pos.x + p.x) as f32 * BLOCK_SIZE;
             let y = (game.current_piece.pos.y + p.y) as f32 * BLOCK_SIZE;
             if x < min_x { min_x = x; }
             if x + BLOCK_SIZE > max_x { max_x = x + BLOCK_SIZE; }
             if y < min_y { min_y = y; }
             if y + BLOCK_SIZE > max_y { max_y = y + BLOCK_SIZE; }
         }
         
         let cx = grid_x + (min_x + max_x) / 2.0;
         let cy = grid_y + (min_y + max_y) / 2.0;
         let radius = (max_x - min_x).max(max_y - min_y) / 2.0;
         
         draw_giant_jelly_circle(cx, cy, radius, game.current_piece.color, false);
         
    } else if game.current_piece.kind == crate::bidule::BiduleType::Bomb {
         // --- SPECIAL RENDERING: GIANT BOMB DIAMOND ---
         
          let mut min_x = f32::MAX;

          let mut max_x = f32::MIN;
          let mut min_y = f32::MAX;
          let mut max_y = f32::MIN;
          
          for p in &game.current_piece.positions {
              let x = (game.current_piece.pos.x + p.x) as f32 * BLOCK_SIZE;
              let y = (game.current_piece.pos.y + p.y) as f32 * BLOCK_SIZE;
              if x < min_x { min_x = x; }
              if x + BLOCK_SIZE > max_x { max_x = x + BLOCK_SIZE; }
              if y < min_y { min_y = y; }
              if y + BLOCK_SIZE > max_y { max_y = y + BLOCK_SIZE; }
          }
          
          let cx = grid_x + (min_x + max_x) / 2.0;
          let cy = grid_y + (min_y + max_y) / 2.0;
          let radius = 55.0; // Tuning size
          
          
          draw_giant_bomb_diamond(cx, cy, radius, game.current_piece.color, false);
         


    } else {
        // Standard Rendering for other pieces (including Ghost)
        for (i, p) in game.current_piece.positions.iter().enumerate() {
            let x = game.current_piece.pos.x + p.x;
            let y = game.current_piece.pos.y + p.y;
            if y >= 0 {
                let neighbors =
                    get_piece_conn(*p, &game.current_piece.positions, game.current_piece.color);
                
                // "Lock Shake": Jitter if lock timer active
                let mut jitter_x = 0.0;
                if game.lock_timer > 0.0 {
                     jitter_x = (get_time() * 50.0).sin() as f32 * 2.0;
                }
    
                draw_jelly_block(
                    grid_x + x as f32 * BLOCK_SIZE + jitter_x,
                    grid_y + y as f32 * BLOCK_SIZE,
                    BLOCK_SIZE,
                    game.current_piece.color,
                    neighbors,
                    false,
                    game.current_piece.seeds[i],
                );
            }
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
}
