use crate::background::NatureBackground;
use crate::bidule::{Bidule, BiduleType};
use crate::bonuses::{ActiveBonus, Bonus};
use crate::constants::*;
use crate::effects::{ComicEffect, Particle, ParticleType};
use crate::grid::Grid;
use crate::sound_effects::AudioSystem;
use macroquad::prelude::*;
use macroquad::text::Font;

#[derive(PartialEq)]
pub enum GameState {
    Start,
    Playing,
    ChooseBonus,
    GameOver,
}

pub struct Game {
    pub grid: Grid,
    pub current_piece: Bidule,
    pub next_pieces: Vec<Bidule>,
    pub hold_piece: Option<Bidule>,
    pub can_hold: bool,
    pub score: i32,
    pub last_fall_time: f64,
    // Visuals
    pub effects: Vec<ComicEffect>,
    pub particles: Vec<Particle>,
    pub background: NatureBackground,
    pub font: Option<Font>,
    pub audio: AudioSystem,
    pub is_music_playing: bool,
    pub state: GameState,
    pub level: i32,
    pub lines_cleared_total: i32,
    pub screen_shake: f32,
    pub ui_pulse: f32, // Timer for UI juice
    pub menu_selection: usize, // 0: Start, 1: Options, 2: Exit
    bag: Vec<BiduleType>,
    
    // Bonus System
    pub bonus_options: Vec<Bonus>,
    pub bonus_selection_idx: usize,
    pub active_bonuses: Vec<ActiveBonus>,
}

impl Game {
    pub fn new(font: Option<Font>, audio: AudioSystem) -> Self {
        let mut game = Self {
            grid: Grid::new(),
            current_piece: Bidule::new(BiduleType::I), // Placeholder
            next_pieces: Vec::new(),
            hold_piece: None,
            can_hold: true,
            score: 0,
            last_fall_time: get_time(),
            effects: Vec::new(),
            particles: Vec::new(),
            background: NatureBackground::new(),
            font,
            audio,
            bag: Vec::new(),
            is_music_playing: true,
            state: GameState::Start,
            level: 1,
            lines_cleared_total: 0,
            screen_shake: 0.0,
            ui_pulse: 0.0,
            menu_selection: 0,
            
            // Bonus System
            bonus_options: Vec::new(),
            bonus_selection_idx: 0,
            active_bonuses: Vec::new(),
        };

        game.fill_bag();
        game.current_piece = game.get_next_piece();
        for _ in 0..3 {
            let p = game.get_next_piece();
            game.next_pieces.push(p);
        }

        game
    }

    fn fill_bag(&mut self) {
        let mut types = vec![
            BiduleType::I,
            BiduleType::O,
            BiduleType::T,
            BiduleType::S,
            BiduleType::Z,
            BiduleType::J,
            BiduleType::L,
        ];
        fastrand::shuffle(&mut types);
        self.bag.extend(types);
    }

