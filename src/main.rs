enum ItemType {
    Item(Item),
    Weapon(Weapon),
}

//Ok, I'm gonna add a match later that will sort out all of functionality of these
//The thing I need to decide on though is whether I need to separate passive (constant score increase) and active (apply effect on weapon hit) attributes
enum PassiveAttribute {
    ArcaneFocus,
    Magical
}

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

impl Player{
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

fn main() {
    let hero = Player::new(100, PlayerClass::Rouge);

    println!("{}", hero.get_health());
}
