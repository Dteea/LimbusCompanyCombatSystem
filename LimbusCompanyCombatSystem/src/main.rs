// Doney Tran
// 11/4/26
// CS423

use crate::entity::Skill;

mod battle;
mod entity;

fn main() {
    let character = entity::Entity::new("hello".to_string(), 92, 30, -32, 1,
     [Skill::new_random(), Skill::new_random(), Skill::new_random()]);
    println!("{}", character);

    let c2 = entity::Entity::new_random();
    println!("{}", c2);

    c2.take_damage();
}
