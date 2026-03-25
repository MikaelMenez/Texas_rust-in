pub mod cards {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter, Result};
    #[derive(Debug)]
    pub enum CardError {
        WrongRank,
        WrongSuit,
    }
    #[derive(Debug)]
    pub enum Suits {
        Hearts,
        Clubs,
        Diamonds,
        Spades,
    }
    #[derive(Debug)]
    pub enum Ranks {
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace,
    }
    #[derive(Debug, PartialEq, Eq)]
    pub struct Card {
        pub rank: char,
        pub suit: char,
    }

    impl Display for CardError {
        fn fmt(&self, f: &mut Formatter) -> Result {
            match self {
                Self::WrongRank => write!(f, "{}", "Wrong value for rank"),
                Self::WrongSuit => write!(f, "{}", "Wrong value for suit"),
            }
        }
    }
    impl Suits {
        pub fn symbol(&self) -> char {
            match self {
                Self::Hearts => '♥',
                Self::Clubs => '♧',
                Self::Diamonds => '♦',
                Self::Spades => '♤',
            }
        }
    }
    impl Ranks {
        pub fn symbol(&self) -> char {
            match self {
                Self::Two => '2',
                Self::Three => '3',
                Self::Four => '4',
                Self::Five => '5',
                Self::Six => '6',
                Self::Seven => '7',
                Self::Eight => '8',
                Self::Nine => '9',
                Self::Ten => '0',
                Self::Jack => 'j',
                Self::Queen => 'q',
                Self::King => 'k',
                Self::Ace => 'a',
            }
        }
    }
    impl PartialOrd for Card {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Display for Card {
        fn fmt(&self, f: &mut Formatter) -> Result {
            let mut ranks: HashMap<char, u8> = HashMap::new();
            let base = 0x1F000_u32;
            ranks.insert('2', 2);
            ranks.insert('3', 3);
            ranks.insert('4', 4);
            ranks.insert('5', 5);
            ranks.insert('6', 6);
            ranks.insert('7', 7);
            ranks.insert('8', 8);
            ranks.insert('9', 9);
            ranks.insert('0', 10);
            ranks.insert('j', 11);
            ranks.insert('q', 13);
            ranks.insert('k', 14);
            ranks.insert('a', 1);
            ranks.insert('♦', 0xC0);
            ranks.insert('♥', 0xB0);
            ranks.insert('♤', 0xA0);
            ranks.insert('♧', 0xD0);
            let rank = ranks.get(&self.rank);
            let rank = match rank {
                Some(a) => a,
                None => &0_u8,
            };
            let suit = ranks.get(&self.suit);
            let suit = match suit {
                Some(a) => a,
                None => &0_u8,
            };
            let card = base + *suit as u32 + *rank as u32;
            write!(f, "{}", std::char::from_u32(card).unwrap())
        }
    }
    impl Ord for Card {
        fn cmp(&self, other: &Self) -> Ordering {
            let mut ranks: HashMap<char, u8> = HashMap::new();
            ranks.insert('2', 2);
            ranks.insert('3', 3);
            ranks.insert('4', 4);
            ranks.insert('5', 5);
            ranks.insert('6', 6);
            ranks.insert('7', 7);
            ranks.insert('8', 8);
            ranks.insert('9', 9);
            ranks.insert('0', 10);
            ranks.insert('j', 11);
            ranks.insert('q', 12);
            ranks.insert('k', 13);
            ranks.insert('a', 14);
            let rank1 = ranks.get(&self.rank);
            let rank2 = ranks.get(&other.rank);
            let rank1 = match rank1 {
                Some(&a) => a,
                None => 0,
            };
            let rank2 = match rank2 {
                Some(&a) => a,
                None => 0,
            };
            rank1.cmp(&rank2)
        }
    }
    impl Card {
        pub fn new(rank: Ranks, suit: Suits) -> Card {
            Card {
                rank: rank.symbol(),
                suit: suit.symbol(),
            }
        }
    }

    pub static DECK: &[Card] = &[
        Card {
            rank: '2',
            suit: '♧',
        },
        Card {
            rank: '3',
            suit: '♧',
        },
        Card {
            rank: '4',
            suit: '♧',
        },
        Card {
            rank: '5',
            suit: '♧',
        },
        Card {
            rank: '6',
            suit: '♧',
        },
        Card {
            rank: '7',
            suit: '♧',
        },
        Card {
            rank: '8',
            suit: '♧',
        },
        Card {
            rank: '9',
            suit: '♧',
        },
        Card {
            rank: '0',
            suit: '♧',
        },
        Card {
            rank: 'j',
            suit: '♧',
        },
        Card {
            rank: 'q',
            suit: '♧',
        },
        Card {
            rank: 'k',
            suit: '♧',
        },
        Card {
            rank: 'a',
            suit: '♧',
        },
        Card {
            rank: '2',
            suit: '♤',
        },
        Card {
            rank: '3',
            suit: '♤',
        },
        Card {
            rank: '4',
            suit: '♤',
        },
        Card {
            rank: '5',
            suit: '♤',
        },
        Card {
            rank: '6',
            suit: '♤',
        },
        Card {
            rank: '7',
            suit: '♤',
        },
        Card {
            rank: '8',
            suit: '♤',
        },
        Card {
            rank: '9',
            suit: '♤',
        },
        Card {
            rank: '0',
            suit: '♤',
        },
        Card {
            rank: 'j',
            suit: '♤',
        },
        Card {
            rank: 'q',
            suit: '♤',
        },
        Card {
            rank: 'k',
            suit: '♤',
        },
        Card {
            rank: 'a',
            suit: '♤',
        },
        Card {
            rank: '2',
            suit: '♥',
        },
        Card {
            rank: '3',
            suit: '♥',
        },
        Card {
            rank: '4',
            suit: '♥',
        },
        Card {
            rank: '5',
            suit: '♥',
        },
        Card {
            rank: '6',
            suit: '♥',
        },
        Card {
            rank: '7',
            suit: '♥',
        },
        Card {
            rank: '8',
            suit: '♥',
        },
        Card {
            rank: '9',
            suit: '♥',
        },
        Card {
            rank: '0',
            suit: '♥',
        },
        Card {
            rank: 'j',
            suit: '♥',
        },
        Card {
            rank: 'q',
            suit: '♥',
        },
        Card {
            rank: 'k',
            suit: '♥',
        },
        Card {
            rank: 'a',
            suit: '♥',
        },
        Card {
            rank: '2',
            suit: '♦',
        },
        Card {
            rank: '3',
            suit: '♦',
        },
        Card {
            rank: '4',
            suit: '♦',
        },
        Card {
            rank: '5',
            suit: '♦',
        },
        Card {
            rank: '6',
            suit: '♦',
        },
        Card {
            rank: '7',
            suit: '♦',
        },
        Card {
            rank: '8',
            suit: '♦',
        },
        Card {
            rank: '9',
            suit: '♦',
        },
        Card {
            rank: '0',
            suit: '♦',
        },
        Card {
            rank: 'j',
            suit: '♦',
        },
        Card {
            rank: 'q',
            suit: '♦',
        },
        Card {
            rank: 'k',
            suit: '♦',
        },
        Card {
            rank: 'a',
            suit: '♦',
        },
    ];
}
