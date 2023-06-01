enum PlayerClass {
    Mage { magic_power: u32, slots: u32 },
    Warrior { attack_power: u32, stamina: u32 },
    Archer { accuracy: u32, arrows: u32 }
}

struct Player {
    name: String,
    hp_max: u32,
    hp_curr: u32,
    level: u32,
    class: PlayerClass
}

impl Player {
    fn new(name: String, hp_max: u32, hp_curr: u32, level: u32, class: PlayerClass) -> Self {
        Player {name, hp_max, hp_curr, level, class}
    }

    fn to_str(&self) -> String {
        match &self.class {
            PlayerClass::Warrior { attack_power, stamina } => {
                format!("Name: {}, hp: {}/{}, level: {}, class: Warrior, att_power: {}, stamina: {}", self.name, self.hp_curr, self.hp_max, self.level, attack_power, stamina)
            }
            PlayerClass::Mage { magic_power, slots } => {
                format!("Name: {}, hp: {}/{}, level: {}, class: Mage, magic_power: {}, slots: {}", self.name, self.hp_curr, self.hp_max, self.level, magic_power, slots)
            }
            PlayerClass::Archer { accuracy, arrows } => {
                format!("Name: {}, hp: {}/{}, level: {}, class: Archer, accuracy: {}, arrows: {}", self.name, self.hp_curr, self.hp_max, self.level, accuracy, arrows)
            }
        }
    }

    fn attack(&mut self) {
        match &mut self.class {
            PlayerClass::Warrior { attack_power, stamina } => {
                println!("Warrior attacks with a sword! Attack Power: {}, Stamina: {}", attack_power, stamina)
            }
            PlayerClass::Mage { magic_power, slots } => {
                println!("Mage casts a fireball! Magic Power: {}, Attunment Slots: {}", magic_power, slots)
            }
            PlayerClass::Archer { accuracy, arrows } => {
                if *arrows > 0 {
                    *arrows -= 1;
                    println!("Archer shoots an arrow! Accuracy: {}, Number of Arrows: {}", accuracy, arrows)
                } else {
                    println!("Archer is out of arrows!")
                }
            }
        }
    }
}

fn main() {

    let mut p1 = Player::new("Tarkus".to_string(), 100, 80, 42, PlayerClass::Warrior { attack_power: 71, stamina: 112 });
    let mut p2 = Player::new("Logan".to_string(), 70, 55, 70, PlayerClass::Mage { magic_power: 120, slots: 7 });
    let mut p3 = Player::new("Pharis".to_string(), 80, 62, 55, PlayerClass::Archer { accuracy: 150, arrows: 2 });

    println!("{}", p1.to_str());
    println!("{}", p2.to_str());
    println!("{}", p3.to_str());

    p1.attack();
    p2.attack();
    p3.attack();

    p3.attack();
    p3.attack();
    p3.attack();

}
