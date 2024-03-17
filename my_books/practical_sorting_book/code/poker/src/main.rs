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

    fn have_flush(&self) -> bool {
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
        let mut ranks: Vec<u8> = self.cards.iter().map(|card| card.name.clone()).collect();

        ranks.sort();

        if ranks == [2, 3, 4, 5, 14] {
            true
        } else {
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

    fn set_hand_type(&mut self) -> Result<(), &'static str> {
        if self.cards.len() != 5 {
            return Err("Must have 5 Cards to set hand type");
        }

        self.poker_hand_type = None;

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

        match (self.have_straight(), self.have_flush()) {
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

        match self.poker_hand_type {
            None => Err("We unable to figure out your poker hand type"),
            Some(ref _hand_type) => Ok(()),
        }
    }

    fn sort_hand(&mut self) -> Result<(), String> {

	let _ = self.set_hand_type()?;
	let card_rank_histogram = self.card_rank_histogram();
	
        if let Some(poker_hand_type) = &self.poker_hand_type {
            match poker_hand_type {
                PokerHandType::Pair | PokerHandType::ThreeOfAKind | PokerHandType::FourOfAKind => {
		    

		    let priority_card_name = card_rank_histogram[0].0;
                 
                    self.cards.sort_by(|a, b| {
                        if a.name == priority_card_name && b.name == priority_card_name {
                            Ordering::Equal
                        } else if a.name == priority_card_name && b.name != priority_card_name {
                            Ordering::Less
                        } else if a.name != priority_card_name && b.name == priority_card_name {
                            Ordering::Greater
                        } else {
                            a.cmp(&b)
                        }
                    });

                    Ok(())
                }
                PokerHandType::RoyalFlush
                | PokerHandType::StraightFlush
                | PokerHandType::Flush
                | PokerHandType::Straight
                | PokerHandType::HighCard => {
                    self.cards.sort();

                    if *poker_hand_type == PokerHandType::StraightFlush
                        || *poker_hand_type == PokerHandType::Straight
                    {
                        // In the case where the straight is Ace, 5, 4 ,3, 2, 1, we need to list
                        // the left by 1 -> 5, 4, 3, 2, Ace
                        if self.cards[0].name == 14 && self.cards[1].name == 5 {
                            self.cards.rotate_left(1);
                        }
                    }

                    Ok(())
                }
                PokerHandType::FullHouse | PokerHandType::TwoPair => {
		    let priority_1 = card_rank_histogram[0].0;
		    let priority_2 = card_rank_histogram[1].0;
		    
                    self.cards.sort_by(|a, b| {
                        if a.name == priority_1 && b.name == priority_1 {
                            a.cmp(&b)
                        } else if a.name == priority_1 && b.name == priority_2 {
                            Ordering::Less
                        } else if a.name == priority_2 && b.name == priority_1 {
                            Ordering::Greater
                        } else if a.name == priority_2 && b.name == priority_2 {
                            a.cmp(&b)
                        } else if a.name == priority_1
                            && b.name != priority_1
                            && b.name != priority_2
                        {
                            Ordering::Less
                        } else if a.name == priority_2
                            && b.name != priority_1
                            && b.name != priority_2
                        {
                            Ordering::Less
                        } else if a.name != priority_1
                            && a.name != priority_2
                            && b.name == priority_1
                        {
                            Ordering::Greater
                        } else if a.name != priority_1
                            && a.name != priority_2
                            && b.name == priority_2
                        {
                            Ordering::Greater
                        } else {
                            a.cmp(&b)
                        }
                    });

                    Ok(())
                }
            }
        } else {
            Err("Cannot sort because no poker hand type is specified".to_string())
        }
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
    fn test_set_hand_type() {
        let cases = vec![
            (
                PokerHand {
                    cards: vec![
                        Card::new(2, Suite::Spade).unwrap(),
                        Card::new(3, Suite::Spade).unwrap(),
                        Card::new(2, Suite::Heart).unwrap(),
                        Card::new(2, Suite::Diamond).unwrap(),
                        Card::new(2, Suite::Club).unwrap(),
                    ],
                    poker_hand_type: None,
                },
                Some(PokerHandType::FourOfAKind),
            ),
            (
                PokerHand {
                    cards: vec![
                        Card::new(2, Suite::Spade).unwrap(),
                        Card::new(3, Suite::Spade).unwrap(),
                        Card::new(2, Suite::Heart).unwrap(),
                        Card::new(2, Suite::Diamond).unwrap(),
                        Card::new(14, Suite::Spade).unwrap(),
                    ],
                    poker_hand_type: None,
                },
                Some(PokerHandType::ThreeOfAKind),
            ),
            (
                PokerHand {
                    cards: vec![
                        Card::new(5, Suite::Spade).unwrap(),
                        Card::new(6, Suite::Spade).unwrap(),
                        Card::new(6, Suite::Heart).unwrap(),
                        Card::new(5, Suite::Heart).unwrap(),
                        Card::new(5, Suite::Club).unwrap(),
                    ],
                    poker_hand_type: None,
                },
                Some(PokerHandType::FullHouse),
            ),
            (
                PokerHand {
                    cards: vec![
                        Card::new(2, Suite::Spade).unwrap(),
                        Card::new(3, Suite::Spade).unwrap(),
                        Card::new(4, Suite::Spade).unwrap(),
                        Card::new(2, Suite::Heart).unwrap(),
                        Card::new(3, Suite::Heart).unwrap(),
                    ],
                    poker_hand_type: None,
                },
                Some(PokerHandType::TwoPair),
            ),
            (
                PokerHand {
                    cards: vec![
                        Card::new(2, Suite::Spade).unwrap(),
                        Card::new(3, Suite::Spade).unwrap(),
                        Card::new(4, Suite::Spade).unwrap(),
                        Card::new(2, Suite::Heart).unwrap(),
                        Card::new(5, Suite::Heart).unwrap(),
                    ],
                    poker_hand_type: None,
                },
                Some(PokerHandType::Pair),
            ),
            (
                PokerHand {
                    cards: vec![
                        Card::new(2, Suite::Spade).unwrap(),
                        Card::new(3, Suite::Spade).unwrap(),
                        Card::new(4, Suite::Spade).unwrap(),
                        Card::new(6, Suite::Spade).unwrap(),
                        Card::new(14, Suite::Spade).unwrap(),
                    ],
                    poker_hand_type: None,
                },
                Some(PokerHandType::Flush),
            ),
            (
                PokerHand {
                    cards: vec![
                        Card::new(2, Suite::Spade).unwrap(),
                        Card::new(3, Suite::Spade).unwrap(),
                        Card::new(4, Suite::Spade).unwrap(),
                        Card::new(5, Suite::Spade).unwrap(),
                        Card::new(6, Suite::Spade).unwrap(),
                    ],
                    poker_hand_type: None,
                },
                Some(PokerHandType::StraightFlush),
            ),
            (
                PokerHand {
                    cards: vec![
                        Card::new(14, Suite::Spade).unwrap(),
                        Card::new(13, Suite::Spade).unwrap(),
                        Card::new(12, Suite::Spade).unwrap(),
                        Card::new(11, Suite::Spade).unwrap(),
                        Card::new(10, Suite::Spade).unwrap(),
                    ],
                    poker_hand_type: None,
                },
                Some(PokerHandType::RoyalFlush),
            ),
            (
                PokerHand {
                    cards: vec![
                        Card::new(2, Suite::Spade).unwrap(),
                        Card::new(3, Suite::Spade).unwrap(),
                        Card::new(4, Suite::Spade).unwrap(),
                        Card::new(5, Suite::Heart).unwrap(),
                        Card::new(6, Suite::Heart).unwrap(),
                    ],
                    poker_hand_type: None,
                },
                Some(PokerHandType::Straight),
            ),
            (
                PokerHand {
                    cards: vec![
                        Card::new(2, Suite::Spade).unwrap(),
                        Card::new(3, Suite::Spade).unwrap(),
                        Card::new(4, Suite::Spade).unwrap(),
                        Card::new(5, Suite::Heart).unwrap(),
                        Card::new(14, Suite::Heart).unwrap(),
                    ],
                    poker_hand_type: None,
                },
                Some(PokerHandType::Straight),
            ),
            (
                PokerHand {
                    cards: vec![
                        Card::new(2, Suite::Spade).unwrap(),
                        Card::new(3, Suite::Spade).unwrap(),
                        Card::new(14, Suite::Spade).unwrap(),
                        Card::new(5, Suite::Heart).unwrap(),
                        Card::new(6, Suite::Heart).unwrap(),
                    ],
                    poker_hand_type: Some(PokerHandType::FullHouse),
                },
                Some(PokerHandType::HighCard),
            ),
        ];

        for (mut hand, expected) in cases {
            let _ = hand.set_hand_type();
            assert_eq!(hand.poker_hand_type, expected);
        }
    }

    #[test]
    fn test_have_straight() {
        let cases = vec![
            (
                PokerHand {
                    cards: vec![
                        Card::new(2, Suite::Spade).unwrap(),
                        Card::new(3, Suite::Spade).unwrap(),
                        Card::new(4, Suite::Spade).unwrap(),
                        Card::new(5, Suite::Spade).unwrap(),
                        Card::new(6, Suite::Spade).unwrap(),
                    ],
                    poker_hand_type: None,
                },
                true,
            ),
            (
                PokerHand {
                    cards: vec![
                        Card::new(2, Suite::Spade).unwrap(),
                        Card::new(3, Suite::Spade).unwrap(),
                        Card::new(4, Suite::Spade).unwrap(),
                        Card::new(5, Suite::Spade).unwrap(),
                        Card::new(14, Suite::Spade).unwrap(),
                    ],
                    poker_hand_type: None,
                },
                true,
            ),
            (
                PokerHand {
                    cards: vec![
                        Card::new(5, Suite::Spade).unwrap(),
                        Card::new(6, Suite::Spade).unwrap(),
                        Card::new(7, Suite::Spade).unwrap(),
                        Card::new(8, Suite::Spade).unwrap(),
                        Card::new(9, Suite::Spade).unwrap(),
                    ],
                    poker_hand_type: None,
                },
                true,
            ),
            (
                PokerHand {
                    cards: vec![
                        Card::new(2, Suite::Spade).unwrap(),
                        Card::new(3, Suite::Spade).unwrap(),
                        Card::new(4, Suite::Spade).unwrap(),
                        Card::new(5, Suite::Spade).unwrap(),
                        Card::new(7, Suite::Spade).unwrap(),
                    ],
                    poker_hand_type: None,
                },
                false,
            ),
        ];

        for (hand, expected) in cases {
            assert_eq!(hand.have_straight(), expected);
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
    fn test_have_flush_true() {
        let mut hand = PokerHand::new();

        for x in [6, 8, 4, 4, 8] {
            let card = Card::new(x, Suite::Heart).unwrap();
            hand.add_card(card).unwrap();
        }

        assert!(hand.have_flush());
    }

    #[test]
    fn test_have_flush_false() {
        let mut hand = PokerHand::new();

        for x in [6, 8, 4, 4] {
            let card = Card::new(x, Suite::Heart).unwrap();
            hand.add_card(card).unwrap();
        }
        hand.add_card(Card::new(2, Suite::Spade).unwrap()).unwrap();

        assert_eq!(hand.have_flush(), false);
    }
}
