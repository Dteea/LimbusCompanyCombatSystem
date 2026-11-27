// Doney Tran
// 11/17/26
// CS423

use crate::skills::*;
use rand::{RngExt, distr::Alphanumeric};

// Default constants to change the random range of generating a random entity with it's attributes. Change it however you like!
// May make this into inputs for program or have another function handle it.
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
#[derive(Debug, Clone)]
pub struct Entity {
    pub name: String,
    health: u64,
    stagger_health: u64,
    sanity: i64,
    pub speed: u64,
    pub skills: [Skill; 3],
}

// Stub for damage types, might add later if it isn't complicated. For now stick to only just regular damage. Traits may be good for this.

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "NAME: {}", self.name)?;
        writeln!(f, "HP: {}", self.health)?;
        writeln!(f, "STAGGER_HP: {}", self.stagger_health)?;
        writeln!(f, "SP: {}", self.sanity)?;
        writeln!(f, "SPEED: {}", self.speed)?;
        writeln!(f, "\nSKILLS:")?;

        for (index, skill) in self.skills.iter().enumerate() {
            writeln!(f, "SKILL: {} {}", index + 1, skill)?;
        }

        Ok(())
    }
}

impl Entity {
    // Construct Entity based on values passed in.
    pub fn new(
        name: String,
        health: u64,
        stagger_health: u64,
        sanity: i64,
        speed: u64,
        skills: [Skill; 3],
    ) -> Self {
        Self {
            name,
            health,
            stagger_health,
            sanity,
            speed,
            skills,
        }
    }

    /// This method can be used to generate a random entity with random attributes. You can tweak the ranges
    /// directly here in the method for your needs. Defaults are provided with the consts.
    pub fn new_random() -> Self {
        // Seeding stuff
        let rng = rand::rng();

        Self {
            // Might need to find a better way instead of cloning.
            // *Play around with clone and rand::rng to see which one is better
            // Create a random alphanumeric name that is 10 characters long with iterators!
            // Neat field init that I didn't know about so I don't have to create variables and then add them here, but I might mix and match
            // for code clarity if it needs to be
            name: rng
                .clone()
                .sample_iter(&Alphanumeric)
                // Can replace with better name generator or change the length of random string.
                .take(10)
                .map(char::from)
                .collect(),
            health: rng.clone().random_range(MIN_HP..=MAX_HP),
            stagger_health: rng.clone().random_range(MIN_STAGGER_HP..=MAX_STAGGER_HP),
            sanity: rng.clone().random_range(MIN_SANITY..=MAX_SANITY),
            speed: rng.clone().random_range(MIN_SPEED..=MAX_SPEED),
            skills: [
                Skill::new_random(),
                Skill::new_random(),
                Skill::new_random(),
            ],
        }
    }

    // Utilize enum types n error stuff later
    // *Make it take in a skill??
    /// This method takes in a damage source and subtracts it from the Entity's health and stagger health.
    pub fn take_damage(&mut self, health_damage: u64, stagger_damage: u64) {
        self.health = self.health.saturating_sub(health_damage);
        self.stagger_health = self.stagger_health.saturating_sub(stagger_damage);
    }

    /// This method takes in a damage source in the form of a skill and subtracts it from the Entity's health and stagger health.
    pub fn take_skill_damage(&mut self, skill: &Skill) {
        let _ = self.health.saturating_sub(skill.health_damage);
        let _ = self.health.saturating_sub(skill.stagger_damage);
    }
}
