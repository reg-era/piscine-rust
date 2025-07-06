use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RomanNumber(Vec<String>);

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let numb = from_roman_sequence(self.0.clone());
        match numb {
            Some(nb) => {
                self.0 = to_roman_sequence(nb + 1);
                Some(self.clone())
            }
            None => None,
        }
    }
}

impl RomanNumber {
    pub fn from(n: u64) -> Self {
        Self(to_roman_sequence(n))
    }
}

pub fn to_roman_sequence(mut num: u64) -> Vec<String> {
    let roman_numerals = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut result = Vec::new();

    for &(value, symbol) in &roman_numerals {
        while num >= value {
            result.push(symbol.to_string());
            num -= value;
        }
    }

    result
}

pub fn from_roman_sequence(seq: Vec<String>) -> Option<u64> {
    let roman_values: HashMap<&str, u64> = HashMap::from([
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
            let combined = format!("{}{}", seq[i], seq[i + 1]);
            if let Some(&value) = roman_values.get(combined.as_str()) {
                total += value;
                i += 2;
                continue;
            }
        }

        if let Some(&value) = roman_values.get(seq[i].as_str()) {
            total += value;
            i += 1;
        } else {
            return None;
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roman_numbers_iterator_test() {
        assert_eq!(
            RomanNumber::from(1).0,
            RomanNumber::from(0).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(9).0,
            RomanNumber::from(8).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(6).0,
            RomanNumber::from(5).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(14).0,
            RomanNumber::from(13).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(34).0,
            RomanNumber::from(33).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(50).0,
            RomanNumber::from(49).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(200).0,
            RomanNumber::from(199).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(500).0,
            RomanNumber::from(499).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(1533).0,
            RomanNumber::from(1532).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(2349).0,
            RomanNumber::from(2348).next().unwrap().0
        );
    }
}
