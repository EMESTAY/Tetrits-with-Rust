use macroquad::prelude::*;

// ==================================================================================
// ARCHIVED: Particle Background
// ==================================================================================

#[allow(dead_code)]
pub struct BackgroundParticle {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub rotation: f32,
    pub rot_speed: f32,
    pub speed: f32,
    pub kind: crate::bidule::BiduleType,
    pub color: Color,
}

#[allow(dead_code)]
impl BackgroundParticle {
    pub fn new() -> Self {
        use crate::bidule::BiduleType;
        let kinds = [
            BiduleType::I,
            BiduleType::O,
            BiduleType::T,
            BiduleType::S,
            BiduleType::Z,
            BiduleType::J,
            BiduleType::L,
        ];
        let kind = kinds[fastrand::usize(..kinds.len())];

        // Random slightly muted color
        let color = Color::new(
            fastrand::f32() * 0.5 + 0.2,
            fastrand::f32() * 0.5 + 0.2,
            fastrand::f32() * 0.5 + 0.2,
            0.15, // Low alpha for background fade
        );

        Self {
            x: fastrand::f32() * screen_width(),
            y: screen_height() + fastrand::f32() * 100.0, // Start below screen
            size: 20.0 + fastrand::f32() * 40.0,
            rotation: fastrand::f32() * 6.28,
            rot_speed: (fastrand::f32() - 0.5) * 2.0,
            speed: 0.5 + fastrand::f32() * 1.5,
            kind,
            color,
        }
    }

    pub fn update(&mut self) {
        self.y -= self.speed;
        self.rotation += self.rot_speed * get_frame_time();
    }
}

#[allow(dead_code)]
pub fn draw_particle_theme(particles: &[BackgroundParticle]) {
    // 1. Deep Gradient Background
    let top_color = Color::new(0.05, 0.0, 0.2, 1.0); // Deep Indigo
    let bottom_color = Color::new(0.0, 0.05, 0.1, 1.0); // Dark Teal/Black

    let steps = 40;
    let h = screen_height() / steps as f32;
    for i in 0..steps {
        let t = i as f32 / steps as f32;
        let color = Color::new(
            top_color.r + (bottom_color.r - top_color.r) * t,
            top_color.g + (bottom_color.g - top_color.g) * t,
            top_color.b + (bottom_color.b - top_color.b) * t,
            1.0,
        );
        draw_rectangle(0.0, i as f32 * h, screen_width(), h + 1.0, color);
    }

    // 2. Floating Particles
    for p in particles {
        use crate::bidule::Bidule;
        let bidule = Bidule::new(p.kind);
        // Center of the piece
        let cx: f32 = bidule.positions.iter().map(|pos| pos.x as f32).sum::<f32>() / 4.0;
        let cy: f32 = bidule.positions.iter().map(|pos| pos.y as f32).sum::<f32>() / 4.0;

        let cos_a = p.rotation.cos();
        let sin_a = p.rotation.sin();

        for pos in &bidule.positions {
            // Position relative to center
            let rx = pos.x as f32 - cx;
            let ry = pos.y as f32 - cy;

            // Rotate
            let rot_x = rx * cos_a - ry * sin_a;
            let rot_y = rx * sin_a + ry * cos_a;

            // Screen pos
            let screen_cx = p.x + rot_x * p.size;
            let screen_cy = p.y + rot_y * p.size;

            let half_s = p.size / 2.0 - 2.0;

            let corners = [
                (-half_s, -half_s),
                (half_s, -half_s),
                (half_s, half_s),
                (-half_s, half_s),
            ];

            let mut transformed_corners = Vec::new();
            for (cx, cy) in corners.iter() {
                let tx = cx * cos_a - cy * sin_a;
                let ty = cx * sin_a + cy * cos_a;
                transformed_corners.push(Vec2::new(screen_cx + tx, screen_cy + ty));
            }

            draw_triangle(
                transformed_corners[0],
                transformed_corners[1],
                transformed_corners[2],
                p.color,
            );
            draw_triangle(
                transformed_corners[0],
                transformed_corners[2],
                transformed_corners[3],
                p.color,
            );
        }
    }
}

// ==================================================================================
// ACTIVE: Nature Theme
// ==================================================================================

pub struct NatureBackground {
    pub clouds: Vec<Cloud>,
    pub sun_x: f32,
    pub tree_sway: f32,
    pub grass_sway: f32,
    pub day_time: f64,
}

pub struct Cloud {
    x: f32,
    y: f32,
    speed: f32,
    scale: f32,
    opacity: f32,
}

