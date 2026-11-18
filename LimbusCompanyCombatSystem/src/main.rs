// Doney Tran
// 11/4/26
// CS423


mod battle;
mod entity;
mod skills;

use crate::skills::*;

fn main() {
    let s1 = Skill::new("Turbo charge".to_string(), 10, 5);
    let t = s1.clone();
    let mut character = entity::Entity::new(
        "hello".to_string(),
        1,
        30,
        -32,
        1,
        [
            s1,
            Skill::new_random(),
            Skill::new_random(),
        ],
    );
    println!("{}", character);

    let mut c2 = entity::Entity::new_random();
    println!("{}", c2);

    c2.take_skill_damage(t);

    character.take_damage(10, 20);
    println!("{}", character);

    //c2.take_damage();
}
