
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

    // Draw 3 Cards
    let card_w = 250.0;
    let card_h = 350.0;
    let gap = 30.0;
    let total_w = 3.0 * card_w + 2.0 * gap;
    let start_x = (screen_w - total_w) / 2.0;
    let start_y = (screen_h - card_h) / 2.0;
    let font_ref = game.font.as_ref();
    let time = get_time();

    for (i, bonus) in game.bonus_options.iter().enumerate() {
        let is_selected = i == game.bonus_selection_idx;
        
        // Hover/Selection effect
        let scale = if is_selected { 
            1.05 + (time * 5.0).sin() as f32 * 0.02 
        } else { 
            1.0 
        };
        
        let x = start_x + i as f32 * (card_w + gap);
        let y = start_y - if is_selected { 20.0 } else { 0.0 };

        // Card Background
        let card_color = if is_selected {
            GRAY
        } else {
            Color::new(0.2, 0.2, 0.2, 1.0)
        };
        
        // Border
        let border_color = if is_selected { bonus.color } else { DARKGRAY };
        
        draw_rectangle(x, y, card_w, card_h, border_color);
        draw_rectangle(x + 5.0, y + 5.0, card_w - 10.0, card_h - 10.0, card_color);

        // Icon
        if let Some(f) = font_ref {
            // Draw Icon Centered
            let icon_size = 80.0;
            let idim = measure_text(bonus.icon, Some(f), icon_size as u16, 1.0);
            draw_text_ex(bonus.icon, x + (card_w - idim.width) / 2.0, y + 100.0, TextParams {
                font: Some(f),
                font_size: icon_size as u16,
                color: WHITE,
                ..Default::default()
            });

            // Name
            let name_size = 30.0;
            // Split name if too long? For now assume it fits or text wrap
            // Just simple text
            let ndim = measure_text(bonus.name, Some(f), name_size as u16, 1.0);
            draw_text_styled(
                bonus.name, 
                x + (card_w - ndim.width) / 2.0, 
                y + 160.0, 
                name_size, 
                bonus.color
            );

            // Description (Wrapped manually approx)
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
                if dim.width > card_w - 20.0 {
                    // Draw current line
                    let ldim = measure_text(&line, Some(f), desc_size as u16, 1.0);
                    draw_text_ex(&line, x + (card_w - ldim.width) / 2.0, ly, TextParams {
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
                 draw_text_ex(&line, x + (card_w - ldim.width) / 2.0, ly, TextParams {
                    font: Some(f),
                    font_size: desc_size as u16,
                    color: LIGHTGRAY,
                    ..Default::default()
                });
            }
        }
    }
    
    // Instructions
    draw_text_styled(
        "Select with Arrows, Confirm with Enter",
        cx - 200.0,
        screen_h - 50.0,
        30.0,
        WHITE
    );
}
