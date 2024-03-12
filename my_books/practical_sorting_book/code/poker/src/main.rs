use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq)]
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

    fn is_flush(&self) -> bool {
        let mut result = true;

        let suite = &self.cards[0].suite;
        for card in &self.cards[1..] {
            if card.suite != *suite {
                result = false;
                break;
            }
        }

        result
    }

    fn card_rank_histogram(&self) -> Vec<(u8, usize)> {
        let mut map = HashMap::new();

        for card in &self.cards {
            map.entry(card.name)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let mut results: Vec<(u8, usize)> = map
            .iter()
            .map(|(k, v)| (k.clone(), v.clone() as usize))
            .collect();
        results.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)).reverse());

        results
    }

    fn have_straight(&self) -> bool {
        // TODO: Add unit tests
        let mut ranks: Vec<u8> = self.cards.iter().map(|card| card.name.clone()).collect();

        ranks.sort();

        match ranks[0] {
            2 => ranks == [2, 3, 4, 5, 14] || ranks == [2, 3, 4, 5, 6],
            _ => {
                let mut result = true;

                let mut expected = ranks[0];
                for num in ranks {
                    if num != expected {
                        result = false;
                        break;
                    }

                    expected += 1
                }

                result
            }
        }
    }

    fn set_hand_type(&mut self) -> Result<(), &'static str> {
        // TODO: Add unit tests
        if self.cards.len() != 5 {
            return Err("Must have 5 Cards to set hand type");
        }

        let card_rank_histogram = self.card_rank_histogram();

        // Check Poker hand Types that have multiple cards of the same rank
        if card_rank_histogram[0].1 == 4 {
            self.poker_hand_type = Some(PokerHandType::FourOfAKind);
        } else if card_rank_histogram[0].1 == 3 && card_rank_histogram[1].1 == 2 {
            self.poker_hand_type = Some(PokerHandType::FullHouse);
        } else if card_rank_histogram[0].1 == 3 && card_rank_histogram[1].1 == 1 {
            self.poker_hand_type = Some(PokerHandType::ThreeOfAKind);
        } else if card_rank_histogram[0].1 == 2 && card_rank_histogram[1].1 == 2 {
            self.poker_hand_type = Some(PokerHandType::TwoPair);
        } else if card_rank_histogram[0].1 == 2 && card_rank_histogram[1].1 == 1 {
            self.poker_hand_type = Some(PokerHandType::Pair)
        }

        // Check to see if we should return early
        if self.poker_hand_type.is_some() {
            return Ok(());
        }

        //TODO change is_flush to has_flush
        match (self.have_straight(), self.is_flush()) {
            (false, false) => self.poker_hand_type = Some(PokerHandType::HighCard),
            (false, true) => self.poker_hand_type = Some(PokerHandType::Flush),
            (true, false) => self.poker_hand_type = Some(PokerHandType::Straight),
            (true, true) => {
                let mut ranks: Vec<u8> = self.cards.iter().map(|card| card.name.clone()).collect();
                ranks.sort();
                if ranks[0] == 10 {
                    self.poker_hand_type = Some(PokerHandType::RoyalFlush);
                } else {
                    self.poker_hand_type = Some(PokerHandType::StraightFlush);
                }
            }
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Too many Cards!!!")]
    fn test_too_many_cards() {
        let mut hand = PokerHand::new();
        for x in 2..8 {
            let card = Card::new(x, Suite::Heart).unwrap();
            hand.add_card(card).expect("Too many Cards!!!");

            println!("{:?}", hand);
        }
    }

    #[test]
    fn test_card_rank_histogram() {
        let mut hand = PokerHand::new();

        for x in [6, 8, 4, 4, 8] {
            let card = Card::new(x, Suite::Heart).unwrap();
            hand.add_card(card).unwrap();
        }

        let result = hand.card_rank_histogram();

        assert_eq!(result, vec![(8, 2), (4, 2), (6, 1)])
    }

    #[test]
    fn test_is_flush_true() {
        let mut hand = PokerHand::new();

        for x in [6, 8, 4, 4, 8] {
            let card = Card::new(x, Suite::Heart).unwrap();
            hand.add_card(card).unwrap();
        }

        assert!(hand.is_flush());
    }

    #[test]
    fn test_is_flush_false() {
        let mut hand = PokerHand::new();

        for x in [6, 8, 4, 4] {
            let card = Card::new(x, Suite::Heart).unwrap();
            hand.add_card(card).unwrap();
        }
        hand.add_card(Card::new(2, Suite::Spade).unwrap()).unwrap();

        assert_eq!(hand.is_flush(), false);
    }
}
