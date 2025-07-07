mod roman_numbers;
pub use roman_numbers::*;

use std::collections::HashMap;

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let numb = from_roman(self.0.clone());
        match numb {
            Some(nb) => {
                let new = RomanNumber::from(nb + 1);
                self.0 = new.0.clone();
                Some(new)
            }
            None => None,
        }
    }
}

pub fn from_roman(seq: Vec<RomanDigit>) -> Option<u32> {
    let roman_values: HashMap<&str, u32> = HashMap::from([
        ("Nulla", 0),
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ]);

    let mut total = 0;
    let mut i = 0;

    while i < seq.len() {
        if i + 1 < seq.len() {
            let combined = format!("{:?}{:?}", seq[i], seq[i + 1]);
            if let Some(&value) = roman_values.get(combined.as_str()) {
                total += value;
                i += 2;
                continue;
            }
        }

        if let Some(&value) = roman_values.get(format!("{:?}", seq[i]).as_str()) {
            total += value;
            i += 1;
        } else {
            return None;
        }
    }

    Some(total)
}
