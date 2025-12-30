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
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug)]
pub struct Bidule {
    pub kind: BiduleType,
    pub positions: [Point; 4],
    pub color: Color,
    pub rotation_state: usize,
    pub pos: Point,
}

impl Bidule {
    pub fn new(kind: BiduleType) -> Self {
        let (positions, color) = match kind {
            BiduleType::I => (
                [
                    Point { x: 0, y: 1 },
                    Point { x: 1, y: 1 },
                    Point { x: 2, y: 1 },
                    Point { x: 3, y: 1 },
                ],
                COLOR_I,
            ),
            BiduleType::O => (
                [
                    Point { x: 1, y: 0 },
                    Point { x: 2, y: 0 },
                    Point { x: 1, y: 1 },
                    Point { x: 2, y: 1 },
                ],
                COLOR_O,
            ),
            BiduleType::T => (
                [
                    Point { x: 1, y: 0 },
                    Point { x: 0, y: 1 },
                    Point { x: 1, y: 1 },
                    Point { x: 2, y: 1 },
                ],
                COLOR_T,
            ),
            BiduleType::S => (
                [
                    Point { x: 1, y: 0 },
                    Point { x: 2, y: 0 },
                    Point { x: 0, y: 1 },
                    Point { x: 1, y: 1 },
                ],
                COLOR_S,
            ),
            BiduleType::Z => (
                [
                    Point { x: 0, y: 0 },
                    Point { x: 1, y: 0 },
                    Point { x: 1, y: 1 },
                    Point { x: 2, y: 1 },
                ],
                COLOR_Z,
            ),
            BiduleType::J => (
                [
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 1 },
                    Point { x: 1, y: 1 },
                    Point { x: 2, y: 1 },
                ],
                COLOR_J,
            ),
            BiduleType::L => (
                [
                    Point { x: 2, y: 0 },
                    Point { x: 0, y: 1 },
                    Point { x: 1, y: 1 },
                    Point { x: 2, y: 1 },
                ],
                COLOR_L,
            ),
        };

        Bidule {
            kind,
            positions,
            color,
            rotation_state: 0,
            pos: Point { x: 3, y: 0 },
        }
    }

    pub fn rotate(&mut self) {
        if self.kind == BiduleType::O {
            return;
        }

        let mut new_positions = self.positions;

        if self.kind == BiduleType::I {
            // I piece rotation (approximate)
            for i in 0..4 {
                let x = self.positions[i].x;
                let y = self.positions[i].y;
                new_positions[i].x = 3 - y;
                new_positions[i].y = x;
            }
        } else {
            // 3x3 rotation
            for i in 0..4 {
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
