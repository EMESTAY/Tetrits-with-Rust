use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BonusType {
    // Active (One-time or Short Duration)
    Bomb,
    Chill,
    VerticalLaser,
    LiquidFiller,
    Drill,
    
    // Relics (Passive / Permanent)
    TimeAnchor,    // Permanent Slow
    GoldenPickaxe, // +Score %
    VolatileGrid,  // Random Explosions
    LifeInsurance, // Revive
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rarity {
    Common,
    Rare,
    Legendary,
}

#[derive(Debug, Clone)]
pub struct Bonus {
    pub kind: BonusType,
    pub name: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    pub color: Color,
    pub rarity: Rarity,
}

impl Bonus {
    pub fn get_all() -> Vec<Bonus> {
        vec![
            // COMMONS (Active)
            Bonus {
                kind: BonusType::Bomb,
                name: "BOMB BLOCK",
                description: "Explodes on impact, clearing a 3x3 area.",
                icon: "💣",
                color: RED,
                rarity: Rarity::Common,
            },
            Bonus {
                kind: BonusType::Chill,
                name: "CHILL TIME",
                description: "Slows time by 50% for 60 seconds.",
                icon: "❄️",
                color: SKYBLUE,
                rarity: Rarity::Common,
            },
            Bonus {
                kind: BonusType::VerticalLaser,
                name: "LASER BEAM",
                description: "Clears the entire column on lock.",
                icon: "⚡",
                color: YELLOW,
                rarity: Rarity::Common,
            },
            Bonus {
                kind: BonusType::LiquidFiller,
                name: "JELLY BIDULE",
                description: "Drops 6 liquid blocks that fill the lowest gaps.",
                icon: "🍮",
                color: Color::new(0.0, 1.0, 1.0, 1.0), // Cyan/Aqua liquid
                rarity: Rarity::Common,
            },
            Bonus {
                kind: BonusType::Drill,
                name: "DRILL",
                description: "Next piece smashes through blocks to the bottom.",
                icon: "🔨",
                color: ORANGE,
                rarity: Rarity::Common,
            },

            // RARE (Relics)
            Bonus {
                kind: BonusType::TimeAnchor,
                name: "TIME ANCHOR",
                description: "Passively slows gravity by 10%. Stacks.",
                icon: "⚓",
                color: GOLD,
                rarity: Rarity::Rare,
            },
            Bonus {
                kind: BonusType::GoldenPickaxe,
                name: "GOLD PICKAXE",
                description: "+20% Score gained from lines. Stacks.",
                icon: "⛏️",
                color: GOLD,
                rarity: Rarity::Rare,
            },

            // LEGENDARY
            Bonus {
                kind: BonusType::VolatileGrid,
                name: "VOLATILE GRID",
                description: "Cleared lines have a 10% chance to EXPLODE.",
                icon: "☢️",
                color: MAGENTA,
                rarity: Rarity::Legendary,
            },
            Bonus {
                kind: BonusType::LifeInsurance,
                name: "LIFE INSURANCE",
                description: "Prevents Game Over once. Consumable.",
                icon: "💖",
                color: PINK,
                rarity: Rarity::Legendary,
            },
        ]
    }

    pub fn get_random_set(count: usize) -> Vec<Bonus> {
        let mut all = Self::get_all();
        // Weights? For now just shuffle.
        // Maybe ensure at least one Common?
        // Let's just shuffle for pure RNG chaos.
        fastrand::shuffle(&mut all);
        all.into_iter().take(count).collect()
    }
}

#[derive(Debug, Clone)]
pub struct ActiveBonus {
    pub kind: BonusType,
    pub timer: f32, // Duration remaining (if applicable)
}
