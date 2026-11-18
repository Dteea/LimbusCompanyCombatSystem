// Doney Tran
// 11/17/26
// CS423


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
    pub fn new(name: String, health: u64, stagger_health: u64, sanity:i64, speed: u64) -> Self {
        Self {name, health, stagger_health, sanity, speed}
    }

    pub fn random() {
        todo!()
    }

    pub fn display(&self) {
        println!("NAME: {} HP: {} STAGGER_HP: {} SP: {} SPEED: {}", self.name, self.health, self.stagger_health, self.sanity, self.speed);
    }

}
