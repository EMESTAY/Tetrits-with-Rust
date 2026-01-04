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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParticleType {
    Droplet,  // falls down, bounces
    Bubble,   // Floats up, wobbles
    GooChunk, // Heavy, sticky
    Explosion, // Fast radial burst
    Spark,    // Very fast, short life
    Snowflake,// Slow fall, sway
    Heart,    // Floats up slowly
    Shockwave,// Expanding ring
}

impl ParticleType {
    /// Returns initial (vx, vy, life, size)
    pub fn init(&self, angle: f32) -> (f32, f32, f32, f32) {
        match self {
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
        }
    }

    pub fn update_motion(&self, p: &mut Particle, dt: f32) {
        match self {
            ParticleType::Droplet => {
                p.vy += 20.0 * dt; // Heavy gravity
            },
            ParticleType::Bubble => {
                p.vy -= 5.0 * dt; // Buoyancy (floats up)
                p.x += (get_time() * 5.0 + p.y as f64 * 0.1).sin() as f32 * 0.1; // Wobble
            },
            ParticleType::GooChunk => {
                p.vy += 10.0 * dt; // Moderate gravity
                p.vx *= 0.95; // Friction
            },
            ParticleType::Explosion => {
                p.vx *= 0.9; // Drag
                p.vy *= 0.9;
                p.size *= 0.95;
            },
            ParticleType::Spark => {
                 p.vx *= 0.8;
                 p.vy *= 0.8;
            },
            ParticleType::Snowflake => {
                p.x += (get_time() * 2.0 + p.y as f64 * 0.05).sin() as f32 * 0.5; // Sway
                p.vy = 1.0; // Constant slow fall
            },
            ParticleType::Heart => {
                p.vy = -1.0; // Float up
                p.size = (get_time() * 5.0).sin() as f32 * 2.0 + 20.0; // Pulse
            },
            ParticleType::Shockwave => {
                p.size += 50.0 * dt; // Expand fast
                p.life -= dt * 2.0; // Die faster
            }
        }
    }

    pub fn draw_style(&self, p: &Particle, px: f32, py: f32, col: Color) {
        match self {
            ParticleType::Bubble => {
                // Outline circle
                draw_circle_lines(px, py, p.size, 1.0, col);
                // Shine
                let alpha = (p.life / p.max_life).min(1.0);
                draw_circle(px - p.size * 0.3, py - p.size * 0.3, p.size * 0.2, Color::new(1.0, 1.0, 1.0, alpha));
            },
            ParticleType::Heart => {
                 draw_text("💖", px - p.size/2.0, py, p.size, WHITE);
            },
            ParticleType::Snowflake => {
                 draw_text("❄", px - p.size/2.0, py, p.size, col);
            },
            ParticleType::Spark => {
                draw_line(px, py, px - p.vx*0.2, py - p.vy*0.2, 2.0, col);
            },
            ParticleType::Shockwave => {
                let alpha = (p.life / p.max_life).min(1.0);
                draw_circle_lines(px, py, p.size, 3.0 * alpha, col);
            },
            _ => {
               // Explosion, Droplet, Goo, etc use circles
               draw_circle(px, py, p.size, col);
            }
        }
    }
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
        
        // kind is Copy now
        let (vx, vy, life, size) = kind.init(angle);

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
        let dt = get_frame_time();
        self.life -= dt;
        self.x += self.vx;
        self.y += self.vy;
        
        let kind = self.kind;
        kind.update_motion(self, dt);
        
        self.rotation += self.angular_velocity * dt;
        self.size *= 0.98; // Slow shrink
        self.life > 0.0
    }

    pub fn draw(&self, grid_x: f32, grid_y: f32) {
        let alpha = (self.life / self.max_life).min(1.0);
        let col = Color::new(self.color.r, self.color.g, self.color.b, alpha * 0.8);
        let px = grid_x + self.x;
        let py = grid_y + self.y;
        
        self.kind.draw_style(self, px, py, col);
    }
}
