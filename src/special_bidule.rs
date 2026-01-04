use crate::bidule::{Bidule, BiduleType, Point};
use macroquad::prelude::*;

/// Returns the shape (positions) and color for special bidules (Jelly, Bomb).
/// Returns None for standard bidules.
pub fn get_special_bidule_properties(kind: BiduleType) -> Option<(Vec<Point>, Color)> {
    match kind {
        BiduleType::Jelly => Some((
            // 3x2 solid rectangle "Blob"
            vec![
                Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 2, y: 0 },
                Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 },
            ],
            // Pink #fb6f92
            Color::new(0.984, 0.435, 0.572, 1.0),
        )),
        BiduleType::Bomb => Some((
            // Diamond shape (Cross/Plus shape 5 blocks)
            //  X
            // XXX
            //  X
            vec![
                                      Point { x: 1, y: 0 },
                Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 },
                                      Point { x: 1, y: 2 },
            ],
            // Lava Orange #f35b04
            Color::new(0.952, 0.356, 0.015, 1.0),
        )),
        BiduleType::Laser => Some((
            // Thunder / Lightning Bolt shape
            //   X
            //  XX
            //   X
            //  XX
            vec![
                Point { x: 1, y: 0 },
                Point { x: 0, y: 1 }, Point { x: 1, y: 1 },
                Point { x: 1, y: 2 },
                Point { x: 0, y: 3 }, Point { x: 1, y: 3 },
            ],
            // Electric Yellow/White
            Color::new(1.0, 1.0, 0.8, 1.0),
        )),
        BiduleType::Drill => Some((
            // Cone / Drill shape
            // XXX
            //  X 
            //  X
            vec![
                Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 2, y: 0 },
                Point { x: 1, y: 1 },
                Point { x: 1, y: 2 },
            ],
            // Brown / Bronze
            Color::new(0.55, 0.27, 0.07, 1.0),
        )),
        BiduleType::Anvil => Some((
            // Heavy Anvil Shape
            // ****
            //  **
            //  **
            vec![
                Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 2, y: 0 }, Point { x: 3, y: 0 },
                                      Point { x: 1, y: 1 }, Point { x: 2, y: 1 },
                                      Point { x: 1, y: 2 }, Point { x: 2, y: 2 },
            ],
            // Iron / Dark Gray (Steel)
            Color::new(0.3, 0.3, 0.35, 1.0),
        )),

        _ => None,
    }
}

/// Resolves the "Sand" physics for Jelly blocks.
/// Iteratively moves blocks down if there is empty space below them.
pub fn resolve_jelly_physics(grid: &mut crate::grid::Grid) {
    let mut settling = true;
    let mut iterations = 0;
    while settling && iterations < 20 { // Limit iterations to prevent freezing
        settling = false;
        iterations += 1;
        
        // Scan from bottom up, left to right
        for y in (0..crate::constants::GRID_HEIGHT-1).rev() { // Start from second to last row
            for x in 0..crate::constants::GRID_WIDTH {
                if grid.cells[y][x].is_some() {
                    // Check if this specific cell is "Jelly" (color Pink)? 
                    // Pink #fb6f92 -> (0.984, 0.435, 0.572, 1.0)
                    let cell = grid.cells[y][x].unwrap();
                    let is_jelly = cell.color.r > 0.9 && cell.color.g > 0.4 && cell.color.g < 0.5 && cell.color.b > 0.5; // Robust range check for Pink
                    
                    if is_jelly {
                        // Check directly below
                        if grid.cells[y+1][x].is_none() {
                            // Fall!
                            grid.cells[y+1][x] = grid.cells[y][x];
                            grid.cells[y][x] = None;
                            settling = true;
                        } else {
                            // Try side ways (Sand/Liquid Physics)
                            // Randomized to avoid bias
                            let mut moves = Vec::new();
                            if x > 0 && grid.cells[y+1][x-1].is_none() && grid.cells[y][x-1].is_none() {
                                moves.push(-1);
                            }
                            if x < crate::constants::GRID_WIDTH - 1 && grid.cells[y+1][x+1].is_none() && grid.cells[y][x+1].is_none() {
                                moves.push(1);
                            }
                            
                            if !moves.is_empty() {
                                let dir = moves[fastrand::usize(..moves.len())];
                                let nx = (x as i32 + dir) as usize;
                                grid.cells[y+1][nx] = grid.cells[y][x];
                                grid.cells[y][x] = None;
                                settling = true;
                            }
                        } 
                    }
                }
            }
        }
    }
}

