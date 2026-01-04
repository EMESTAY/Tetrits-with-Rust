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

    // Dynamics (DAS/ARR/Lock Delay)
    pub das_timer: f64,
    pub das_direction: Option<i32>, // -1 Left, 1 Right
    pub arr_timer: f64,
    pub lock_timer: f64,
    pub lock_resets: usize,
    pub is_soft_dropping: bool,
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

            // Dynamics
            das_timer: 0.0,
            das_direction: None,
            arr_timer: 0.0,
            lock_timer: 0.0,
            lock_resets: 0,
            is_soft_dropping: false,
        };

        game.fill_bag();
        game.current_piece = game.get_next_piece();
        for _ in 0..3 {
            let p = game.get_next_piece();
            game.next_pieces.push(p);
        }

        game
    }

    // --- Helper Methods for Bonuses ---
    pub fn has_bonus(&self, kind: crate::bonuses::BonusType) -> bool {
        self.active_bonuses.iter().any(|b| b.kind == kind)
    }

    pub fn bonus_count(&self, kind: crate::bonuses::BonusType) -> usize {
        self.active_bonuses.iter().filter(|b| b.kind == kind).count()
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
        
        // 20% chance to add a Plus piece to this bag
        if fastrand::f32() < 0.20 {
             types.push(BiduleType::Plus);
        }

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
                // Grid Navigation (3 columns)
                if is_key_pressed(KeyCode::Down) {
                    let next = self.bonus_selection_idx + 3;
                    if next < self.bonus_options.len() {
                        self.bonus_selection_idx = next;
                        self.audio.play_hold();
                    }
                }
                if is_key_pressed(KeyCode::Up) {
                    if self.bonus_selection_idx >= 3 {
                        self.bonus_selection_idx -= 3;
                        self.audio.play_hold();
                    }
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

                // Gravity / Soft Drop
                let mut speed;
                let base_speed = (0.5 * (0.9f64.powi(self.level - 1))).max(0.05);
                
                if self.is_soft_dropping {
                    speed = 0.05; // Fast drop
                } else {
                    speed = base_speed;
                    // CHILL Bonus
                    if self.has_bonus(crate::bonuses::BonusType::Chill) {
                        speed *= 1.5;
                    }
                    // TIME ANCHOR
                    let anchors = self.bonus_count(crate::bonuses::BonusType::TimeAnchor);
                    if anchors > 0 {
                        speed *= 1.0 + (0.1 * anchors as f64);
                    }
                }

                // Check for lock delay condition (touching down)
                // Temporarily move down to check
                self.current_piece.pos.y += 1;
                let is_touching_down = self.grid.is_collision(&self.current_piece);
                self.current_piece.pos.y -= 1;

                if is_touching_down {
                    if self.lock_timer == 0.0 {
                         self.lock_timer = time;
                    }
                    // Time limit on bottom
                    if time - self.lock_timer > LOCK_DELAY {
                        self.lock_and_spawn();
                    }
                } else {
                    self.lock_timer = 0.0;
                    // Standard Gravity
                     if time - self.last_fall_time > speed {
                        self.current_piece.pos.y += 1;
                        if self.grid.is_collision(&self.current_piece) {
                             self.current_piece.pos.y -= 1;
                             // Just landed, don't lock immediately. Wait for next frame/lock delay.
                             // Reset fall timer to prevent double-move
                             self.last_fall_time = time;
                        } else {
                            // Falling successfully
                            self.last_fall_time = time;
                            // Reset lock resets when falling? Standard tetris does.
                            if self.lock_resets < MAX_LOCK_RESETS {
                                self.lock_timer = 0.0; 
                            }
                        }
                    }
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

        self.background.active_tomb = self.has_bonus(crate::bonuses::BonusType::LifeInsurance);
        self.background.update();
    }

    fn handle_input(&mut self) {
        let dt = get_frame_time();

        // --- DAS / ARR (Horizontal Movement) ---
        let mut dir = 0;
        if is_key_down(KeyCode::Left) { dir = -1; }
        if is_key_down(KeyCode::Right) { dir = 1; } // Right takes priority if both pressed? Or cancel?

        // Reset if direction changed or released
        if self.das_direction != Some(dir) {
            self.das_direction = Some(dir);
            self.das_timer = 0.0;
            self.arr_timer = 0.0;
            
            // Initial Move on press
            if dir != 0 {
                self.move_piece(dir, 0);
            }
        } else if let Some(d) = self.das_direction {
            // Holding same direction
             if d != 0 {
                self.das_timer += dt as f64;
                if self.das_timer > DAS_DELAY {
                    self.arr_timer += dt as f64;
                    if self.arr_timer > ARR_DELAY {
                        self.move_piece(d, 0);
                        self.arr_timer = 0.0; // Reset for next zip
                    }
                }
             }
        }

        // --- Soft Drop ---
        self.is_soft_dropping = is_key_down(KeyCode::Down);

        // --- Rotation ---
        if is_key_pressed(KeyCode::Up) {
            let mut rotated = self.current_piece.clone();
            rotated.rotate();
            if !self.grid.is_collision(&rotated) {
                self.current_piece = rotated;
                self.on_move_reset_lock();
            } else {
                // Wall kick (simple)
                rotated.pos.x += 1;
                if !self.grid.is_collision(&rotated) {
                    self.current_piece = rotated;
                    self.on_move_reset_lock();
                } else {
                    rotated.pos.x -= 2;
                    if !self.grid.is_collision(&rotated) {
                        self.current_piece = rotated;
                        self.on_move_reset_lock();
                    }
                }
            }
        }

        // --- Hold ---
        if is_key_pressed(KeyCode::C) {
            if self.can_hold {
                self.audio.play_hold();
                if let Some(mut held) = self.hold_piece.clone() {
                    let mut current = self.current_piece.clone();
                    // Reset position
                    let spawn_x = (GRID_WIDTH / 2) as i32 - 2;
                    current.pos = crate::bidule::Point { x: spawn_x, y: 0 };
                    held.pos = crate::bidule::Point { x: spawn_x, y: 0 };

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
                // Reset timers
                self.lock_timer = 0.0;
                self.lock_resets = 0;
            }
        }

        // --- Hard Drop ---
        if is_key_pressed(KeyCode::Space) {
            let ghost = self.get_ghost_position();
            let distance = ghost.y - self.current_piece.pos.y;
            
            // Slam effects
            if distance > 0 {
                // Reduced shake: Base 2.0 + 0.2 per block fallen
                self.screen_shake = 2.0 + distance as f32 * 0.2; 
                
                // Slam Shockwave
                let cx = (ghost.x as f32 * BLOCK_SIZE) + BLOCK_SIZE * 2.0;
                let cy = (ghost.y as f32 * BLOCK_SIZE) + BLOCK_SIZE;
                 self.particles.push(Particle::new(cx, cy, WHITE, ParticleType::Shockwave));
            }
            
            self.current_piece.pos = ghost;
            self.lock_and_spawn();
        }


        // --- DEV MODE: Click "Sun" (Top Left Background) to Unlock All Bonuses ---
        if is_mouse_button_pressed(MouseButton::Left) {
             let (mx, my) = mouse_position();
             // Target the actual Sun in the background
             let sun_pos = self.background.sun_pos;
             let dx = mx - sun_pos.x;
             let dy = my - sun_pos.y;
             
             // Radius 80 (Sun is 50 + glow)
             if dx*dx + dy*dy < 80.0 * 80.0 {
                 self.state = GameState::ChooseBonus;
                 self.bonus_options = crate::bonuses::Bonus::get_all(); // DEBUG: Show ALL bonuses
                 self.bonus_selection_idx = 0;
                 self.audio.play_level_up();
             }
        }
    }

    fn move_piece(&mut self, dx: i32, _dy: i32) {
        self.current_piece.pos.x += dx;
        if self.grid.is_collision(&self.current_piece) {
            self.current_piece.pos.x -= dx;
        } else {
            // Successful move
            self.on_move_reset_lock();
        }
    }

    // Standard Tetris Rule: Moving/Rotating resets lock timer (up to limit)
    fn on_move_reset_lock(&mut self) {
        if self.lock_timer > 0.0 && self.lock_resets < MAX_LOCK_RESETS {
            self.lock_timer = 0.0;
            self.lock_resets += 1;
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

        // --- BOMB EXPLOSION LOGIC ---
        if self.current_piece.kind == crate::bidule::BiduleType::Bomb {
            // Explode 3x3 area around center of piece? 
            // The user said "explodes on impact, clearing a 3x3 area".
            // Since the piece itself is large (Diamond), maybe explode around its center?
            // Diamond center is at pos + (1,1).
            let cx = self.current_piece.pos.x + 1;
            let cy = self.current_piece.pos.y + 1;
            
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let tx = cx + dx;
                    let ty = cy + dy;
                    if tx >= 0 && tx < GRID_WIDTH as i32 && ty >= 0 && ty < GRID_HEIGHT as i32 {
                         self.grid.cells[ty as usize][tx as usize] = None;
                         // Particle
                         self.particles.push(Particle::new(
                            (tx as f32 * BLOCK_SIZE) + BLOCK_SIZE/2.0,
                            (ty as f32 * BLOCK_SIZE) + BLOCK_SIZE/2.0,
                            ORANGE, // Lava colorish
                            ParticleType::Explosion,
                        ));
                    }
                }
            }
            self.screen_shake = 20.0;
            self.audio.play_level_up(); // BOOM!
        }

        // --- SPECIAL PIECE MECHANICS (Jelly/Sand) ---
        if self.current_piece.kind == crate::bidule::BiduleType::Jelly {
             crate::special_bidule::resolve_jelly_physics(&mut self.grid);
        }

        // --- ONE-TIME BONUSES (Bomb / Laser) ---
        // --- ONE-TIME BONUSES (Bomb / Laser) ---
        // --- SPECIAL MECHANICS (On Lock) ---
        crate::special_bidule::resolve_special_mechanics_on_lock(
            &mut self.grid,
            &self.current_piece,
            &mut self.particles,
            &self.audio,
            &mut self.screen_shake
        );

        // Remove expired bonuses (Time limit) - logic is in update, but here we might remove 1-time uses?
        // Actually, with new system, Bomb/Laser/Drill/Anvil/Ghost are PIECES, simpler.
        // Active Bonuses are only TimeAnchor, Chill, Pickaxe, LifeInsurance.
        // Chill has timer. Others are infinite.
        // So no need to remove "used" bonuses here because they are consumed as pieces.

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
        

        
        // Apply Golden Pickaxe (+20% per stack)
        let pickaxes = self.bonus_count(crate::bonuses::BonusType::GoldenPickaxe);
        if pickaxes > 0 {
            let mult = 1.0 + 0.2 * pickaxes as f32;
            self.score = (self.score as f32 * mult) as i32;
        }

        self.current_piece = self.next_pieces.remove(0);
        let p = self.get_next_piece();
        self.next_pieces.push(p);
        self.can_hold = true;
        
        // Reset Logic for new piece
        self.lock_timer = 0.0;
        self.lock_resets = 0;
        self.last_fall_time = get_time(); // Give full beat before drop

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
            BonusType::LiquidFiller => 0.0, // Instant

            // Relics are Infinite
            BonusType::TimeAnchor | BonusType::GoldenPickaxe | BonusType::LifeInsurance => 999999.0,
            // Others are "One Time Use" on next lock
            _ => 9999.0, // Until used
        };

        // --- INSTANT EFFECT BONUSES ---
        // --- INSTANT/SPAWN EFFECT BONUSES ---
        if bonus.kind == BonusType::LiquidFiller {
             // Force next piece to be JELLY
             let mut jelly = crate::bidule::Bidule::new(crate::bidule::BiduleType::Jelly);
             jelly.pos.y = 2; // Spawn CLEARLY ON grid
             self.next_pieces[0] = jelly;
             
             self.audio.play_level_up();
             return; // Don't add to active bonuses
        }

        // --- INSTANT EFFECT: BOMB BIDULE ---
        if bonus.kind == BonusType::Bomb {
             let mut bomb = crate::bidule::Bidule::new(crate::bidule::BiduleType::Bomb);
             bomb.pos.y = 2;
             self.next_pieces[0] = bomb;
             self.audio.play_level_up();
             return;
        }

        // --- INSTANT EFFECT: LASER BIDULE ---
        if bonus.kind == BonusType::VerticalLaser {
             let mut laser = crate::bidule::Bidule::new(crate::bidule::BiduleType::Laser);
             laser.pos.y = 2;
             self.next_pieces[0] = laser;
             self.audio.play_level_up();
             return;
        }

        // --- INSTANT EFFECT: DRILL BIDULE ---
        if bonus.kind == BonusType::Drill {
             let mut drill = crate::bidule::Bidule::new(crate::bidule::BiduleType::Drill);
             drill.pos.y = 2;
             self.next_pieces[0] = drill;
             self.audio.play_level_up();
             return;
        }

        // --- INSTANT EFFECT: ANVIL BIDULE ---
        if bonus.kind == BonusType::Anvil {
             let mut anvil = crate::bidule::Bidule::new(crate::bidule::BiduleType::Anvil);
             anvil.pos.y = 2;
             self.next_pieces[0] = anvil;
             self.audio.play_level_up();
             return;
        }

        // --- INSTANT EFFECT: GHOST BIDULE ---



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
                 BonusType::Bomb => ParticleType::Explosion,
                 BonusType::VerticalLaser => ParticleType::Spark,
                 BonusType::LiquidFiller => ParticleType::GooChunk,
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
