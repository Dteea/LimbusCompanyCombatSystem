// Doney Tran
// 11/17/26
// CS423

use crate::entity::Entity;
use rand::RngExt;

const MAX_ENTITES: u64 = 5;

/// This struct has components that make up a battle. An example battle starts like this:
/// We have Entities on the left side fighting against Entities on the right side. The battle will be keeping track of the game
/// state, applying logic such as ordering entities depending on speed, targetting behaviors, applying affects during combat, and
/// calculating the ordering of when each action should take place.
/// Left is the main team where you will be controlling your characters.
#[derive(Debug, Clone)]
pub struct Battle {
    turn_count: u64,
    _left_team: Vec<Entity>,
    _right_team: Vec<Entity>,
    // Work ok targetting and redirection of skills later if possible
    //_targets: u64
    // move random function into here if implementing coins on skills.
}

impl Battle {
    pub fn _new(_left_team: Vec<Entity>, _right_team: Vec<Entity>) -> Self {
        Self {
            // Turn always start at 1
            turn_count: 1,
            _left_team,
            _right_team,
        }
    }
    pub fn _new_random() -> Self {
        let rng = rand::rng();

        Self {
            turn_count: 1,
            _left_team: vec![
                Entity::new_random();
                rng.clone().random_range(1..=MAX_ENTITES) as usize
            ],
            _right_team: vec![
                Entity::new_random();
                rng.clone().random_range(1..=MAX_ENTITES) as usize
            ],
        }
    }

    // TODO: Figure out how to pick which skill to use, and check
    // Suppose to use rng for coins, but for now keep simple
    // Maybe return the index of which entity won
    // Don't know if I should call clash a bunch of times or do it 
    // all at once, probably stick to the first
    //pub fn clash_team(&mut self) {todo!()}
    pub fn clash(left: &mut Entity, right: &mut Entity) {
        // For now test with skill 1
        // Clash logic
        // Left entity damage > Right Entity damage, Right takes damage and vice versa
        // A Tie keeps going until a coin rolls higher than other skill or clash reaches 99
        if left.skills[0].health_damage > right.skills[0].health_damage {
            right.take_skill_damage(&left.skills[0]);
        } else if left.skills[0].health_damage > right.skills[0].health_damage {
            left.take_skill_damage(&right.skills[0]);
        } else {
            todo!("make tie")
        }
    }

    pub fn start_actions(&self) /* -> u64*/
    {
        loop {



            // Last team standing wins the battle.
            if self._left_team.is_empty() {
                println!("You win!");
                break;
            } else if self._right_team.is_empty() {
                println!("You lose...");
                break;
            }
        }
    }
}
