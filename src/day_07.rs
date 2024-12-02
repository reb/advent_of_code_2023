/// --- Day 7: Camel Cards ---
///
/// Your all-expenses-paid trip turns out to be a one-way, five-minute ride in an airship. (At least
/// it's a cool airship!) It drops you off at the edge of a vast desert and descends back to Island
/// Island.
///
/// "Did you bring the parts?"
///
/// You turn around to see an Elf completely covered in white clothing, wearing goggles, and riding
/// a large camel.
///
/// "Did you bring the parts?" she asks again, louder this time. You aren't sure what parts she's
/// looking for; you're here to figure out why the sand stopped.
///
/// "The parts! For the sand, yes! Come with me; I will show you." She beckons you onto the camel.
///
/// After riding a bit across the sands of Desert Island, you can see what look like very large
/// rocks covering half of the horizon. The Elf explains that the rocks are all along the part of
/// Desert Island that is directly above Island Island, making it hard to even get there. Normally,
/// they use big machines to move the rocks and filter the sand, but the machines have broken down
/// because Desert Island recently stopped receiving the parts they need to fix the machines.
///
/// You've already assumed it'll be your job to figure out why the parts stopped when she asks if
/// you can help. You agree automatically.
///
/// Because the journey will take a few days, she offers to teach you the game of Camel Cards. Camel
/// Cards is sort of similar to poker except it's designed to be easier to play while riding a
/// camel.
///
/// In Camel Cards, you get a list of hands, and your goal is to order them based on the strength of
/// each hand. A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or
/// 2. The relative strength of each card follows this order, where A is the highest and 2 is the
/// lowest.
///
/// Every hand is exactly one type. From strongest to weakest, they are:
///
///   - Five of a kind, where all five cards have the same label: AAAAA
///   - Four of a kind, where four cards have the same label and one card has a different label:
///     AA8AA
///   - Full house, where three cards have the same label, and the remaining two cards share a
///     different label: 23332
///   - Three of a kind, where three cards have the same label, and the remaining two cards are each
///     different from any other card in the hand: TTT98
///   - Two pair, where two cards share one label, two other cards share a second label, and the
///     remaining card has a third label: 23432
///   - One pair, where two cards share one label, and the other three cards have a different label
///     from the pair and each other: A23A4
///   - High card, where all cards' labels are distinct: 23456
///
/// Hands are primarily ordered based on type; for example, every full house is stronger than any
/// three of a kind.
///
/// If two hands have the same type, a second ordering rule takes effect. Start by comparing the
/// first card in each hand. If these cards are different, the hand with the stronger first card is
/// considered stronger. If the first card in each hand have the same label, however, then move on
/// to considering the second card in each hand. If they differ, the hand with the higher second
/// card wins; otherwise, continue with the third card in each hand, then the fourth, then the
/// fifth.
///
/// So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its first card
/// is stronger. Similarly, 77888 and 77788 are both a full house, but 77888 is stronger because its
/// third card is stronger (and both hands have the same first and second card).
///
/// To play Camel Cards, you are given a list of hands and their corresponding bid (your puzzle
/// input). For example:
///
/// 32T3K 765
/// T55J5 684
/// KK677 28
/// KTJJT 220
/// QQQJA 483
///
/// This example shows five hands; each hand is followed by its bid amount. Each hand wins an amount
/// equal to its bid multiplied by its rank, where the weakest hand gets rank 1, the second-weakest
/// hand gets rank 2, and so on up to the strongest hand. Because there are five hands in this
/// example, the strongest hand will have rank 5 and its bid will be multiplied by 5.
///
/// So, the first step is to put the hands in order of strength:
///
///   - 32T3K is the only one pair and the other hands are all a stronger type, so it gets rank 1.
///   - KK677 and KTJJT are both two pair. Their first cards both have the same label, but the
///     second card of KK677 is stronger (K vs T), so KTJJT gets rank 2 and KK677 gets rank 3.
///   - T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first card, so it gets rank 5
///     and T55J5 gets rank 4.
///
/// Now, you can determine the total winnings of this set of hands by adding up the result of
/// multiplying each hand's bid with its rank (765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5). So
/// the total winnings in this example are 6440.
///
/// Find the rank of every hand in your set. What are the total winnings?
///
/// --- Part Two ---
///
/// To make things a little more interesting, the Elf introduces one additional rule. Now, J cards
/// are jokers - wildcards that can act like whatever card would make the hand the strongest type
/// possible.
///
/// To balance this, J cards are now the weakest individual cards, weaker even than 2. The other
/// cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.
///
/// J cards can pretend to be whatever card is best for the purpose of determining hand type; for
/// example, QJJQ2 is now considered four of a kind. However, for the purpose of breaking ties
/// between two hands of the same type, J is always treated as J, not the card it's pretending to
/// be: JKKK2 is weaker than QQQQ2 because J is weaker than Q.
///
/// Now, the above example goes very differently:
///
/// 32T3K 765
/// T55J5 684
/// KK677 28
/// KTJJT 220
/// QQQJA 483
///
///   - 32T3K is still the only one pair; it doesn't contain any jokers, so its strength doesn't
///     increase.
///   - KK677 is now the only two pair, making it the second-weakest hand.
///   - T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3, QQQJA gets rank 4,
///     and KTJJT gets rank 5.
///
/// With the new joker rule, the total winnings in this example are 5905.
///
/// Using the new joker rule, find the rank of every hand in your set. What are the new total
/// winnings?
use nom::character::complete;
use nom::character::complete::{newline, one_of, space1};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_07");