pub fn resolve_special_mechanics_on_lock(
    grid: &mut crate::grid::Grid,
    piece: &Bidule,
    particles: &mut Vec<crate::effects::Particle>,
    audio: &crate::sound_effects::AudioSystem,
    screen_shake: &mut f32
) {
    match piece.kind {
        BiduleType::Bomb => {
            // Explode 3x3 around each block of the locked piece
            for p in piece.positions.iter() {
                let cx = piece.pos.x + p.x;
                let cy = piece.pos.y + p.y;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = cx + dx;
                        let ny = cy + dy;
                        if nx >= 0 && nx < crate::constants::GRID_WIDTH as i32 && ny >= 0 && ny < crate::constants::GRID_HEIGHT as i32 {
                            grid.cells[ny as usize][nx as usize] = None;
                            // Visual
                            for _ in 0..5 {
                                particles.push(crate::effects::Particle::new(
                                    (nx as f32 * crate::constants::BLOCK_SIZE) + crate::constants::BLOCK_SIZE/2.0, 
                                    (ny as f32 * crate::constants::BLOCK_SIZE) + crate::constants::BLOCK_SIZE/2.0, 
                                    RED, 
                                    crate::effects::ParticleType::Explosion
                                ));
                            }
                        }
                    }
                }
            }
            *screen_shake = 30.0;
            audio.play_tetris();
        }
        BiduleType::Laser => {
            // Clear columns occupied by the piece
            let mut cols = std::collections::HashSet::new();
            for p in piece.positions.iter() {
                cols.insert(piece.pos.x + p.x);
            }
            for c in cols {
                 if c >= 0 && c < crate::constants::GRID_WIDTH as i32 {
                    for y in 0..crate::constants::GRID_HEIGHT {
                        grid.cells[y][c as usize] = None;
                        // Sparks along the beam
                        if fastrand::f32() < 0.3 {
                            particles.push(crate::effects::Particle::new(
                                (c as f32 * crate::constants::BLOCK_SIZE) + crate::constants::BLOCK_SIZE/2.0, 
                                (y as f32 * crate::constants::BLOCK_SIZE) + crate::constants::BLOCK_SIZE/2.0, 
                                crate::constants::COLOR_YELLOW, 
                                crate::effects::ParticleType::Spark
                            ));
                        }
                    }
                 }
            }
            *screen_shake = 10.0;
            audio.play_tetris();
        }
        BiduleType::Drill => {
            // Clear columns BELOW the piece positions
            for p in piece.positions.iter() {
                let cx = piece.pos.x + p.x;
                let cy = piece.pos.y + p.y;
                if cx >= 0 && cx < crate::constants::GRID_WIDTH as i32 {
                     for y in cy..crate::constants::GRID_HEIGHT as i32 {
                         if y >= 0 {
                             grid.cells[y as usize][cx as usize] = None;
                         }
                     }
                      // Drill particles
                      if fastrand::f32() < 0.5 {
                        let y_start = cy as f32;
                        particles.push(crate::effects::Particle::new(
                            (cx as f32 * crate::constants::BLOCK_SIZE) + crate::constants::BLOCK_SIZE/2.0, 
                            (y_start * crate::constants::BLOCK_SIZE) + crate::constants::BLOCK_SIZE/2.0, 
                            crate::constants::COLOR_ORANGE, 
                            crate::effects::ParticleType::Spark
                        ));
                    }
                }
            }
            *screen_shake = 10.0;
        }
        BiduleType::Anvil => {
             // COMPRESSOR / COMPACTOR LOGIC
             // Rigid Anvil: Squishes, then moves down as a WHOLE UNIT. No deformation.
             
             // 1. Identify affected columns and Anvil's bottom Y in each
             let mut cols = std::collections::HashSet::new();
             let mut col_bottoms = std::collections::HashMap::new();
             
             for p in piece.positions.iter() {
                 let x = piece.pos.x + p.x;
                 let y = piece.pos.y + p.y;
                 cols.insert(x);
                 
                 let current_max = *col_bottoms.get(&x).unwrap_or(&0);
                 if y > current_max {
                     col_bottoms.insert(x, y);
                 }
                 // Ensure we at least track it
                 col_bottoms.entry(x).or_insert(y);
             }

             // 2. Perform Compression per column and calculate "Gap" (Potential Drop)
             let mut gaps = Vec::new();
             let mut processed_cols = Vec::new();

             for &c in &cols {
                 if c >= 0 && c < crate::constants::GRID_WIDTH as i32 {
                     let col = c as usize;
                     let bottom_y = *col_bottoms.get(&c).unwrap_or(&0) as usize;
                     let start_scan_y = bottom_y + 1;
                     
                     // Collect blocks strictly BELOW the anvil in this column
                     let mut stack_below = Vec::new();
                     for y in start_scan_y..crate::constants::GRID_HEIGHT {
                         if let Some(cell) = grid.cells[y][col] {
                             stack_below.push(cell);
                             grid.cells[y][col] = None; // Clear strictly below
                         }
                     }
                     

                     
                     // Compress (Merge pairs)
                     let mut compressed_stack = Vec::new();
                     let mut i = 0;
                     while i < stack_below.len() {
                         if i + 1 < stack_below.len() {
                             compressed_stack.push(stack_below[i+1]); // Keep bottom
                             i += 2;
                         } else {
                             compressed_stack.push(stack_below[i]);
                             i += 1;
                         }
                     }
                     
                     let new_height = compressed_stack.len();
                     // Calculate Clearance (Distance to new stack top)
                     // Top of new stack is at `GRID_HEIGHT - new_height`.
                     // Target Y for Anvil Bottom is `GRID_HEIGHT - new_height - 1`.
                     // Current Anvil Bottom is `bottom_y`.
                     let clearance = (crate::constants::GRID_HEIGHT - new_height - 1).saturating_sub(bottom_y);
                     gaps.push(clearance);
                     
                     // Repack compressed stack at the bottom of the grid
                     let mut y_cursor = crate::constants::GRID_HEIGHT - 1;
                     for cell in compressed_stack.into_iter().rev() {
                         grid.cells[y_cursor][col] = Some(cell);
                         if y_cursor > 0 { y_cursor -= 1; }
                     }
                     
                     processed_cols.push(col);
                 }
             }

             // 3. Determine Rigid Drop Amount (Min of all gaps)
             // If ANY column had resistance (gap=0), the Anvil cannot move.
             // If all columns had gaps, we move by the smallest gap.
             let drop_amount = if gaps.is_empty() { 0 } else { *gaps.iter().min().unwrap_or(&0) };
             
             // 4. Shift Anvil (and above) down by drop_amount
             if drop_amount > 0 {
                 // We need to move everything from [0..=anvil_bottom_y] down.
                 // We must do this for ALL columns affected by the anvil.
                 // To be safe, we should process affected columns.
                 
                 for &col in &processed_cols {
                      let bottom_y = *col_bottoms.get(&(col as i32)).unwrap_or(&0) as usize;
                      
                      // Move blocks down from bottom_y upwards to 0
                      // We are moving them into the empty space created by compression (which is strictly below bottom_y).
                      // The space available is at least `drop_amount` (since it's the min).
                      // But wait, we repacked the bottom stack.
                      // The empty space starts at `GRID_HEIGHT - new_height - 1` and goes up.
                      // Our Anvil is at `bottom_y`.
                      // We simply want to shift `grid[y][col]` to `grid[y + drop_amount][col]`.
                      
                      // Iterate backwards from bottom_y to 0
                      for y in (0..=bottom_y).rev() {
                          if let Some(cell) = grid.cells[y][col] {
                              grid.cells[y][col] = None;
                              // Target: y + drop_amount.
                              // Check bounds just in case, though logic implies safety.
                              let target_y = y + drop_amount;
                              if target_y < crate::constants::GRID_HEIGHT {
                                  grid.cells[target_y][col] = Some(cell);
                              }
                          }
                      }
                 }
             }

             *screen_shake = 15.0; 
             audio.play_anvil();
        }

        _ => {}
    }
}
