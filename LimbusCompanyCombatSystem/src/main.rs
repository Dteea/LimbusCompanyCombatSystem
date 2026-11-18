// Doney Tran
// 11/4/26
// CS423

mod entity;
mod battle;

fn main() {

    let character = entity::Entity::new("hello".to_string(),1,1,-32,1);
    character.display();
}
