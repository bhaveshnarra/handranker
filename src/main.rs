extern crate fixedbitset;
use std::fmt;
use fixedbitset::FixedBitSet;

enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

enum Value {
    Two=2,
    Three=3,
    Four=5,
    Five=7,
    Six=11,
    Seven=13,
    Eight=17,
    Nine=19,
    Ten=23,
    Jack=29,
    Queen=31,
    King=37,
    Ace=41,
}

struct Card {
    suit: Suit,
    value: Value,
}

impl Card {
    fn new(suit:Suit,value:Value) -> Card {
        Card{suit,value}
    }

    fn CardToFixedBitSet(self) -> () {
        let mut x = FixedBitSet::with_capacity(32);
        let P = x.clone();
        let T = self.value as u32;
        // x = x.as_slice().chunks(32)|T;
        for chunk in P.as_slice().iter() {
            println!("{}",chunk | T);
            println!("{}",chunk & T);
        }
        ()
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut x = "".to_string();
        let mut y = "".to_string();
        match self.suit {
                    Suit::Club => {
                        x = String::from("♣");
                    },
                    Suit::Diamond => {
                        x = String::from("♦");
                    },
                    Suit::Heart => {
                        x = String::from("♥");
                    },
                    Suit::Spade => {
                        x = String::from("♠");
                    },

        }


        match self.value {
            Value::Ace => {
                y = "A".to_string();
            },
            Value::King => {
                y = "K".to_string();
            },
            Value::Queen => {
                y = "Q".to_string();
            },
            Value::Jack => {
                y = "J".to_string();
            },
            Value::Ten => {
                y = "10".to_string();
            },
            Value::Nine => {
                y = "9".to_string();
            },
            Value::Eight => {
                y = "8".to_string();
            },
            Value::Seven => {
                y = "7".to_string();
            },
            Value::Six => {
                y = "6".to_string();
            },
            Value::Five => {
                y = "5".to_string();
            },
            Value::Four => {
                y = "4".to_string();
            },
            Value::Three => {
                y = "3".to_string();
            },
            Value::Two => {
                y = "2".to_string();
            }
        }

        write!(f, "({}, {})", x, y)
    }


}




fn main() {
    let c1 = Card::new(Suit::Spade,Value::Ace);
    println!("{}",c1);
    c1.CardToFixedBitSet();
}