    fn get_next_piece(&mut self) -> Bidule {
        if self.bag.is_empty() {
            self.fill_bag();
        }
        Bidule::new(self.bag.pop().unwrap())
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        // Shake decay
        if self.screen_shake > 0.0 {
            self.screen_shake -= dt * 10.0;
            if self.screen_shake < 0.0 {
                self.screen_shake = 0.0;
            }
        }

        // UI Pulse decay
        if self.ui_pulse > 0.0 {
            self.ui_pulse -= dt;
            if self.ui_pulse < 0.0 {
                self.ui_pulse = 0.0;
            }
        }

        match self.state {
            GameState::Start => {
                if is_key_pressed(KeyCode::Down) {
                    self.menu_selection = (self.menu_selection + 1) % 3;
                    self.audio.play_hold(); // reusing a bloop sound
                }
                if is_key_pressed(KeyCode::Up) {
                    if self.menu_selection == 0 {
                        self.menu_selection = 2;
                    } else {
                        self.menu_selection -= 1;
                    }
                    self.audio.play_hold();
                }

                if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                    match self.menu_selection {
                        0 => {
                            self.state = GameState::Playing;
                            self.audio.play_level_up(); // Confirm sound
                        }
                        1 => {
                            // Option: Toggle Music
                            self.is_music_playing = !self.is_music_playing;
                            self.audio.toggle_music(self.is_music_playing);
                        }
                        2 => {
                            // Exit
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
            }
            GameState::ChooseBonus => {
                 if is_key_pressed(KeyCode::Right) {
                    self.bonus_selection_idx = (self.bonus_selection_idx + 1) % self.bonus_options.len();
                    self.audio.play_hold();
                }
                if is_key_pressed(KeyCode::Left) {
                    if self.bonus_selection_idx == 0 {
                        self.bonus_selection_idx = self.bonus_options.len() - 1;
                    } else {
                        self.bonus_selection_idx -= 1;
                    }
                    self.audio.play_hold();
                }
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                    // Activate Bonus
                    if let Some(bonus) = self.bonus_options.get(self.bonus_selection_idx) {
                        self.activate_bonus(bonus.clone());
                    }
                    self.state = GameState::Playing;
                    self.audio.play_level_up();
                }
            }
            GameState::Playing => {
                let time = get_time();

                self.handle_input();

                let is_soft_drop = is_key_down(KeyCode::Down);
                let speed = if is_soft_drop {
                    0.05
                } else {
                    // Level-based speed
                    let base_speed = (0.5 * (0.9f64.powi(self.level - 1))).max(0.05);
                    
                    // CHILL Bonus: 50% slower
                    let mut speed_mod = 1.0;
                    if self.active_bonuses.iter().any(|b| b.kind == crate::bonuses::BonusType::Chill) {
                        speed_mod *= 1.5;
                    }
                    
                    // TIME ANCHOR: 10% slower per stack
                    let anchors = self.active_bonuses.iter().filter(|b| b.kind == crate::bonuses::BonusType::TimeAnchor).count();
                    if anchors > 0 {
                        speed_mod *= 1.0 + (0.1 * anchors as f64);
                    }
                    
                    base_speed * speed_mod
                };

                if time - self.last_fall_time > speed {
                    self.current_piece.pos.y += 1;
                    if self.grid.is_collision(&self.current_piece) {
                        self.current_piece.pos.y -= 1;
                        self.lock_and_spawn();
                    }
                    self.last_fall_time = time;
                }
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::R) {
                    let font = self.font.take();
                    let audio = self.audio.clone();
                    *self = Game::new(font, audio);
                    self.state = GameState::Playing; // Start immediately on reset
                }
            }
        }

        self.effects.retain_mut(|e| e.update());
        self.particles.retain_mut(|p| p.update());
        
        // Update active bonuses
        let dt = get_frame_time();
        self.active_bonuses.retain_mut(|b| {
            b.timer -= dt;
            b.timer > 0.0
        });

        self.background.update();
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Left) {
            self.current_piece.pos.x -= 1;
            if self.grid.is_collision(&self.current_piece) {
                self.current_piece.pos.x += 1;
            }
        }
        if is_key_pressed(KeyCode::Right) {
            self.current_piece.pos.x += 1;
            if self.grid.is_collision(&self.current_piece) {
                self.current_piece.pos.x -= 1;
            }
        }

