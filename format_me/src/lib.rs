use std::fmt;

pub struct Park {
    name: String,
    park_type: ParkType,
    address: String,
    cap: String,
    state: String,
}

pub enum ParkType {
    Garden,
    Forest,
    Playground,
}

// garden - Les Tuileries, Pl. de la Concorde, 75001 - France
// playground - No name, No address, No cap - No state

impl fmt::Display for Park {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = if self.name.len() != 0 {
            self.name.to_string()
        } else {
            "No name".to_string()
        };

        let adress = if self.address.len() != 0 {
            self.address.to_string()
        } else {
            "No address".to_string()
        };

        let cap = if self.cap.len() != 0 {
            self.cap.to_string()
        } else {
            "No cap".to_string()
        };

        let state = if self.state.len() != 0 {
            self.state.to_string()
        } else {
            "No state".to_string()
        };

        write!(
            f,
            "{} - {}, {}, {} - {}",
            self.park_type, name, adress, cap, state
        )
    }
}

impl fmt::Display for ParkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Garden => write!(f, "garden"),
            Self::Forest => write!(f, "forest"),
            Self::Playground => write!(f, "playground"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_park() {
        let park = Park {
            name: "Central Park".to_string(),
            park_type: ParkType::Garden,
            address: "Av. Sidónio Pais 4".to_string(),
            cap: "1050-214".to_string(),
            state: "Portugal".to_string(),
        };

        assert_eq!(
            park.to_string(),
            "garden - Central Park, Av. Sidónio Pais 4, 1050-214 - Portugal"
        );
    }

    #[test]
    fn test_empty_name() {
        let park = Park {
            name: "".to_string(),
            park_type: ParkType::Forest,
            address: "Av. Sidónio Pais 4".to_string(),
            cap: "1050-214".to_string(),
            state: "Portugal".to_string(),
        };

        assert_eq!(
            park.to_string(),
            "forest - No name, Av. Sidónio Pais 4, 1050-214 - Portugal"
        );
    }

    #[test]
    fn test_empty_all() {
        let park = Park {
            name: "".to_string(),
            park_type: ParkType::Playground,
            address: "".to_string(),
            cap: "".to_string(),
            state: "".to_string(),
        };

        assert_eq!(
            park.to_string(),
            "playground - No name, No address, No cap - No state"
        );
    }
}