#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Nulla,
            1 => Self::I,
            5 => Self::V,
            10 => Self::X,
            50 => Self::L,
            100 => Self::C,
            500 => Self::D,
            1000 => Self::M,
            _ => panic!("invalid roman numbre {}", value),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

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
