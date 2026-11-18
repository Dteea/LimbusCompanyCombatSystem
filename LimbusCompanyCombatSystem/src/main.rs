// Doney Tran
// 11/4/26
// CS423

mod battle;
mod entity;

fn main() {
    let character = entity::Entity::new("hello".to_string(), 1, 1, -32, 1);
    character.display();

    let c2 = entity::Entity::new_random();
    c2.display();
}
