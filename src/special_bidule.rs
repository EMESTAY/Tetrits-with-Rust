use crate::bidule::{BiduleType, Point};

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
        _ => None,
    }
}
