use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BonusType {
    // Active (Special Pieces)
    Bomb,
    Chill,
    VerticalLaser,
    LiquidFiller,
    Drill,
    Anvil,

    TimeAnchor,    // Permanent Slow
    GoldenPickaxe, // +Score %
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
            Bonus {
                kind: BonusType::Anvil,
                name: "THE ANVIL",
                description: "Heavy block that COMPRESSES rows below it.",
                icon: "🪨",
                color: DARKGRAY,
                rarity: Rarity::Rare,
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
        fastrand::shuffle(&mut all);
        all.into_iter().take(count).collect()
    }
}

#[derive(Debug, Clone)]
pub struct ActiveBonus {
    pub kind: BonusType,
    pub timer: f32, // Duration remaining (if applicable)
}

pub fn resolve_bonus_on_lock(
    kind: BonusType,
    _grid: &mut crate::grid::Grid,
    _piece: &crate::bidule::Bidule,
    _particles: &mut Vec<crate::effects::Particle>,
    _effects: &mut Vec<crate::effects::ComicEffect>,
    _audio: &crate::sound_effects::AudioSystem,
    _screen_shake: &mut f32
) {
    match kind {
        // Active bonuses that modify next piece are handled in activate_bonus.
        // Mechanics are handled in resolve_special_mechanics_on_lock.
        
        // Relics might have on-lock effects?
        // Golden Pickaxe is score multiplier, handled in game.rs.
        // Time Anchor is gravity, handled in game.rs.
        
        // So this function is mostly unused for now, potentially useful for future "On Lock" relics.
        _ => {}
    }
}
