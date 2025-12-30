use crate::bidule::{Bidule, BiduleType};
use crate::constants::*;
use crate::effects::{ComicEffect, Particle};
use crate::grid::Grid;
use crate::sound_effects::AudioSystem;
use macroquad::prelude::*;
use macroquad::text::Font;

pub struct Game {
    pub grid: Grid,
    pub current_piece: Bidule,
    pub next_pieces: Vec<Bidule>,
    pub hold_piece: Option<Bidule>,
    pub can_hold: bool,
    pub score: i32,
    pub game_over: bool,
    pub last_fall_time: f64,
    pub fall_speed: f64,
    // Visuals
    pub effects: Vec<ComicEffect>,
    pub particles: Vec<Particle>,
    pub font: Option<Font>,
    pub audio: AudioSystem,
    pub is_music_playing: bool,
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
            game_over: false,
            last_fall_time: get_time(),
            fall_speed: 0.5,
            effects: Vec::new(),
            particles: Vec::new(),
            font,
            audio,
            bag: Vec::new(),
            is_music_playing: true,
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
        if self.game_over {
            if is_key_pressed(KeyCode::R) {
                let font = self.font.take();
                let audio = self.audio.clone();
                *self = Game::new(font, audio);
            }
            return;
        }

        let time = get_time();

        self.handle_input();

        let is_soft_drop = is_key_down(KeyCode::Down);
        let speed = if is_soft_drop { 0.05 } else { self.fall_speed };

        if time - self.last_fall_time > speed {
            self.current_piece.pos.y += 1;
            if self.grid.is_collision(&self.current_piece) {
                self.current_piece.pos.y -= 1;
                self.lock_and_spawn();
            }
            self.last_fall_time = time;
        }

        self.effects.retain_mut(|e| e.update());
        self.particles.retain_mut(|p| p.update());
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
                    if let Some(c) = self.grid.cells[ny as usize][nx as usize] {
                        if c == p_color {
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
        let cleared = self.grid.clear_lines();

        if cleared == 4 {
            self.audio.play_tetris();
        }

        if cleared > 0 {
            let text = match cleared {
                1 => "ZAP!",
                2 => "POW!",
                3 => "BAM!",
                4 => "KABOOM!",
                _ => "!",
            };
            self.effects.push(ComicEffect::new(
                text.to_string(),
                (self.current_piece.pos.x as f32 * BLOCK_SIZE) + 150.0,
                (self.current_piece.pos.y as f32 * BLOCK_SIZE) + 100.0,
                RED,
            ));

            for _ in 0..50 {
                self.particles.push(Particle::new(
                    (self.current_piece.pos.x as f32 * BLOCK_SIZE) + 60.0,
                    (self.current_piece.pos.y as f32 * BLOCK_SIZE) + 60.0,
                    Color::new(
                        0.5 + fastrand::f32() * 0.5,
                        0.5 + fastrand::f32() * 0.5,
                        0.5 + fastrand::f32() * 0.5,
                        1.0,
                    ),
                ));
            }
        }

        self.score += match cleared {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0,
        };

        self.current_piece = self.next_pieces.remove(0);
        let p = self.get_next_piece();
        self.next_pieces.push(p);
        self.can_hold = true;

        if self.grid.is_collision(&self.current_piece) {
            self.game_over = true;
        }
    }
}
