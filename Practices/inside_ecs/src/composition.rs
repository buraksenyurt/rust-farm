struct Position(f32, f32);
struct Velocity(f32, f32);
struct Health(f32);
struct Strength(f32);
struct Mana(f32);
struct Damage(f32);
struct Range(f32);

struct Warrior {
    id: String,
    position: Position,
    health: Health,
    strength: Strength,
}

struct Archer {
    id: String,
    position: Position,
    health: Health,
    damage: Damage,
    range: Range,
}

struct Healer {
    id: String,
    position: Position,
    health: Health,
    mana: Mana,
}

struct Villager {
    id: String,
    position: Position,
    health: Health,
}

struct Tower {
    position: Position,
    damage: Damage,
    range: Range,
}

fn attack(id: &str, strength: &Strength) {
    println!("{} attacks with {} strength", id, strength.0);
}

fn take_damage(health: &mut Health, amount: f32) {
    health.0 += amount;
}

fn shoot_arrows(name: &str, damage: &Damage, range: &Range) {
    println!(
        "{} shoots an arrow {} damage at range {}",
        name, damage.0, range.0
    );
}

fn heal(id: &str, mana: &Mana, target: &mut Health) {
    target.0 += mana.0;
    println!("{} heals with {} mana power", id, mana.0);
}

fn setup(position: &Position, damage: &Damage, range: &Range) {
    println!("Located")
}

pub fn run() {
    let mut warrior = Warrior {
        id: "Red Skull".to_string(),
        position: Position(10.0, 10.0),
        health: Health(100.0),
        strength: Strength(25.0),
    };

    let healer = Healer {
        id: "Athena".to_string(),
        position: Position(1.0, 2.0),
        health: Health(30f32),
        mana: Mana(20f32),
    };
    let archer = Archer {
        id: "Legolas".to_string(),
        position: Position(2.0, 0.0),
        health: Health(70.0),
        damage: Damage(40.0),
        range: Range(50.0),
    };

    attack(&warrior.id, &warrior.strength);
    heal(&healer.id, &healer.mana, &mut warrior.health);
    shoot_arrows(&archer.id, &archer.damage, &archer.range);
}
