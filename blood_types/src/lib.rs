#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};
use std::str::FromStr;

impl FromStr for Antigen {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        let antigen = match s {
            "A" => Antigen::A,
            "B" => Antigen::B,
            "AB" => Antigen::AB,
            "O" => Antigen::O,
            _ => return Err(String::from("anigen doesnt match")),
        };

        Ok(antigen)
    }
}

impl FromStr for RhFactor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        if s == "-" {
            Ok(RhFactor::Negative)
        } else if s == "+" {
            Ok(RhFactor::Positive)
        } else {
            Err(String::from("rhfactore isnt compatible"))
        }
    }
}

impl FromStr for BloodType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        let antigen = s.trim_end_matches(|c| c == '-' || c == '+').parse();
        let rh_factor = s
            .trim_start_matches(|c| c == 'A' || c == 'B' || c == 'O')
            .parse();

        if let Err(msg) = antigen {
            return Err(msg);
        }
        if let Err(msg) = antigen {
            return Err(msg);
        }

        Ok(Self {
            antigen: antigen.unwrap(),
            rh_factor: rh_factor.unwrap(),
        })
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => self.rh_factor.cmp(&other.rh_factor),
            non_eq => non_eq,
        }
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Antigen::*;
        use RhFactor::*;

        match self.antigen {
            A => match self.rh_factor {
                Negative => write!(f, "{}", "A-"),
                Positive => write!(f, "{}", "A+"),
            },
            B => match self.rh_factor {
                Negative => write!(f, "{}", "B-"),
                Positive => write!(f, "{}", "B+"),
            },
            AB => match self.rh_factor {
                Negative => write!(f, "{}", "AB-"),
                Positive => write!(f, "{}", "AB+"),
            },
            O => match self.rh_factor {
                Negative => write!(f, "{}", "O-"),
                Positive => write!(f, "{}", "O+"),
            },
        }
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        use Antigen::*;
        use RhFactor::*;

        if self.rh_factor == Negative && other.rh_factor == Positive {
            return false;
        }

        match (&self.antigen, &other.antigen) {
            (O, O) => true,
            (A, A) | (A, O) => true,
            (B, B) | (B, O) => true,
            (AB, A) | (AB, B) | (AB, AB) | (AB, O) => true,
            _ => false,
        }
    }

    pub fn donors(&self) -> Vec<Self> {
        use Antigen::*;
        use RhFactor::*;

        let mut donors = Vec::new();

        for antigen in [O, A, B, AB] {
            // Rh- --> Rh-
            // Rh+ -->  Rh- Rh+
            let rh_options = if self.rh_factor == Negative {
                vec![Negative]
            } else {
                vec![Negative, Positive]
            };

            for rh in rh_options {
                let donor = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh,
                };
                if self.can_receive_from(&donor) {
                    donors.push(donor);
                }
            }
        }

        donors
    }

    // Who can receive blood from self
    pub fn recipients(&self) -> Vec<Self> {
        use Antigen::*;
        use RhFactor::*;

        let mut recipients = Vec::new();

        for antigen in [O, A, B, AB] {
            let rh_options = vec![Negative, Positive];

            for rh in rh_options {
                let recipient = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh,
                };
                if recipient.can_receive_from(self) {
                    recipients.push(recipient);
                }
            }
        }

        recipients
    }
}
