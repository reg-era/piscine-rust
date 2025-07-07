#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub strength: f64,
    pub score: i32,
    pub money: i32,
    pub weapons: Vec<String>,
}

pub struct Fruit {
    pub weight_in_kg: f64,
}

pub struct Meat {
    pub weight_in_kg: f64,
    pub fat_content: f64,
}

impl Player {
    pub fn eat(&mut self, food: impl Food) {
        self.strength += food.gives();
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let formated = format!(
            "{}\nStrength: {}, Score: {}, Money: {}\nWeapons: {:?}",
            self.name, self.strength, self.score, self.money, self.weapons
        );
        write!(f, "{}", formated)
    }
}

pub trait Food {
    fn gives(&self) -> f64;
}

impl Food for Fruit {
    fn gives(&self) -> f64 {
        self.weight_in_kg * 4.
    }
}

impl Food for Meat {
    fn gives(&self) -> f64 {
        let fac = self.fat_content;
        let wei = self.weight_in_kg;
        ((wei * fac) * 9.) + ((wei * (1. - fac)) * 4.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let apple = Fruit { weight_in_kg: 1.0 };

        println!("this apple gives {} units of strength", apple.gives());

        let steak = Meat {
            weight_in_kg: 1.0,
            fat_content: 1.0,
        };

        let mut player1 = Player {
            name: String::from("player1"),
            strength: 1.0,
            score: 0,
            money: 0,
            weapons: vec![String::from("knife")],
        };
        println!("Before eating {:?}", player1);
        player1.eat(apple);
        println!("After eating an apple\n{}", player1);
        player1.eat(steak);
        println!("After eating a steak\n{}", player1);
    }
}
