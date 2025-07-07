use rand::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        Self::translate(thread_rng().gen_range(1..=4))
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Self::Heart,
            2 => Self::Diamond,
            3 => Self::Spade,
            4 => Self::Club,
            _ => unreachable!(),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        Self::translate(thread_rng().gen_range(1..=13))
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => Rank::Number(value),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    match (card.suit, card.rank) {
        (Suit::Spade, Rank::Ace) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let your_card = Card {
            rank: Rank::random(),
            suit: Suit::random(),
        };

        println!("Your card is {:?}", &your_card);

        if winner_card(&your_card) {
            println!("You are the winner!");
        }
    }
}