        if is_key_pressed(KeyCode::Up) {
            let mut rotated = self.current_piece.clone();
            rotated.rotate();
            if !self.grid.is_collision(&rotated) {
                self.current_piece = rotated;
            } else {
                // Wall kick (simple)
                rotated.pos.x += 1;
                if !self.grid.is_collision(&rotated) {
                    self.current_piece = rotated;
                } else {
                    rotated.pos.x -= 2;
                    if !self.grid.is_collision(&rotated) {
                        self.current_piece = rotated;
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::C) {
            if self.can_hold {
                self.audio.play_hold();
                if let Some(mut held) = self.hold_piece.clone() {
                    let mut current = self.current_piece.clone();
                    // Reset position
                    current.pos = crate::bidule::Point { x: 3, y: 0 };
                    held.pos = crate::bidule::Point { x: 3, y: 0 };

                    self.hold_piece = Some(Bidule::new(current.kind)); // Reset orientation
                    self.current_piece = Bidule::new(held.kind);
                } else {
                    let current = self.current_piece.clone();
                    self.hold_piece = Some(Bidule::new(current.kind));

                    // Pop next piece
                    self.current_piece = self.next_pieces.remove(0);
                    let p = self.get_next_piece();
                    self.next_pieces.push(p);
                }
                self.can_hold = false;
            }
        }

        if is_key_pressed(KeyCode::Space) {
            self.current_piece.pos = self.get_ghost_position();
            self.lock_and_spawn();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();

            // Music Button Geometry (Bottom Left Icon)
            let btn_size = 50.0;
            let btn_x = 50.0;
            let btn_y = screen_height() - 100.0;

            if mx >= btn_x && mx <= btn_x + btn_size && my >= btn_y && my <= btn_y + btn_size {
                self.is_music_playing = !self.is_music_playing;
                self.audio.toggle_music(self.is_music_playing);
            }
        }
    }

    pub fn get_ghost_position(&self) -> crate::bidule::Point {
        let mut ghost = self.current_piece.clone();
        while !self.grid.is_collision(&ghost) {
            ghost.pos.y += 1;
        }
        ghost.pos.y -= 1;
        ghost.pos
    }

    fn lock_and_spawn(&mut self) {
        // Play Merge/Lock Sounds based on neighbors
        let mut same_color = false;
        let mut diff_color = false;
        let p_color = self.current_piece.color;

        for p in self.current_piece.positions.iter() {
            let x = self.current_piece.pos.x + p.x;
            let y = self.current_piece.pos.y + p.y;

            let neighbors = [(0, -1), (0, 1), (-1, 0), (1, 0)];
            for (dx, dy) in neighbors {
                let nx = x + dx;
                let ny = y + dy;

                if nx >= 0 && nx < GRID_WIDTH as i32 && ny >= 0 && ny < GRID_HEIGHT as i32 {
                    if let Some(cell) = &self.grid.cells[ny as usize][nx as usize] {
                        if cell.color == p_color {
                            same_color = true;
                        } else {
                            diff_color = true;
                        }
                    }
                }
            }
        }
        self.audio.play_land(same_color, diff_color);

        self.grid.lock_piece(&self.current_piece);

        // --- ONE-TIME BONUSES (Bomb / Laser) ---
        let mut bonuses_to_remove = Vec::new();
        for (i, bonus) in self.active_bonuses.iter().enumerate() {
            match bonus.kind {
                crate::bonuses::BonusType::Bomb => {
                    // Explode 3x3 around each block of the locked piece
                    // Or just around the center? Let's do around each block for massive destruction, or just one big boom?
                    // Let's do: For each block in the piece, explode radius 1
                    for p in self.current_piece.positions.iter() {
                        let cx = self.current_piece.pos.x + p.x;
                        let cy = self.current_piece.pos.y + p.y;
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                let nx = cx + dx;
                                let ny = cy + dy;
                                if nx >= 0 && nx < GRID_WIDTH as i32 && ny >= 0 && ny < GRID_HEIGHT as i32 {
                                    self.grid.cells[ny as usize][nx as usize] = None;
                                    // Visual?
                                    for _ in 0..5 {
                                        self.particles.push(Particle::new(
                                            (nx as f32 * BLOCK_SIZE) + BLOCK_SIZE/2.0, 
                                            (ny as f32 * BLOCK_SIZE) + BLOCK_SIZE/2.0, 
                                            RED, 
                                            ParticleType::Explosion
                                        ));
                                    }
                                }
                            }
                        }
                    }
                    self.screen_shake = 30.0;
                    self.audio.play_tetris(); // Boom sound replacement?
                    bonuses_to_remove.push(i);
                }
                crate::bonuses::BonusType::VerticalLaser => {
                    // Clear columns occupied by the piece
                    let mut cols = std::collections::HashSet::new();
                    for p in self.current_piece.positions.iter() {
                        cols.insert(self.current_piece.pos.x + p.x);
                    }
                    for c in cols {
                         if c >= 0 && c < GRID_WIDTH as i32 {
                            for y in 0..GRID_HEIGHT {
                                self.grid.cells[y][c as usize] = None;
                                // Sparks along the beam
                                if fastrand::f32() < 0.3 {
                                    self.particles.push(Particle::new(
                                        (c as f32 * BLOCK_SIZE) + BLOCK_SIZE/2.0, 
                                        (y as f32 * BLOCK_SIZE) + BLOCK_SIZE/2.0, 
                                        YELLOW, 
                                        ParticleType::Spark
                                    ));
                                }
                            }
                         }
                    }
                    self.screen_shake = 10.0;
                    bonuses_to_remove.push(i);
                }
                crate::bonuses::BonusType::Drill => {
                    // Same as laser basically in this implementation?
                    // Or maybe it clears *below* the piece?
                    // Let's implement Drill as clearing the columns BELOW the piece positions
                    for p in self.current_piece.positions.iter() {
                        let cx = self.current_piece.pos.x + p.x;
                        let cy = self.current_piece.pos.y + p.y;
                        if cx >= 0 && cx < GRID_WIDTH as i32 {
                             for y in cy..GRID_HEIGHT as i32 {
                                 if y >= 0 {
                                     self.grid.cells[y as usize][cx as usize] = None;
                                 }
                             }
                        }
                    }
                    bonuses_to_remove.push(i);
                }
                crate::bonuses::BonusType::VolatileGrid => {
                     // 10% chance to explode 3x3
                     if fastrand::f32() < 0.10 {
                        // Explosion logic (copied from Bomb but maybe smaller?)
                        // Reuse Bomb logic 3x3
                        for p in self.current_piece.positions.iter() {
                            let cx = self.current_piece.pos.x + p.x;
                            let cy = self.current_piece.pos.y + p.y;
                            for dy in -1..=1 {
                                for dx in -1..=1 {
                                    let nx = cx + dx;
                                    let ny = cy + dy;
                                    if nx >= 0 && nx < GRID_WIDTH as i32 && ny >= 0 && ny < GRID_HEIGHT as i32 {
                                        self.grid.cells[ny as usize][nx as usize] = None;
                                        // Explosion particles
                                        for _ in 0..3 {
                                           self.particles.push(Particle::new(
                                                (nx as f32 * BLOCK_SIZE) + BLOCK_SIZE/2.0, 
                                                (ny as f32 * BLOCK_SIZE) + BLOCK_SIZE/2.0, 
                                                ORANGE, 
                                                ParticleType::Explosion
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                        self.effects.push(ComicEffect::new("BOOM!".to_string(), 
                            self.current_piece.pos.x as f32 * BLOCK_SIZE, 
                            self.current_piece.pos.y as f32 * BLOCK_SIZE, RED));
                        self.screen_shake = 20.0;
                     }
                }
                _ => {}
            }
        }
        // Remove used bonuses (reverse order to keep indices valid? No, active_bonuses logic needed)
        // efficient removal:
        for idx in bonuses_to_remove.iter().rev() {
             self.active_bonuses.remove(*idx);
        }

        let cleared_rows = self.grid.clear_lines(); // Now returns Vec<usize>
        let cleared_count = cleared_rows.len() as i32;

        if cleared_count > 0 {
            self.lines_cleared_total += cleared_count;
            self.ui_pulse = 0.5;

            // Level up every 10 lines
            let new_level = (self.lines_cleared_total / 10) + 1;
            if new_level > self.level {
                self.level = new_level;
                self.audio.play_level_up();
                self.effects.push(ComicEffect::new(
                    "LEVEL UP!".to_string(),
                    screen_width() / 2.0,
                    screen_height() / 2.0,
                    GOLD,
                ));
                
                // TRIGGER BONUS SELECTION
                self.state = GameState::ChooseBonus;
                self.bonus_options = crate::bonuses::Bonus::get_random_set(3);
                self.bonus_selection_idx = 1; // Center default
            }

            if cleared_count == 4 {
                self.audio.play_tetris();
                self.screen_shake = 15.0;
            } else {
                self.screen_shake = 5.0 * cleared_count as f32;
            }

            let text = match cleared_count {
                1 => "ZAP!",
                2 => "POW!",
                3 => "BAM!",
                4 => "KABOOM!",
                _ => "!",
            };
            
            // Effect text center of action
           let center_y = if !cleared_rows.is_empty() {
                cleared_rows[0] as f32 * BLOCK_SIZE
            } else {
                self.current_piece.pos.y as f32 * BLOCK_SIZE
            };

            self.effects.push(ComicEffect::new(
                text.to_string(),
                (GRID_WIDTH as f32 * BLOCK_SIZE) / 2.0 + 100.0, // Center of grid roughly (offset for UI)
                 center_y + 100.0,
                RED,
            ));

            // --- TIERED PARTICLE SPAWNING ---
            use crate::effects::ParticleType;
            
            for &row_y in &cleared_rows {
                let py = row_y as f32 * BLOCK_SIZE + BLOCK_SIZE / 2.0;
                
                // Spawn across the width of the row
                for x in 0..GRID_WIDTH {
                    let px = x as f32 * BLOCK_SIZE + BLOCK_SIZE / 2.0;
                    
                    // Base Colors
                    let base_color = match cleared_count {
                        4 => Color::new(fastrand::f32(), fastrand::f32(), fastrand::f32(), 1.0), // Rainbow
                        3 => Color::new(1.0, 0.4 + fastrand::f32() * 0.4, 0.0, 1.0), // Orange/Red
                        2 => Color::new(0.2, 1.0, 0.2, 1.0), // Green/Toxic
                        _ => Color::new(0.4, 0.6, 1.0, 1.0), // Blue/Water
                    };

                    match cleared_count {
                        1 => {
                            // Tier 1: Small Splash
                            if fastrand::f32() < 0.3 {
                                self.particles.push(Particle::new(px, py, base_color, ParticleType::Droplet));
                            }
                        },
                        2 => {
                            // Tier 2: Splash + Bubbles
                            if fastrand::f32() < 0.5 {
                                self.particles.push(Particle::new(px, py, base_color, ParticleType::Droplet));
                            }
                            if fastrand::f32() < 0.2 {
                                self.particles.push(Particle::new(px, py, base_color, ParticleType::Bubble));
                            }
                        },
                        3 => {
                            // Tier 3: Viscous Goo
                             if fastrand::f32() < 0.6 {
                                self.particles.push(Particle::new(px, py, base_color, ParticleType::Droplet));
                            }
                            if fastrand::f32() < 0.3 {
                                self.particles.push(Particle::new(px, py, base_color, ParticleType::GooChunk));
                            }
                        },
                        4 => {
                             // Tier 4: Total Meltdown
                            self.particles.push(Particle::new(px, py, base_color, ParticleType::Droplet));
                            if fastrand::f32() < 0.5 {
                                self.particles.push(Particle::new(px, py, base_color, ParticleType::Bubble));
                            }
                            if fastrand::f32() < 0.5 {
                                self.particles.push(Particle::new(px, py, base_color, ParticleType::GooChunk));
                            }
                        },
                        _ => {}
                    }
                }
            }
        }

        self.score += match cleared_count {
            1 => 100 * self.level,
            2 => 300 * self.level,
            3 => 500 * self.level,
            4 => 800 * self.level,
            _ => 0,
        };
        
        // Apply Score Multiplier
        if self.active_bonuses.iter().any(|b| b.kind == crate::bonuses::BonusType::ScoreMultiplier) {
             self.score *= 2; // Simple double
        }
        
        // Apply Golden Pickaxe (+20% per stack)
        let pickaxes = self.active_bonuses.iter().filter(|b| b.kind == crate::bonuses::BonusType::GoldenPickaxe).count();
        if pickaxes > 0 {
            let mult = 1.0 + 0.2 * pickaxes as f32;
            self.score = (self.score as f32 * mult) as i32;
        }

        self.current_piece = self.next_pieces.remove(0);
        let p = self.get_next_piece();
        self.next_pieces.push(p);
        self.can_hold = true;

        if self.grid.is_collision(&self.current_piece) {
             // Life Insurance Check
             if let Some(pos) = self.active_bonuses.iter().position(|b| b.kind == crate::bonuses::BonusType::LifeInsurance) {
                 // Consume Life Insurance
                 self.active_bonuses.remove(pos);
                 self.grid.cells = [[None; GRID_WIDTH]; GRID_HEIGHT]; // Clear board!
                 self.effects.push(ComicEffect::new("SAVED!".to_string(), screen_width()/2.0, screen_height()/2.0, PINK));
                 self.audio.play_level_up(); // Sound feedback
             } else {
                 self.state = GameState::GameOver;
             }
        }
    }

    pub fn activate_bonus(&mut self, bonus: Bonus) {
        use crate::bonuses::BonusType;
        
        // Some bonuses might have immediate effects, others are stored
        let duration = match bonus.kind {
            BonusType::Chill => 60.0, // 60 seconds
            BonusType::ScoreMultiplier => 60.0, // Lasts for 60s
            // Relics are Infinite
            BonusType::TimeAnchor | BonusType::GoldenPickaxe | BonusType::VolatileGrid | BonusType::LifeInsurance => 999999.0,
            // Others are "One Time Use" on next lock
            _ => 9999.0, // Until used
        };

        self.active_bonuses.push(ActiveBonus {
            kind: bonus.kind,
            timer: duration,
        });

        // Visual Feedback
        self.effects.push(ComicEffect::new(
            format!("BONUS: {}", bonus.name),
            screen_width() / 2.0,
            screen_height() / 2.0 + 50.0,
            bonus.color,
        ));
        
        // Immediate Particle Burst for activation
        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;
        for _ in 0..20 {
             let ptype = match bonus.kind {
                 BonusType::Chill => ParticleType::Snowflake,
                 BonusType::LifeInsurance => ParticleType::Heart,
                 BonusType::Bomb | BonusType::VolatileGrid => ParticleType::Explosion,
                 BonusType::VerticalLaser => ParticleType::Spark,
                 _ => ParticleType::Bubble,
             };
             self.particles.push(Particle::new(
                 (cx - 100.0) + fastrand::f32() * 200.0,
                 (cy - 100.0) + fastrand::f32() * 200.0,
                 bonus.color,
                 ptype
             ));
        }
    }
}
