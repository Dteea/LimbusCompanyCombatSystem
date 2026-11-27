// Doney Tran
// 11/17/26
// CS423

use crate::entity::Entity;


/// This struct has components that make up a battle. An example battle starts like this:
/// We have Entities on the left side fighting against Entities on the right side. The battle will be keeping track of the game
/// state, applying logic such as ordering entities depending on speed, targetting behaviors, applying affects during combat, and
/// calculating the ordering of when each action should take place.
pub struct Battle {
    turn_count: u64,
    _left_team: Vec<Entity>,
    _right_team: Vec<Entity>,
    // Work ok targetting and redirection of skills later if possible
    //_targets: u64

}