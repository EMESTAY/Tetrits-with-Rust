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
// ACTIVE: Nature Theme (Advanced)
// ==================================================================================

pub struct NatureBackground {
    pub clouds: Vec<Cloud>,
    pub river_ripples: Vec<Ripple>,
    pub trees: Vec<Tree>,
    pub sun_pos: Vec2,
    pub day_time: f64,
    pub active_tomb: bool,
}

pub struct Cloud {
    x: f32,
    y: f32,
    speed: f32,
    scale: f32,
    opacity: f32,
    puff_offsets: Vec<(f32, f32, f32)>, // (dx, dy, scale)
}

pub struct Ripple {
    t: f32, // 0.0 to 1.0 along the river path
    speed: f32,
    offset_x: f32, // Horizontal offset from river center
}

pub struct Tree {
    x: f32,
    y: f32,
    scale: f32,
    _variation: i32, // To choose foliage shape
    sway_phase: f32,
}

impl NatureBackground {
    pub fn new() -> Self {
        let mut clouds = Vec::new();
        for _ in 0..8 {
            clouds.push(Cloud::new());
        }

        let mut river_ripples = Vec::new();
        for _ in 0..20 {
            river_ripples.push(Ripple::new());
        }

        let mut trees = Vec::new();
        // Trees placement - Procedural but somewhat deterministic for nice composition
        // Far hills
        trees.push(Tree::new(150.0, screen_height() - 250.0, 0.6));
        trees.push(Tree::new(300.0, screen_height() - 280.0, 0.5));
        trees.push(Tree::new(
            screen_width() - 200.0,
            screen_height() - 260.0,
            0.6,
        ));

        // Mid hills
        trees.push(Tree::new(80.0, screen_height() - 150.0, 0.9));
        trees.push(Tree::new(
            screen_width() - 100.0,
            screen_height() - 180.0,
            0.8,
        ));

        // Foreground (large)
        trees.push(Tree::new(50.0, screen_height() - 50.0, 1.3));
        trees.push(Tree::new(
            screen_width() - 60.0,
            screen_height() - 60.0,
            1.2,
        ));

        Self {
            clouds,
            river_ripples,
            trees,
            sun_pos: Vec2::new(100.0, 100.0),
            day_time: 0.0,
            active_tomb: false,
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();
        self.day_time += dt as f64;

        // Animate Clouds
        for c in &mut self.clouds {
            c.update();
        }

        // Animate Ripples
        for r in &mut self.river_ripples {
            r.update();
        }

        // Sun Sway
        self.sun_pos.x = 120.0 + (self.day_time * 0.2).sin() as f32 * 10.0;
        self.sun_pos.y = 100.0 + (self.day_time * 0.3).sin() as f32 * 5.0;
    }

    pub fn draw(&self) {
        let sw = screen_width();
        let sh = screen_height();

        // 1. Sky (Cartoony Blue Gradient)
        let sky_top = Color::new(0.4, 0.8, 1.0, 1.0); // Bright Sky Blue
        let sky_bot = Color::new(0.8, 0.95, 1.0, 1.0); // Pale Blue/White
        self.draw_gradient_rect(0.0, 0.0, sw, sh, sky_top, sky_bot);

        // 2. Sun (Top Left)
        draw_circle(
            self.sun_pos.x,
            self.sun_pos.y,
            50.0,
            Color::new(1.0, 0.9, 0.0, 1.0),
        ); // Core
        draw_circle(
            self.sun_pos.x,
            self.sun_pos.y,
            65.0,
            Color::new(1.0, 0.9, 0.0, 0.3),
        ); // Glow 1
        draw_circle(
            self.sun_pos.x,
            self.sun_pos.y,
            90.0,
            Color::new(1.0, 0.9, 0.0, 0.1),
        ); // Glow 2

        // 3. Clouds (Behind hills)
        for c in &self.clouds {
            c.draw();
        }

        // 4. Hills and River Layers
        // We draw back-to-front

        // --- Layer 3: Distant Hills (Faded Green) ---
        self.draw_hill_layer(sh - 250.0, 60.0, Color::new(0.5, 0.75, 0.4, 1.0), 0.01);

        // --- Layer 2: Mid Hills (Vibrant Green) ---
        // We split the midground to let the river flow through?
        // Actually, let's draw the river "on top" of a base valley, then hills on sides.

        // Draw Valley Base
        draw_rectangle(0.0, sh - 300.0, sw, 300.0, Color::new(0.45, 0.8, 0.3, 1.0));

        // --- RIVER ---
        self.draw_river();

        // --- Layer 2.5: Mid Hills (Side Mounds) ---
        // Left Mid Mound
        self.draw_hill_mound(
            0.0,
            sh - 100.0,
            400.0,
            150.0,
            Color::new(0.4, 0.75, 0.25, 1.0),
        );
        // Right Mid Mound
        self.draw_hill_mound(
            sw - 300.0,
            sh - 120.0,
            500.0,
            180.0,
            Color::new(0.4, 0.75, 0.25, 1.0),
        );

        // --- Layer 1: Foreground Hills (Brightest Green) ---
        // Left Fore Mound
        self.draw_hill_mound(
            -50.0,
            sh - 20.0,
            300.0,
            100.0,
            Color::new(0.5, 0.85, 0.3, 1.0),
        );
        // Right Fore Mound
        self.draw_hill_mound(
            sw - 200.0,
            sh - 30.0,
            450.0,
            120.0,
            Color::new(0.5, 0.85, 0.3, 1.0),
        );

        // 5. Features (Houses, Trees)
        // Houses on mid hills
        self.draw_house(200.0, sh - 230.0, 0.7);
        self.draw_house(sw - 250.0, sh - 250.0, 0.6);
        self.draw_house(sw - 120.0, sh - 200.0, 0.8);

        // Trees (Sorted by Y for depth?)
        // Simple manual ordering logic based on creation order in `new()` (Far to Near)
        // Trees (Sorted by Y for depth?)
        // Simple manual ordering logic based on creation order in `new()` (Far to Near)
        for t in &self.trees {
            t.draw(self.day_time as f32);
        }

        // 6. Special Objects: Tomb of Life Insurance
        if self.active_tomb {
            // Draw somewhat prominently in foreground
            self.draw_tomb(sw - 150.0, sh - 80.0, 1.0);
        }
    }

    fn draw_tomb(&self, x: f32, y: f32, scale: f32) {
        let w = 40.0 * scale;
        let h = 60.0 * scale;
        
        let stone_col = Color::new(0.6, 0.6, 0.65, 1.0);
        let dark_stone = Color::new(0.5, 0.5, 0.55, 1.0);
        
        // Base Mounded Dirt
        draw_ellipse(x + w/2.0, y + h, w * 0.8, 10.0 * scale, 0.0, Color::new(0.35, 0.25, 0.15, 1.0));
        
        // Stone Body (Arched top)
        // Rect bottom
        draw_rectangle(x, y + h*0.3, w, h*0.7, stone_col);
        // Circle top
        draw_circle(x + w/2.0, y + h*0.3, w/2.0, stone_col);
        
        // Inner detail/Shadow
        draw_circle(x + w/2.0, y + h*0.3, w/2.0 - 4.0, dark_stone);
        draw_rectangle(x + 4.0, y + h*0.3, w - 8.0, h*0.7 - 4.0, dark_stone);
        
        // Cross / Text
        let text_col = Color::new(0.3, 0.3, 0.35, 1.0); // Dark gray
        // Vertical line
        draw_rectangle(x + w/2.0 - 2.0, y + h*0.3, 4.0, 20.0, text_col);
        // Horizontal line
        draw_rectangle(x + w/2.0 - 8.0, y + h*0.3 + 6.0, 16.0, 4.0, text_col);
        
        // "R.I.P" (Too small for text? Maybe just lines)
    }

    fn draw_river(&self) {
        // Curve parameters
        let sw = screen_width();
        let sh = screen_height();

        // River Path: Starts right-mid, curves to bottom-center
        // P0: (sw, sh - 300)
        // P1: (sw/2, sh - 200) -> Control
        // P2: (sw/2 - 100, sh - 50) -> Control
        // P3: (sw/2 - 200, sh + 50) -> End

        // We simulate a river by drawing many quads (strips)
        let segments = 50;
        let river_color = Color::new(0.3, 0.6, 0.9, 1.0);

        let get_river_point = |t: f32| -> Vec2 {
            // Simple Quadratic Bezier
            let start = Vec2::new(sw, sh - 250.0);
            let control = Vec2::new(sw * 0.5, sh - 200.0);
            let end = Vec2::new(sw * 0.2, sh + 50.0);

            let mt = 1.0 - t;
            let x = mt * mt * start.x + 2.0 * mt * t * control.x + t * t * end.x;
            let y = mt * mt * start.y + 2.0 * mt * t * control.y + t * t * end.y;
            Vec2::new(x, y)
        };

        // Draw River Body
        for i in 0..segments {
            let t1 = i as f32 / segments as f32;
            let t2 = (i + 1) as f32 / segments as f32;

            let p1 = get_river_point(t1);
            let p2 = get_river_point(t2);

            // Width increases as it comes closer (Perspective)
            let w1 = 50.0 + t1 * 200.0;
            let w2 = 50.0 + t2 * 200.0;

            let vertices = [
                Vec2::new(p1.x - w1 / 2.0, p1.y),
                Vec2::new(p1.x + w1 / 2.0, p1.y),
                Vec2::new(p2.x + w2 / 2.0, p2.y),
                Vec2::new(p2.x - w2 / 2.0, p2.y),
            ];

            draw_triangle(vertices[0], vertices[1], vertices[2], river_color);
            draw_triangle(vertices[0], vertices[2], vertices[3], river_color);
        }

        // Draw Flowing Ripples
        for r in &self.river_ripples {
            let p = get_river_point(r.t);
            let w = 50.0 + r.t * 200.0;
            // Constrain offset to be within river width
            let x = p.x + r.offset_x * (w * 0.4);
            let y = p.y;

            let size_x = 10.0 + r.t * 20.0;
            let size_y = 2.0 + r.t * 4.0;

            draw_rectangle(
                x - size_x / 2.0,
                y - size_y / 2.0,
                size_x,
                size_y,
                Color::new(1.0, 1.0, 1.0, 0.4),
            );
        }
    }

    fn draw_hill_layer(&self, start_y: f32, amplitude: f32, color: Color, freq: f32) {
        let sw = screen_width();
        let sh = screen_height();

        let mut polygon = Vec::new();
        polygon.push(Vec2::new(0.0, sh)); // Bottom Left

        for x in (0..sw as i32 + 10).step_by(10) {
            let xf = x as f32;
            // Sine wave combination
            let y = start_y
                + (xf * freq).sin() * amplitude
                + (xf * freq * 2.5).cos() * (amplitude * 0.3);
            polygon.push(Vec2::new(xf, y));
        }

        polygon.push(Vec2::new(sw, sh)); // Bottom Right

        // Fill area (Triangulation hack for Macroquad? simpler to just draw vertical lines or quads)
        // Macroquad doesn't have a fill_polygon easily. We'll use strips.
        // Or just `draw_triangle` loop
        for i in 1..polygon.len() - 2 {
            // Fan triangulation from Bottom Left
            draw_triangle(polygon[0], polygon[i], polygon[i + 1], color);
        }
    }

    fn draw_hill_mound(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        // Ellipse section
        let segments = 30;
        let _center = Vec2::new(x + w / 2.0, y + h); // Bottom center of ellipse?
                                                     // Actually drawing a filled ellipse is easier as a stretched circle

        // Using many triangles to form the mound
        for i in 0..segments {
            let t1 = (i as f32 / segments as f32) * std::f32::consts::PI; // 0 to PI (Arch)
            let t2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::PI;

            // Ellipse equation: x = a cos t, y = b sin t
            // We want the mound to be an arch on top of y (which is bottom)
            // So -cos(t) maps 0..PI to -1..1 (Left to right)

            let p1_x = x + w / 2.0 - (w / 2.0) * t1.cos();
            let p1_y = y + h - h * t1.sin();

            let p2_x = x + w / 2.0 - (w / 2.0) * t2.cos();
            let p2_y = y + h - h * t2.sin();

            // Triangle to bottom center (x+w/2, y+h)
            draw_triangle(
                Vec2::new(x + w / 2.0, y + h),
                Vec2::new(p1_x, p1_y),
                Vec2::new(p2_x, p2_y),
                color,
            );
        }
    }

    fn draw_gradient_rect(&self, x: f32, y: f32, w: f32, h: f32, c1: Color, c2: Color) {
        let steps = 20;
        let step_h = h / steps as f32;
        for i in 0..steps {
            let t = i as f32 / steps as f32;
            let c = Color::new(
                c1.r + (c2.r - c1.r) * t,
                c1.g + (c2.g - c1.g) * t,
                c1.b + (c2.b - c1.b) * t,
                1.0,
            );
            draw_rectangle(x, y + i as f32 * step_h, w, step_h + 1.0, c);
        }
    }

    fn draw_house(&self, x: f32, y: f32, scale: f32) {
        let w = 60.0 * scale;
        let h = 50.0 * scale;

        // Body (Cream/White)
        draw_rectangle(x, y, w, h, Color::new(0.95, 0.9, 0.8, 1.0));

        // Roof (Brown Triangle)
        let roof_overhang = 10.0 * scale;
        let roof_h = 35.0 * scale;
        draw_triangle(
            Vec2::new(x - roof_overhang, y),
            Vec2::new(x + w + roof_overhang, y),
            Vec2::new(x + w / 2.0, y - roof_h),
            Color::new(0.6, 0.3, 0.1, 1.0),
        );

        // Door
        let door_w = 15.0 * scale;
        let door_h = 25.0 * scale;
        draw_rectangle(
            x + w / 2.0 - door_w / 2.0,
            y + h - door_h,
            door_w,
            door_h,
            Color::new(0.4, 0.2, 0.0, 1.0),
        );

        // Window
        let win_size = 12.0 * scale;
        draw_rectangle(
            x + 8.0 * scale,
            y + 10.0 * scale,
            win_size,
            win_size,
            Color::new(0.5, 0.8, 1.0, 1.0),
        );
    }
}

// ---------------- Implementation of Sub-Elements ----------------

impl Cloud {
    fn new() -> Self {
        let mut puffs = Vec::new();
        // Generate 3-5 puffs for blobby cloud shape
        for _ in 0..4 {
            puffs.push((
                (fastrand::f32() - 0.5) * 40.0, // dx
                (fastrand::f32() - 0.5) * 15.0, // dy
                25.0 + fastrand::f32() * 15.0,  // scale
            ));
        }

        Self {
            x: fastrand::f32() * screen_width(),
            y: 20.0 + fastrand::f32() * 200.0,
            speed: 5.0 + fastrand::f32() * 10.0,
            scale: 0.8 + fastrand::f32() * 0.4,
            opacity: 0.8,
            puff_offsets: puffs,
        }
    }

