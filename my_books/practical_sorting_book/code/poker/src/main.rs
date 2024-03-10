use std::cmp::Ordering;
use std::fmt;

#[derive(Debug)]
enum Suite {
    Heart,
    Club,
    Spade,
    Diamond,
}

struct Card {
    name: u8,
    suite: Suite,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self.name {
            x @ 2..=10 => x.to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            14 => "A".to_string(),
            _ => panic!("number {:?} is not a valid card number", &self.name),
        };

        f.debug_struct("Card")
            .field("name", &name)
            .field("suite", &self.suite)
            .finish()
    }
}

impl Card {
    fn new(name: u8, suite: Suite) -> Result<Self, &'static str> {
        match name {
            x @ 2..=14 => Ok(Card { name: x, suite }),
            _ => Err("name must be in the range of 2 - 14"),
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name).reverse()
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum PokerHandType {
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

#[derive(Debug)]
struct PokerHand {
    cards: Vec<Card>,
    poker_hand_type: Option<PokerHandType>,
}

impl PokerHand {
    fn new() -> Self {
        PokerHand {
            cards: Vec::new(),
            poker_hand_type: None,
        }
    }

    fn add_card(&mut self, card: Card) -> Result<(), &'static str> {
        // Our card hand limit is 5.
        if self.cards.len() == 5 {
            return Err("Max hand limit is 5");
        }

        self.cards.push(card);
        Ok(())
    }
}

fn main() {
    let mut hand = PokerHand::new();
    for x in 2..8 {
        let card = Card::new(x, Suite::Heart).unwrap();
        hand.add_card(card).expect("Too many Cards!!!");

        println!("{:?}", hand);
    }
}