pub fn run() {
    let (_, hands) = separated_list1(newline, Hand::parse)(INPUT).expect("Parsing went wrong");

    let mut valued_hands: Vec<_> = hands.clone().into_iter().map(ValuedHand::from).collect();
    valued_hands.sort();

    let winnings: u32 = valued_hands
        .iter()
        .enumerate()
        .map(|(rank, valued_hand)| (rank + 1) as u32 * valued_hand.hand.bid)
        .sum();

    println!("The total winnings are: {}", winnings);

    let mut valued_hands: Vec<_> = hands
        .into_iter()
        .map(Hand::convert_jack_to_joker)
        .map(ValuedHand::from)
        .collect();
    valued_hands.sort();

    let winnings: u32 = valued_hands
        .iter()
        .enumerate()
        .map(|(rank, valued_hand)| (rank + 1) as u32 * valued_hand.hand.bid)
        .sum();

    println!("The total winnings with jokers are: {}", winnings);
}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
enum Card {
    Joker,
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

impl Card {
    fn jack_to_joker(self) -> Self {
        match self {
            Card::Jack => Card::Joker,
            c => c,
        }
    }
}

impl From<char> for Card {
    fn from(card: char) -> Card {
        match card {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Tried to make a card from an unknown character"),
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Value {
    High,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
struct ValuedHand {
    value: Value,
    hand: Hand,
}

impl Hand {
    fn parse(input: &str) -> IResult<&str, Hand> {
        let (input, (cards, _, bid)) = tuple((
            many1(map(one_of("AKQJT98765432"), Card::from)),
            space1,
            complete::u32,
        ))(input)?;

        Ok((input, Hand { cards, bid }))
    }

    fn convert_jack_to_joker(mut self) -> Self {
        self.cards = self.cards.into_iter().map(Card::jack_to_joker).collect();
        self
    }
}

impl From<Hand> for ValuedHand {
    fn from(hand: Hand) -> Self {
        // count all occurrences
        let card_counts = hand.cards.iter().fold(HashMap::new(), |mut counts, card| {
            if card != &Card::Joker {
                // filter out Jokers
                *counts.entry(card).or_insert(0) += 1;
            }
            counts
        });

        // put the card counts in a vec and sort it with the highest amount first
        let mut sorted_card_counts: Vec<_> = card_counts
            .into_iter()
            .map(|(card, count)| (count, card))
            .collect();
        sorted_card_counts.sort();
        sorted_card_counts.reverse();

        // determine the value
        let value = match sorted_card_counts[..] {
            [(5, _)] => Value::FiveOfAKind,
            [(4, _)] => Value::FiveOfAKind,
            [(3, _)] => Value::FiveOfAKind,
            [(2, _)] => Value::FiveOfAKind,
            [(1, _)] => Value::FiveOfAKind,
            [] => Value::FiveOfAKind,
            [(4, _), (1, _)] => Value::FourOfAKind,
            [(3, _), (1, _)] => Value::FourOfAKind,
            [(2, _), (1, _)] => Value::FourOfAKind,
            [(1, _), (1, _)] => Value::FourOfAKind,
            [(3, _), (2, _)] => Value::FullHouse,
            [(2, _), (2, _)] => Value::FullHouse,
            [(3, _), (1, _), (1, _)] => Value::ThreeOfAKind,
            [(2, _), (1, _), (1, _)] => Value::ThreeOfAKind,
            [(1, _), (1, _), (1, _)] => Value::ThreeOfAKind,
            [(2, _), (2, _), (1, _)] => Value::TwoPairs,
            [(2, _), (1, _), (1, _), (1, _)] => Value::Pair,
            [(1, _), (1, _), (1, _), (1, _)] => Value::Pair,
            [(1, _), (1, _), (1, _), (1, _), (1, _)] => Value::High,
            _ => panic!("Could not match on a known hand value"),
        };

        ValuedHand { value, hand }
    }
}

#[cfg(test)]
mod tests {
    use super::Card::{Ace, Five, Jack, Joker, King, Queen, Seven, Six, Ten, Three, Two};
    use super::*;

    #[test]
    fn test_hand_parse_1() {
        assert_eq!(
            Hand::parse("32T3K 765"),
            Ok((
                "",
                Hand {
                    cards: vec![Three, Two, Ten, Three, King],
                    bid: 765
                }
            ))
        );
    }

    #[test]
    fn test_hand_parse_2() {
        assert_eq!(
            Hand::parse("T55J5 684"),
            Ok((
                "",
                Hand {
                    cards: vec![Ten, Five, Five, Jack, Five],
                    bid: 684
                }
            ))
        );
    }

    #[test]
    fn test_hand_parse_3() {
        assert_eq!(
            Hand::parse("KK677 28"),
            Ok((
                "",
                Hand {
                    cards: vec![King, King, Six, Seven, Seven],
                    bid: 28
                }
            ))
        );
    }

    #[test]
    fn test_hand_parse_4() {
        assert_eq!(
            Hand::parse("KTJJT 220"),
            Ok((
                "",
                Hand {
                    cards: vec![King, Ten, Jack, Jack, Ten],
                    bid: 220
                }
            ))
        );
    }

    #[test]
    fn test_hand_parse_5() {
        assert_eq!(
            Hand::parse("QQQJA 483"),
            Ok((
                "",
                Hand {
                    cards: vec![Queen, Queen, Queen, Jack, Ace],
                    bid: 483
                }
            ))
        );
    }

    #[test]
    fn test_valued_hand_from_1() {
        let hand = Hand {
            cards: vec![Three, Two, Ten, Three, King],
            bid: 765,
        };

        assert_eq!(
            ValuedHand::from(hand.clone()),
            ValuedHand {
                value: Value::Pair,
                hand
            }
        );
    }

    #[test]
    fn test_valued_hand_from_2() {
        let hand = Hand {
            cards: vec![Ten, Five, Five, Jack, Five],
            bid: 684,
        };

        assert_eq!(
            ValuedHand::from(hand.clone()),
            ValuedHand {
                value: Value::ThreeOfAKind,
                hand
            }
        );
    }

    #[test]
    fn test_valued_hand_from_3() {
        let hand = Hand {
            cards: vec![King, King, Six, Seven, Seven],
            bid: 28,
        };

        assert_eq!(
            ValuedHand::from(hand.clone()),
            ValuedHand {
                value: Value::TwoPairs,
                hand
            }
        );
    }

    #[test]
    fn test_valued_hand_from_4() {
        let hand = Hand {
            cards: vec![King, Ten, Jack, Jack, Ten],
            bid: 220,
        };

        assert_eq!(
            ValuedHand::from(hand.clone()),
            ValuedHand {
                value: Value::TwoPairs,
                hand
            }
        );
    }

    #[test]
    fn test_valued_hand_from_5() {
        let hand = Hand {
            cards: vec![Queen, Queen, Queen, Jack, Ace],
            bid: 483,
        };

        assert_eq!(
            ValuedHand::from(hand.clone()),
            ValuedHand {
                value: Value::ThreeOfAKind,
                hand
            }
        );
    }

    #[test]
    fn test_valued_hand_from_joker_1() {
        let hand = Hand {
            cards: vec![Ten, Five, Five, Joker, Five],
            bid: 684,
        };

        assert_eq!(
            ValuedHand::from(hand.clone()),
            ValuedHand {
                value: Value::FourOfAKind,
                hand
            }
        );
    }

    #[test]
    fn test_valued_hand_from_joker_2() {
        let hand = Hand {
            cards: vec![King, Ten, Joker, Joker, Ten],
            bid: 220,
        };

        assert_eq!(
            ValuedHand::from(hand.clone()),
            ValuedHand {
                value: Value::FourOfAKind,
                hand
            }
        );
    }

    #[test]
    fn test_valued_hand_from_joker_3() {
        let hand = Hand {
            cards: vec![Queen, Queen, Queen, Joker, Ace],
            bid: 483,
        };

        assert_eq!(
            ValuedHand::from(hand.clone()),
            ValuedHand {
                value: Value::FourOfAKind,
                hand
            }
        );
    }

    #[test]
    fn test_valued_hand_sort() {
        let mut valued_hands = vec![
            ValuedHand {
                value: Value::Pair,
                hand: Hand {
                    cards: vec![Three, Two, Ten, Three, King],
                    bid: 765,
                },
            },
            ValuedHand {
                value: Value::ThreeOfAKind,
                hand: Hand {
                    cards: vec![Ten, Five, Five, Jack, Five],
                    bid: 684,
                },
            },
            ValuedHand {
                value: Value::TwoPairs,
                hand: Hand {
                    cards: vec![King, King, Six, Seven, Seven],
                    bid: 28,
                },
            },
            ValuedHand {
                value: Value::TwoPairs,
                hand: Hand {
                    cards: vec![King, Ten, Jack, Jack, Ten],
                    bid: 220,
                },
            },
            ValuedHand {
                value: Value::ThreeOfAKind,
                hand: Hand {
                    cards: vec![Queen, Queen, Queen, Jack, Ace],
                    bid: 483,
                },
            },
        ];
        valued_hands.sort();

        let expected_valued_hands = vec![
            ValuedHand {
                value: Value::Pair,
                hand: Hand {
                    cards: vec![Three, Two, Ten, Three, King],
                    bid: 765,
                },
            },
            ValuedHand {
                value: Value::TwoPairs,
                hand: Hand {
                    cards: vec![King, Ten, Jack, Jack, Ten],
                    bid: 220,
                },
            },
            ValuedHand {
                value: Value::TwoPairs,
                hand: Hand {
                    cards: vec![King, King, Six, Seven, Seven],
                    bid: 28,
                },
            },
            ValuedHand {
                value: Value::ThreeOfAKind,
                hand: Hand {
                    cards: vec![Ten, Five, Five, Jack, Five],
                    bid: 684,
                },
            },
            ValuedHand {
                value: Value::ThreeOfAKind,
                hand: Hand {
                    cards: vec![Queen, Queen, Queen, Jack, Ace],
                    bid: 483,
                },
            },
        ];

        assert_eq!(valued_hands, expected_valued_hands);
    }

    #[test]
    fn test_valued_hand_sort_jokers() {
        let mut valued_hands = vec![
            ValuedHand {
                value: Value::Pair,
                hand: Hand {
                    cards: vec![Three, Two, Ten, Three, King],
                    bid: 765,
                },
            },
            ValuedHand {
                value: Value::FourOfAKind,
                hand: Hand {
                    cards: vec![Ten, Five, Five, Joker, Five],
                    bid: 684,
                },
            },
            ValuedHand {
                value: Value::TwoPairs,
                hand: Hand {
                    cards: vec![King, King, Six, Seven, Seven],
                    bid: 28,
                },
            },
            ValuedHand {
                value: Value::FourOfAKind,
                hand: Hand {
                    cards: vec![King, Ten, Joker, Joker, Ten],
                    bid: 220,
                },
            },
            ValuedHand {
                value: Value::FourOfAKind,
                hand: Hand {
                    cards: vec![Queen, Queen, Queen, Joker, Ace],
                    bid: 483,
                },
            },
        ];
        valued_hands.sort();

        let expected_valued_hands = vec![
            ValuedHand {
                value: Value::Pair,
                hand: Hand {
                    cards: vec![Three, Two, Ten, Three, King],
                    bid: 765,
                },
            },
            ValuedHand {
                value: Value::TwoPairs,
                hand: Hand {
                    cards: vec![King, King, Six, Seven, Seven],
                    bid: 28,
                },
            },
            ValuedHand {
                value: Value::FourOfAKind,
                hand: Hand {
                    cards: vec![Ten, Five, Five, Joker, Five],
                    bid: 684,
                },
            },
            ValuedHand {
                value: Value::FourOfAKind,
                hand: Hand {
                    cards: vec![Queen, Queen, Queen, Joker, Ace],
                    bid: 483,
                },
            },
            ValuedHand {
                value: Value::FourOfAKind,
                hand: Hand {
                    cards: vec![King, Ten, Joker, Joker, Ten],
                    bid: 220,
                },
            },
        ];

        assert_eq!(valued_hands, expected_valued_hands);
    }
}