    fn update(&mut self) {
        self.x += self.speed * get_frame_time();
        if self.x > screen_width() + 100.0 {
            self.x = -150.0;
            self.y = 20.0 + fastrand::f32() * 200.0;
        }
    }

    fn draw(&self) {
        let c_main = Color::new(1.0, 1.0, 1.0, self.opacity);
        let c_shadow = Color::new(0.9, 0.9, 0.95, self.opacity);

        for (dx, dy, r) in &self.puff_offsets {
            let px = self.x + dx * self.scale;
            let py = self.y + dy * self.scale;
            let radius = r * self.scale;

            // Slight shadow at bottom
            draw_circle(px, py + 5.0, radius, c_shadow);
            draw_circle(px, py, radius, c_main);
        }
    }
}

impl Ripple {
    fn new() -> Self {
        Self {
            t: fastrand::f32(),
            speed: 0.1 + fastrand::f32() * 0.2,
            offset_x: (fastrand::f32() - 0.5) * 2.0, // -1.0 to 1.0
        }
    }

    fn update(&mut self) {
        self.t += self.speed * get_frame_time();
        if self.t > 1.0 {
            self.t = 0.0;
            self.offset_x = (fastrand::f32() - 0.5) * 2.0;
        }
    }
}

impl Tree {
    fn new(x: f32, y: f32, scale: f32) -> Self {
        Self {
            x,
            y,
            scale,
            _variation: fastrand::i32(0..100),
            sway_phase: fastrand::f32() * 100.0,
        }
    }