impl NatureBackground {
    pub fn new() -> Self {
        let mut clouds = Vec::new();
        for _ in 0..6 {
            clouds.push(Cloud::new());
        }

        Self {
            clouds,
            sun_x: 100.0,
            tree_sway: 0.0,
            grass_sway: 0.0,
            day_time: 0.0,
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();
        self.day_time += dt as f64;

        // Animate Clouds
        for c in &mut self.clouds {
            c.update();
        }

        // Animate Sun slowly
        self.sun_x =
            100.0 + (self.day_time * 5.0).sin() as f32 * 50.0 + (self.day_time * 2.0) as f32;
        // Wrap sun approx
        if self.sun_x > screen_width() + 100.0 {
            self.day_time = 0.0; // Reset for simplicity
        }

        // Sway
        self.tree_sway = (self.day_time * 2.0).sin() as f32 * 0.1; // Radians
        self.grass_sway = (self.day_time * 3.0).sin() as f32 * 3.0; // Pixels
    }

    pub fn draw(&self) {
        // 1. Sky Gradient
        let top = Color::new(0.4, 0.7, 1.0, 1.0); // Nice Blue
        let bot = Color::new(0.8, 0.95, 1.0, 1.0); // White/Blueish

        let steps = 40;
        let h = screen_height() / steps as f32;
        for i in 0..steps {
            let t = i as f32 / steps as f32;
            let c = Color::new(
                top.r + (bot.r - top.r) * t,
                top.g + (bot.g - top.g) * t,
                top.b + (bot.b - top.b) * t,
                1.0,
            );
            draw_rectangle(0.0, i as f32 * h, screen_width(), h + 1.0, c);
        }

        // 2. Sun
        // Bright core, soft glow
        draw_circle(self.sun_x, 80.0, 40.0, Color::new(1.0, 0.9, 0.2, 1.0));
        draw_circle(self.sun_x, 80.0, 50.0, Color::new(1.0, 0.9, 0.2, 0.5));
        draw_circle(self.sun_x, 80.0, 70.0, Color::new(1.0, 0.9, 0.2, 0.2));

        // 3. Clouds
        for c in &self.clouds {
            c.draw();
        }

        // 4. Background Hills?
        // Let's do some rolling green hills at the bottom
        self.draw_hills();

        // 5. Trees (Parallax or just dynamic?)
        self.draw_trees();
    }

    fn draw_hills(&self) {
        let base_y = screen_height() - 50.0;
        // Draw a sine wave based hill
        let hill_color = Color::new(0.2, 0.6, 0.3, 1.0);

        for x in (0..screen_width() as i32).step_by(5) {
            let xf = x as f32;
            let y_off = (xf * 0.005).sin() * 50.0 + (xf * 0.02).sin() * 10.0;
            draw_line(
                xf,
                base_y - 50.0 + y_off,
                xf,
                screen_height(),
                6.0,
                hill_color,
            );
        }
    }

    fn draw_trees(&self) {
        // Simple procedural trees
        // Left Side
        self.draw_tree(100.0, screen_height() - 80.0, 1.0);
        self.draw_tree(50.0, screen_height() - 60.0, 0.8);

        // Right Side
        self.draw_tree(screen_width() - 100.0, screen_height() - 90.0, 1.1);
        self.draw_tree(screen_width() - 40.0, screen_height() - 50.0, 0.9);
    }

    fn draw_tree(&self, x: f32, y: f32, scale: f32) {
        let trunk_w = 15.0 * scale;
        let trunk_h = 60.0 * scale;
        let foliage_rad = 40.0 * scale;

        // Trunk
        draw_rectangle(
            x - trunk_w / 2.0,
            y - trunk_h,
            trunk_w,
            trunk_h,
            Color::new(0.4, 0.25, 0.1, 1.0),
        );

        // Foliage (Swaying)
        // Two or three circles
        let sway = (x * 0.1 + self.tree_sway * 5.0).sin() * 5.0;

        let cx = x + sway;
        let cy = y - trunk_h;

        let foliage_c = Color::new(0.1, 0.5, 0.2, 1.0);

        draw_circle(cx, cy - 20.0 * scale, foliage_rad, foliage_c);
        draw_circle(
            cx - 20.0 * scale,
            cy + 10.0 * scale,
            foliage_rad * 0.8,
            foliage_c,
        );
        draw_circle(
            cx + 20.0 * scale,
            cy + 10.0 * scale,
            foliage_rad * 0.8,
            foliage_c,
        );
    }
}

impl Cloud {
    fn new() -> Self {
        Self {
            x: fastrand::f32() * screen_width(),
            y: 50.0 + fastrand::f32() * 150.0,
            speed: 5.0 + fastrand::f32() * 15.0,
            scale: 0.8 + fastrand::f32() * 0.5,
            opacity: 0.5 + fastrand::f32() * 0.3,
        }
    }

    fn update(&mut self) {
        self.x += self.speed * get_frame_time();
        if self.x > screen_width() + 100.0 {
            self.x = -100.0;
            self.y = 50.0 + fastrand::f32() * 150.0;
        }
    }

    fn draw(&self) {
        let c = Color::new(1.0, 1.0, 1.0, self.opacity);
        let s = self.scale;
        draw_circle(self.x, self.y, 30.0 * s, c);
        draw_circle(self.x - 25.0 * s, self.y + 10.0 * s, 20.0 * s, c);
        draw_circle(self.x + 25.0 * s, self.y + 5.0 * s, 22.0 * s, c);
    }
}
