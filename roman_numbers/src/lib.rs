use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            0 => self::Nulla,
            1 => self::I,
            5 => self::V,
            10 => self::X,
            50 => self::L,
            100 => self::C,
            500 => self::D,
            1000 => self::M,
            _ => panic!("invalid roman numbre {}", value),
        }
    }
}
impl From<u32> for RomanNumber {
    fn from(mut value: u32) -> Self {
        use RomanDigit::*;
        let match_roman = [
            (1000, Vec::from([M])),
            (900, Vec::from([C, M])),
            (500, Vec::from([D])),
            (400, Vec::from([C, D])),
            (100, Vec::from([C])),
            (90, Vec::from([X, C])),
            (50, Vec::from([L])),
            (40, Vec::from([X, L])),
            (10, Vec::from([X])),
            (9, Vec::from([I, X])),
            (5, Vec::from([V])),
            (4, Vec::from([I, V])),
            (1, Vec::from([I])),
        ];

        let mut res = Vec::new();
        for (nb, rome) in match_roman.iter() {
            while value >= *nb {
                res.extend_from_slice(rome);
                value -= nb;
            }
        }

        if res.is_empty() {
            return Self(Vec::from([Nulla]));
        }

        Self(res)
    }
}

#[test]
fn it_works() {
	println!("{:?}", RomanNumber::from(32));
	println!("{:?}", RomanNumber::from(9));
	println!("{:?}", RomanNumber::from(45));
	println!("{:?}", RomanNumber::from(0));
}
