use crate::constants::*;
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BiduleType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
    Plus,
    Jelly,
    Bomb,
    Laser,
    Drill,
    Anvil,
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug)]
pub struct Bidule {
    pub kind: BiduleType,
    pub positions: Vec<Point>,
    pub color: Color,
    pub rotation_state: usize,
    pub pos: Point,
    pub seeds: Vec<usize>,
}

impl Bidule {
    pub fn new(kind: BiduleType) -> Self {
        // Try to get properties from special_bidule (Jelly, Bomb)
        let (positions, color) = if let Some(props) = crate::special_bidule::get_special_bidule_properties(kind) {
            props
        } else {
            // Fallback to standard blocks
            match kind {
                BiduleType::I => (
                    vec![Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 }, Point { x: 3, y: 1 }],
                    COLOR_I,
                ),
                BiduleType::O => (
                    vec![Point { x: 1, y: 0 }, Point { x: 2, y: 0 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 }],
                    COLOR_O,
                ),
                BiduleType::T => (
                    vec![Point { x: 1, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 }],
                    COLOR_T,
                ),
                BiduleType::S => (
                    vec![Point { x: 1, y: 0 }, Point { x: 2, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }],
                    COLOR_S,
                ),
                BiduleType::Z => (
                    vec![Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 }],
                    COLOR_Z,
                ),
                BiduleType::J => (
                    vec![Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 }],
                    COLOR_J,
                ),
                BiduleType::L => (
                    vec![Point { x: 2, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 }],
                    COLOR_L,
                ),
                BiduleType::Plus => (
                    vec![
                        Point { x: 1, y: 0 }, // Top
                        Point { x: 0, y: 1 }, // Left
                        Point { x: 1, y: 1 }, // Center
                        Point { x: 2, y: 1 }, // Right
                        Point { x: 1, y: 2 }, // Bottom
                    ],
                    COLOR_PLUS,
                ),
                // Should be covered by special_bidule, but exhaustiveness check might need this or default

                _ => (vec![], WHITE), 
            }
        };

        // Standard initialization...
        let mut seeds = Vec::with_capacity(positions.len());
        for _ in 0..positions.len() {
            // Random seeds. Use 0 for "no bubbles" (20% chance?).
            if fastrand::f32() < 0.2 {
                seeds.push(0);
            } else {
                seeds.push(fastrand::usize(1..10000));
            }
        }

        // Center on grid (width 12 -> center ~6, so start at 4)
        // GRID_WIDTH 12 / 2 - 2 = 4.
        let start_x = (GRID_WIDTH / 2) as i32 - 2;

        Bidule {
            kind,
            positions,
            color,
            rotation_state: 0,
            pos: Point { x: start_x, y: 0 },
            seeds,
        }
    }

    pub fn rotate(&mut self) {
        if self.kind == BiduleType::O || self.kind == BiduleType::Jelly {
            return;
        }

        let mut new_positions = self.positions.clone();

        if self.kind == BiduleType::I {
            // I piece rotation (approximate) - 4x4
            for i in 0..self.positions.len() {
                let x = self.positions[i].x;
                let y = self.positions[i].y;
                new_positions[i].x = 3 - y;
                new_positions[i].y = x;
            }
        } else {
            // 3x3 rotation - Works for Plus too (centered at 1,1)
            for i in 0..self.positions.len() {
                let x = self.positions[i].x;
                let y = self.positions[i].y;
                new_positions[i].x = 2 - y;
                new_positions[i].y = x;
            }
        }

        self.positions = new_positions;
        self.rotation_state = (self.rotation_state + 1) % 4;
    }
}
