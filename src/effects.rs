// use crate::draw; removed
use macroquad::prelude::*;


pub struct ComicEffect {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub timer: f32,
    pub color: Color,
}

impl ComicEffect {
    pub fn new(text: String, x: f32, y: f32, color: Color) -> Self {
        Self {
            text,
            x,
            y,
            timer: 1.5,
            color,
        }
    }

    pub fn update(&mut self) -> bool {
        self.timer -= get_frame_time();
        self.y -= 20.0 * get_frame_time();
        self.timer > 0.0
    }

    pub fn draw(&self) {
        crate::ui::draw_text_styled(&self.text, self.x, self.y, 50.0, self.color);
    }
}

pub enum ParticleType {
    Droplet,  // Fails down, bounces
    Bubble,   // Floats up, wobbles
    GooChunk, // Heavy, sticky
    Explosion, // Fast radial burst
    Spark,    // Very fast, short life
    Snowflake,// Slow fall, sway
    Heart,    // Floats up slowly
    Shockwave,// Expanding ring
}

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub color: Color,
    pub life: f32,
    pub max_life: f32,
    pub size: f32,
    pub rotation: f32,
    pub angular_velocity: f32,
    pub kind: ParticleType,
}

impl Particle {
    pub fn new(x: f32, y: f32, color: Color, kind: ParticleType) -> Self {
        let angle = fastrand::f32() * std::f32::consts::PI * 2.0;
        
        let (vx, vy, life, size) = match kind {
            ParticleType::Droplet => {
                let speed = 2.0 + fastrand::f32() * 6.0;
                (angle.cos() * speed, -2.0 - fastrand::f32() * 5.0, 1.0, 5.0 + fastrand::f32() * 5.0)
            },
            ParticleType::Bubble => {
                let speed = 0.5 + fastrand::f32() * 1.5;
                (angle.cos() * speed, -1.0 - fastrand::f32() * 2.0, 2.0, 4.0 + fastrand::f32() * 6.0)
            },
            ParticleType::GooChunk => {
                let speed = 1.0 + fastrand::f32() * 3.0;
                (angle.cos() * speed, -1.0 - fastrand::f32() * 4.0, 1.5, 10.0 + fastrand::f32() * 8.0)
            },
            ParticleType::Explosion => {
                let speed = 5.0 + fastrand::f32() * 10.0;
                (angle.cos() * speed, angle.sin() * speed, 0.8, 10.0 + fastrand::f32() * 10.0)
            },
            ParticleType::Spark => {
                let speed = 10.0 + fastrand::f32() * 15.0;
                (angle.cos() * speed, angle.sin() * speed, 0.4, 3.0 + fastrand::f32() * 3.0)
            },
            ParticleType::Snowflake => {
                let speed = 0.5 + fastrand::f32() * 1.5;
                (angle.cos() * speed, 1.0 + fastrand::f32(), 3.0, 3.0 + fastrand::f32() * 3.0)
            },
            ParticleType::Heart => {
                (0.0, -2.0, 2.0, 20.0)
            },
            ParticleType::Shockwave => {
                (0.0, 0.0, 0.5, 10.0) // Short life, starts small
            }
        };

        Self {
            x,
            y,
            vx,
            vy,
            color,
            life,
            max_life: life,
            size,
            rotation: fastrand::f32() * 6.28,
            angular_velocity: -3.0 + fastrand::f32() * 6.0,
            kind,
        }
    }

    pub fn update(&mut self) -> bool {
        self.life -= get_frame_time();
        self.x += self.vx;
        self.y += self.vy;
        
        match self.kind {
            ParticleType::Droplet => {
                self.vy += 20.0 * get_frame_time(); // Heavy gravity
            },
            ParticleType::Bubble => {
                self.vy -= 5.0 * get_frame_time(); // Buoyancy (floats up)
                self.x += (get_time() * 5.0 + self.y as f64 * 0.1).sin() as f32 * 0.1; // Wobble
            },
            ParticleType::GooChunk => {
                self.vy += 10.0 * get_frame_time(); // Moderate gravity
                self.vx *= 0.95; // Friction
            },
            ParticleType::Explosion => {
                self.vx *= 0.9; // Drag
                self.vy *= 0.9;
                self.size *= 0.95;
            },
            ParticleType::Spark => {
                 self.vx *= 0.8;
                 self.vy *= 0.8;
            },
            ParticleType::Snowflake => {
                self.x += (get_time() * 2.0 + self.y as f64 * 0.05).sin() as f32 * 0.5; // Sway
                self.vy = 1.0; // Constant slow fall
            },
            ParticleType::Heart => {
                self.vy = -1.0; // Float up
                self.size = (get_time() * 5.0).sin() as f32 * 2.0 + 20.0; // Pulse
            },
            ParticleType::Shockwave => {
                self.size += 50.0 * get_frame_time(); // Expand fast
                self.life -= get_frame_time() * 2.0; // Die faster
            }
        }
        
        self.rotation += self.angular_velocity * get_frame_time();
        self.size *= 0.98; // Slow shrink
        self.life > 0.0
    }

    pub fn draw(&self, grid_x: f32, grid_y: f32) {
        let alpha = (self.life / self.max_life).min(1.0);
        let col = Color::new(self.color.r, self.color.g, self.color.b, alpha * 0.8);
        let px = grid_x + self.x;
        let py = grid_y + self.y;
        
        match self.kind {
            ParticleType::Bubble => {
                // Outline circle
                draw_circle_lines(px, py, self.size, 1.0, col);
                // Shine
                draw_circle(px - self.size * 0.3, py - self.size * 0.3, self.size * 0.2, Color::new(1.0, 1.0, 1.0, alpha));
            },
            ParticleType::Heart => {
                 draw_text("💖", px - self.size/2.0, py, self.size, WHITE);
            },
            ParticleType::Snowflake => {
                 draw_text("❄", px - self.size/2.0, py, self.size, col);
            },
            ParticleType::Spark => {
                draw_line(px, py, px - self.vx*0.2, py - self.vy*0.2, 2.0, col);
            },
            ParticleType::Shockwave => {
                draw_circle_lines(px, py, self.size, 3.0 * alpha, col);
            },
            _ => {
               // Explosion, Droplet, Goo, etc use circles
               draw_circle(px, py, self.size, col);
            }
        }
    }
}
