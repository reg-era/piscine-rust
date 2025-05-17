mod mobs;

use mobs::{Boss, Member, Mob, Role, boss, member};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_steal() {
        let mut mob1 = Mob {
            name: "mob1".to_string(),
            boss: Boss {
                name: "boss1".to_string(),
                age: 50,
            },
            members: HashMap::new(),
            cities: HashSet::new(),
            wealth: 100,
        };
        let mut mob2 = Mob {
            name: "mob2".to_string(),
            boss: Boss {
                name: "boss2".to_string(),
                age: 45,
            },
            members: HashMap::new(),
            cities: HashSet::new(),
            wealth: 50,
        };
        mob1.steal(&mut mob2, 30);
        assert_eq!(mob1.wealth, 130);
        assert_eq!(mob2.wealth, 20);

        mob1.steal(&mut mob2, 50); // mob2 only has 20 left
        assert_eq!(mob1.wealth, 150);
        assert_eq!(mob2.wealth, 0);
    }
}
