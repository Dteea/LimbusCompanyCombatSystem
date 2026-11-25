// Doney Tran
// 11/17/26
// CS423

// I love random*
use rand::{RngExt, distr::Alphanumeric};

const MIN_DAMAGE: u64 = 5;
const MAX_DAMAGE: u64 = 15;

const MIN_STAGGER_DAMAGE: u64 = 1;
const MAX_STAGGER_DAMAGE: u64 = 10;

// Option to move into it's own module if I really want it.
// *DEF move it into another file!
#[derive(Debug, Clone)]
pub struct Skill {
    name: String,
    pub health_damage: u64,
    pub stagger_damage: u64,
}


// Pretty Print with colors in terminal for the damage!
impl std::fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NAME: {} DMG: {}, STAGG_DMG: {}",
            self.name, self.health_damage, self.stagger_damage
        )
    }
}

impl Skill {
    pub fn new(name: String, health_damage: u64, stagger_damage: u64) -> Self {
        Self {
            name,
            health_damage,
            stagger_damage,
        }
    }

    pub fn new_random() -> Self {
        let rng = rand::rng();

        Self {
            name: rng
                .clone()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect(),
            health_damage: rng.clone().random_range(MIN_DAMAGE..=MAX_DAMAGE),
            stagger_damage: rng
                .clone()
                .random_range(MIN_STAGGER_DAMAGE..=MAX_STAGGER_DAMAGE),
        }
    }
}