use crate::background::NatureBackground;
use crate::bidule::{Bidule, BiduleType};
use crate::constants::*;
use crate::effects::{ComicEffect, Particle};
use crate::grid::Grid;
use crate::sound_effects::AudioSystem;
use macroquad::prelude::*;
use macroquad::text::Font;

#[derive(PartialEq)]
pub enum GameState {
    Start,
    Playing,
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
            GameState::Playing => {
                let time = get_time();

                self.handle_input();

                let is_soft_drop = is_key_down(KeyCode::Down);
                let speed = if is_soft_drop {
                    0.05
                } else {
                    // Level-based speed: 0.5 at lvl 1, 0.4 at lvl 2, etc.
                    (0.5 * (0.9f64.powi(self.level - 1))).max(0.05)
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

        self.current_piece = self.next_pieces.remove(0);
        let p = self.get_next_piece();
        self.next_pieces.push(p);
        self.can_hold = true;

        if self.grid.is_collision(&self.current_piece) {
            self.state = GameState::GameOver;
        }
    }
}
