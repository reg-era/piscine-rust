use rand::{Rng, thread_rng};

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
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
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

#[derive(Debug,PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    let is_spadel = match card.suit {
        Suit::Spade => true,
        _ => false,
    };
    let is_ace = match card.rank {
        Rank::Ace => true,
        _ => false,
    };
    is_ace && is_spadel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        loop {
            let your_card = Card {
                rank: Rank::random(),
                suit: Suit::random(),
            };
            println!("Your card is {:?}", &your_card);

            if winner_card(&your_card) {
                println!("You are the winner!");
                break;
            }
        }
    }

    #[test]
    fn test_winner() {
        let winner = Card {
            rank: Rank::Ace,
            suit: Suit::Spade,
        };

        for rank in 1..14 {
            for suit in 1..5 {
                let card = Card {
                    rank: Rank::translate(rank),
                    suit: Suit::translate(suit),
                };

                assert_eq!(winner_card(&card), card == winner);
            }
        }
    }
}
