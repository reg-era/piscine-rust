pub mod mobs;

use std::collections::HashMap;
use std::collections::HashSet;

pub use mobs::*;

pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn recruit(&mut self, (name, age_nb): (&str, u32)) {
        let new_member = Member {
            age: age_nb,
            role: Role::Associate,
        };
        self.members.insert(String::from(name), new_member);
    }

    pub fn attack(&mut self, other: &mut Mob) {
        fn combat_power(members: &HashMap<String, Member>) -> u32 {
            let mut acc = 0;
            let calcul_power = members.iter().map(|(_, m)| match m.role {
                Role::Underboss => 4,
                Role::Caporegime => 3,
                Role::Soldier => 2,
                Role::Associate => 1,
            });
            for amount in calcul_power {
                acc += amount;
            }
            acc
        }

        let (loser, winner) = if combat_power(&self.members) > combat_power(&other.members) {
            (other, self)
        } else {
            (self, other)
        };

        let min_age = loser.members.iter().map(|(_, mem)| mem.age).min().unwrap();
        let mut dead_name = String::new();
        for (name, mem) in loser.members.iter() {
            if mem.age == min_age {
                dead_name = name.clone();
                break;
            }
        }

        loser.members.remove(&dead_name);

        if loser.members.is_empty() {
            winner.cities.extend(loser.cities.drain());
            winner.wealth += loser.wealth;
            loser.wealth = 0;
        }
    }

    pub fn steal(&mut self, target: &mut Mob, steal: u64) {
        let amount = steal.min(target.wealth);
        target.wealth -= amount;
        self.wealth += amount;
    }

    pub fn conquer_city(&mut self, all_mobs: &[&Mob], city_name: String) {
        for mob in all_mobs {
            if mob.cities.contains(&city_name) {
                return;
            }
        }
        self.cities.insert(city_name);
    }
}
