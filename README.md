This project is meant to be a recreation of some of the aspects of gameplay, mainly the combat in Limbus Company, written in Rust.
I plan on splitting this into two parts because I don't know how much I'll get done, most likely I will focus only on Part 1.

## Part 1 Goals
- Create a "battle" which captures and manages the game state, who attacks what, attack order, win conditions
- Add Entities that have properities such as hp, stagger, sanity, etc. 
- Make a random entity generator for testing
- Basic display to see current state and user interaction to progress the battle
- Combine and intergrate all to create a battle!
- **If time allows, create simple AI logic for targeting**

## Part 2 Future
- try out and learn the Bevy Engine
- move all displays to Bevy to let it handle rendering. 
- move logic to ECS system that Bevy recommends
- Create 2d battle scene with properties like hp rendered for each entity