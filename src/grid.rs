use crate::bidule::Bidule;
use crate::constants::*;
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub color: Color,
    pub bubble_seed: usize,
}

pub struct Grid {
    pub cells: [[Option<Cell>; GRID_WIDTH]; GRID_HEIGHT],
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
                // Ghost pieces ignore block collisions (Telefrag logic)

                return true;
            }
        }
        false
    }

    pub fn lock_piece(&mut self, piece: &Bidule) {
        for (i, p) in piece.positions.iter().enumerate() {
            let x = (piece.pos.x + p.x) as usize;
            let y = (piece.pos.y + p.y) as usize;

            if x < GRID_WIDTH && y < GRID_HEIGHT {
                self.cells[y][x] = Some(Cell {
                    color: piece.color,
                    bubble_seed: piece.seeds[i],
                });
            }
        }
    }

    pub fn clear_lines(&mut self) -> Vec<usize> {
        let mut cleared_rows = Vec::new();

        // 1. Identify full lines
        for y in 0..GRID_HEIGHT {
            let full_line = (0..GRID_WIDTH).all(|x| self.cells[y][x].is_some());
            if full_line {
                cleared_rows.push(y);
            }
        }

        // 2. Compact grid (if needed)
        if !cleared_rows.is_empty() {
            let mut new_cells = [[None; GRID_WIDTH]; GRID_HEIGHT];
            let mut target_y = GRID_HEIGHT - 1;

            for src_y in (0..GRID_HEIGHT).rev() {
                if !cleared_rows.contains(&src_y) {
                    new_cells[target_y] = self.cells[src_y];
                    if target_y > 0 {
                        target_y -= 1;
                    }
                }
            }
            // Fill remaining top rows with None (already done by init, but implicit here because target_y stops)
            self.cells = new_cells;
        }

        cleared_rows
    }
}
