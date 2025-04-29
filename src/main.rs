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
// I should probably have strength, defense, mag_strength, and mag_defense stats
struct Player {
    health: u64,
    class: PlayerClass,
    inventory: Vec<ItemType>,
    equipped_weapon: Option<Weapon>,
}

impl Player {
    fn new(health: u64, class: PlayerClass) -> Self {
        Self {
            health: health,
            class: class,
            inventory: Vec::new(),
            equipped_weapon: None,
        }
    }
    
    fn attack(&self, target: &mut impl Attackable) -> String {
        target.take_damage(10)
    }
    
    fn get_health(&self) -> u64 {
        self.health
    }
    
}

pub trait Attackable {
    fn take_damage(&mut self, damage: u64) -> String;
}

struct Enemy {
    name: String,
    health: u64,
}

impl Enemy {
    fn new(name: &str, health: u64) -> Self {
        Self {
            name: String::from(name),
            health: health,
        }
    }
}

impl Attackable for Enemy {
    fn take_damage(&mut self, damage: u64) -> String {
        self.health -= damage;
        format!("{} took {damage} points of damage.", &self.name)
    }
}

//I know it would be probably better to return an iterator instead of a vector of Strings, but I wrote this code right before reading that section in the book. I don't know quite yet how to output that iterator
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

//This is where the user input is going to be translated into the actions the player can take
//I actually might want to replace this with a iterator that runs through a list of available actions, so you can gain/lose available actions with different classes
fn parse_input(input: &Vec<String>) -> Result<String, ()>  {
    match input[0].as_str() {
        "attack" => Ok(String::from("You punch a wall")),
        _ => return Err(()),
        
    }
    
}

fn main() {
    let mut hero = Player::new(100, PlayerClass::Rouge);
    let mut goblin = Enemy::new("Bob", 100);
    
    hero.attack(&mut goblin);
    
    println!("{}", goblin.health)
    
    //let some_text: Vec<String> = get_input();
    //let result = parse_input(&some_text);
    //println!("You entered: {}", &result.unwrap());
}