    fn draw(&self, time: f32) {
        let sway = ((time * 2.0 + self.sway_phase).sin() * 2.0) * self.scale;

        let trunk_w = 20.0 * self.scale;
        let trunk_h = 70.0 * self.scale;

        // Trunk (Dark Brown, slightly tapered?)
        draw_rectangle(
            self.x - trunk_w / 2.0 + sway * 0.5,
            self.y - trunk_h,
            trunk_w,
            trunk_h,
            Color::new(0.4, 0.25, 0.1, 1.0),
        );

        // Foliage
        let blobs = [
            (0.0, -trunk_h - 10.0, 45.0),
            (-25.0, -trunk_h + 10.0, 35.0),
            (25.0, -trunk_h + 10.0, 35.0),
        ];

        let color_light = Color::new(0.2, 0.7, 0.3, 1.0);
        let color_shadow = Color::new(0.15, 0.55, 0.25, 1.0);

        for (dx, dy, r) in blobs.iter() {
            let cx = self.x + dx * self.scale + sway * (*dy / -100.0); // More sway at top
            let cy = self.y + dy * self.scale;
            let rad = r * self.scale;

            // Shadow
            draw_circle(cx, cy + 5.0 * self.scale, rad, color_shadow);
            // Light
            draw_circle(cx, cy, rad, color_light);
        }
    }
}
