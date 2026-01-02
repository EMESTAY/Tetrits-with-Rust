use macroquad::prelude::*;
use std::f32::consts::PI;

/// Helper to draw a rounded rectangle
pub fn draw_rounded_rect(x: f32, y: f32, w: f32, h: f32, r: f32, color: Color) {
    if r <= 0.0 {
        draw_rectangle(x, y, w, h, color);
        return;
    }
    let r = f32::min(r, f32::min(w, h) / 2.0);

    // 1. Center Body (Full Width, Vertical Middle)
    draw_rectangle(x, y + r, w, h - 2.0 * r, color);

    draw_rectangle(x + r, y, w - 2.0 * r, r, color);

    draw_rectangle(x + r, y + h - r, w - 2.0 * r, r, color);
    let draw_sector = |cx: f32, cy: f32, radius: f32, start_angle: f32, end_angle: f32| {
        let segments = 12;
        let step = (end_angle - start_angle) / segments as f32;

        for i in 0..segments {
            let a1 = start_angle + step * i as f32;
            let a2 = start_angle + step * (i + 1) as f32;

            draw_triangle(
                Vec2::new(cx, cy),
                Vec2::new(cx + a1.cos() * radius, cy + a1.sin() * radius),
                Vec2::new(cx + a2.cos() * radius, cy + a2.sin() * radius),
                color,
            );
        }
    };

    draw_sector(x + r, y + r, r, PI, PI * 1.5);
    draw_sector(x + w - r, y + r, r, PI * 1.5, PI * 2.0);
    draw_sector(x + w - r, y + h - r, r, 0.0, PI * 0.5);
    draw_sector(x + r, y + h - r, r, PI * 0.5, PI);
}

/// Helper to draw a rectangle with a hardware-accelerated vertex gradient
pub fn _draw_mesh_gradient_rect(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    c1: Color,
    c2: Color,
    is_vertical: bool,
) {
    let (v1_col, v2_col, v3_col, v4_col) = if is_vertical {
        (c1, c1, c2, c2)
    } else {
        (c1, c2, c1, c2)
    };

    let vertices = [
        Vertex {
            position: Vec3::new(x, y, 0.0),
            uv: Vec2::ZERO,
            color: v1_col.into(),
            normal: Vec4::new(0.0, 0.0, 1.0, 0.0),
        },
        Vertex {
            position: Vec3::new(x + w, y, 0.0),
            uv: Vec2::ZERO,
            color: v2_col.into(),
            normal: Vec4::new(0.0, 0.0, 1.0, 0.0),
        },
        Vertex {
            position: Vec3::new(x, y + h, 0.0),
            uv: Vec2::ZERO,
            color: v3_col.into(),
            normal: Vec4::new(0.0, 0.0, 1.0, 0.0),
        },
        Vertex {
            position: Vec3::new(x + w, y + h, 0.0),
            uv: Vec2::ZERO,
            color: v4_col.into(),
            normal: Vec4::new(0.0, 0.0, 1.0, 0.0),
        },
    ];

    let indices = [0, 1, 2, 1, 2, 3];

    let mesh = Mesh {
        vertices: vertices.to_vec(),
        indices: indices.to_vec(),
        texture: None,
    };

    draw_mesh(&mesh);
}
