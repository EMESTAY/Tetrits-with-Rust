use crate::draw;
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
        draw::draw_text_styled(&self.text, self.x, self.y, 50.0, self.color);
    }
}

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub color: Color,
    pub life: f32,
    pub size: f32,
    pub rotation: f32,
    pub angular_velocity: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        let angle = fastrand::f32() * std::f32::consts::PI * 2.0;
        let speed = 3.0 + fastrand::f32() * 5.0;

        Self {
            x,
            y,
            vx: angle.cos() * speed,
            vy: angle.sin() * speed,
            color,
            life: 1.2,
            size: 8.0 + fastrand::f32() * 7.0,
            rotation: fastrand::f32() * 6.28,
            angular_velocity: -5.0 + fastrand::f32() * 10.0,
        }
    }

    pub fn update(&mut self) -> bool {
        self.life -= get_frame_time();
        self.x += self.vx;
        self.y += self.vy;
        self.vy += 10.0 * get_frame_time(); // Gravity
        self.rotation += self.angular_velocity * get_frame_time();
        self.size *= 0.95; // Shrink
        self.life > 0.0
    }

    pub fn draw(&self, grid_x: f32, grid_y: f32) {
        draw::draw_jelly_block(
            grid_x + self.x,
            grid_y + self.y,
            self.size,
            self.color,
            draw::Connectivity {
                top: None,
                right: None,
                bottom: None,
                left: None,
            },
            false,
        );
    }
}
