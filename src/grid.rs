use crate::bidule::Bidule;
use crate::constants::*;
use macroquad::prelude::*;

pub struct Grid {
    pub cells: [[Option<Color>; GRID_WIDTH]; GRID_HEIGHT],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cells: [[None; GRID_WIDTH]; GRID_HEIGHT],
        }
    }

    pub fn is_collision(&self, piece: &Bidule) -> bool {
        for p in piece.positions.iter() {
            let x = piece.pos.x + p.x;
            let y = piece.pos.y + p.y;

            if x < 0 || x >= GRID_WIDTH as i32 || y >= GRID_HEIGHT as i32 {
                return true;
            }

            if y < 0 {
                continue;
            }
            if self.cells[y as usize][x as usize].is_some() {
                return true;
            }
        }
        false
    }

    pub fn lock_piece(&mut self, piece: &Bidule) {
        for p in piece.positions.iter() {
            let x = (piece.pos.x + p.x) as usize;
            let y = (piece.pos.y + p.y) as usize;

            if x < GRID_WIDTH && y < GRID_HEIGHT {
                self.cells[y][x] = Some(piece.color);
            }
        }
    }

    pub fn clear_lines(&mut self) -> i32 {
        let mut lines_cleared = 0;
        let mut y = GRID_HEIGHT - 1;

        while y > 0 {
            let mut full_line = true;
            for x in 0..GRID_WIDTH {
                if self.cells[y][x].is_none() {
                    full_line = false;
                    break;
                }
            }

            if full_line {
                lines_cleared += 1;

                for row in (1..=y).rev() {
                    self.cells[row] = self.cells[row - 1];
                }
                self.cells[0] = [None; GRID_WIDTH];
            } else {
                y -= 1;
            }
        }
        lines_cleared
    }
}
