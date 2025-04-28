use std::io;

enum ItemType {
    Item(Item),
    Weapon(Weapon),
}

//Ok, I'm gonna add a match later that will sort out all of functionality of these
enum PassiveAttribute {
    ArcaneFocus,
    Magical
}

//I'm going to need to figure out how these attributes are going to be activated
enum ActiveAttribute {
    Crippling, //My idea for this one is that there is a small chance that you cripple the enemy (applying the crippled effect) which than permanently greatly decreases their damage
}

struct Item {
    name: String,
    description: String,
    attributes: Vec<PassiveAttribute>,
    value: u64,
}

struct Weapon {
    name: String,
    description: String,
    damage: u64,
    attributes: Vec<ActiveAttribute>,
    value: u64,
}

enum PlayerClass {
    Fighter,
    Paladin,
    Rouge,
    Wizard,
    Warlock,
}

//Maybe I should have strength modifier (for enemies too) that multiplies with the damage of the weapon
struct Player {
    health: u64,
    class: PlayerClass,
    inventory: Vec<ItemType>,
}

impl Player {
    fn new(health: u64, class: PlayerClass) -> Self {
        Self {
            health: health,
            class: class,
            inventory: Vec::new(),
        }
    }
    
    fn get_health(&self) -> u64 {
        self.health
    }
}

fn get_input() -> Vec<String> {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read Stdin");
    input = input.to_lowercase();
    let input: Vec<&str> = input
        .trim()
        .split(' ')
        .collect();
    
    input.iter().map(|&s| s.into()).collect()
}

fn main() {
    let hero = Player::new(100, PlayerClass::Rouge);
    
    let some_text: Vec<String> = get_input();
    dbg!("You entered: {}", some_text);
}
