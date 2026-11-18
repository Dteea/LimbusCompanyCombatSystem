// Doney Tran
// 11/17/26
// CS423

use rand::{RngExt, distr::Alphanumeric};
// Default constants to change the random range of generating a random entity with it's attributes. Change it however you like!
const MIN_SANITY: i64 = -45;
const MAX_SANITY: i64 = 45;

const MIN_HP: u64 = 1;
const MAX_HP: u64 = 200;

const MIN_STAGGER_HP: u64 = 30;
const MAX_STAGGER_HP: u64 = 50;

// Set to random number for now, but the gameplay loop will change it every turn in the battle based on rng.
const MIN_SPEED: u64 = 1;
const MAX_SPEED: u64 = 20;

/// This struct describes what an "Entity" is for this project. Most of the fields are unsigned and do not need negative values except for sanity
/// which is one of the unique mechanics.
pub struct Entity {
    name: String,
    health: u64,
    stagger_health: u64,
    sanity: i64,
    speed: u64,
}

// Stub for damage types, might add later if it isn't complicated. For now stick to only just regular damage.

impl Entity {
    // Construct Entity based on values passed in.
    pub fn new(name: String, health: u64, stagger_health: u64, sanity: i64, speed: u64) -> Self {
        Self {
            name,
            health,
            stagger_health,
            sanity,
            speed,
        }
    }

    /// This method can be used to generate a random entity with random attributes. You can tweak the ranges 
    /// directly here in the method for your needs.
    pub fn new_random() -> Self {
        // Seeding stuff
        let rng = rand::rng();

        Self {
            // *Play around with clone and rand::rng to see which one is better
            // Create a random alphanumeric name that is 10 characters long with iterators!
            name: rng
                .clone()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect(),
            health: rng.clone().random_range(MIN_HP..=MAX_HP),
            stagger_health: rng.clone().random_range(MIN_STAGGER_HP..=MAX_STAGGER_HP),
            sanity: rng.clone().random_range(MIN_SANITY..=MAX_SANITY),
            speed: rng.clone().random_range(MIN_SPEED..=MAX_SPEED),
        }
    }

    /// This method displays all the values of a entity and does not return anything.
    pub fn display(&self) {
        println!(
            "
    NAME: {} 
    HP: {} 
    STAGGER_HP: {}
    SP: {}
    SPEED: {}",
            self.name, self.health, self.stagger_health, self.sanity, self.speed
        );
    }
}
