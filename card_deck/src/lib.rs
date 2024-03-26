use rand::Rng;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}
impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(1, 5);
        Self::translate(n)
    }
    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("{} is invalid for a Suit", value),
        }
    }
}
impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(1, 14);
        Self::translate(n)
    }
    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("{} is invalid for a Rank", value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}
pub fn winner_card(card: &Card) -> bool {
    if let Rank::Ace = card.rank {
        if let Suit::Spade = card.suit {
            return true;
        };
    };
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Rank::*;
    use super::Suit::*;
    #[test]
    fn suit_test() {
        let suits = [Heart, Diamond, Spade, Club];
        for n in 1..=4 {
            println!("Translating {} to suit", n);
            let suit = Suit::translate(n);
            assert_eq!(suits[n as usize-1], suit)
        }
        let _ = Suit::random();
    }
    #[test]
    fn rank_test() {
        let ranks = [Ace, Number(2), Number(3), Number(4), Number(5), Number(6), Number(7), Number(8), Number(9), Number(10), Jack, Queen, King];
        for n in 1..=13 {
            println!("Translating {} to rank", n);
            let rank = Rank::translate(n);
            assert_eq!(ranks[n as usize-1], rank)
        }
        let _ = Rank::random();
    }
    #[test]
    fn card_test() {
        let win_card = Card {
            rank: Ace,
            suit: Spade,
        };
        let other_card = Card {
            rank: King,
            suit: Heart,
        };
        assert!(winner_card(&win_card));
        assert!(!winner_card(&other_card));
    }
}