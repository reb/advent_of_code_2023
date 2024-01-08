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
use nom::bytes::complete::is_a;
use nom::character::complete;
use nom::character::complete::{newline, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;
use std::str::FromStr;

const INPUT: &str = include_str!("../input/day_07");

pub fn run() {
    let (_, mut hands) =
        separated_list1(newline, Hand::from_str)(INPUT).expect("Parsing went wrong");
    hands.sort();

    let winnings: u32 = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) as u32 * hand.bid)
        .sum();

    println!("The total winnings are: {}", winnings);
}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
enum Card {
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

impl From<char> for Card {
    fn from(card: char) -> Self {
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
    High(Vec<Card>),
    Pair(Vec<Card>),
    TwoPairs(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FiveOfAKind(Vec<Card>),
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Hand {
    value: Value,
    bid: u32,
}

impl Hand {
    fn from_str(input: &str) -> IResult<&str, Hand> {
        let (input, (value, _, bid)) = tuple((
            map_res(is_a("AKQJT98765432"), Value::from_str),
            space1,
            complete::u32,
        ))(input)?;

        Ok((input, Hand { value, bid }))
    }
}

impl FromStr for Value {
    type Err = String;

    fn from_str(hand: &str) -> Result<Self, Self::Err> {
        // transform the cards to a vec
        let cards: Vec<Card> = hand.chars().map(Card::from).collect();

        // count all occurrences
        let card_counts = cards.iter().fold(HashMap::new(), |mut counts, card| {
            *counts.entry(card).or_insert(0) += 1;
            counts
        });

        // put the card counts in a vec and sort it with the highest amount first
        let mut sorted_card_counts: Vec<_> = card_counts
            .into_iter()
            .map(|(card, count)| (count, card))
            .collect();
        sorted_card_counts.sort();
        sorted_card_counts.reverse();

        match sorted_card_counts[..] {
            [(5, _)] => Ok(Value::FiveOfAKind(cards)),
            [(4, _), (1, _)] => Ok(Value::FourOfAKind(cards)),
            [(3, _), (2, _)] => Ok(Value::FullHouse(cards)),
            [(3, _), (1, _), (1, _)] => Ok(Value::ThreeOfAKind(cards)),
            [(2, _), (2, _), (1, _)] => Ok(Value::TwoPairs(cards)),
            [(2, _), (1, _), (1, _), (1, _)] => Ok(Value::Pair(cards)),
            [(1, _), (1, _), (1, _), (1, _), (1, _)] => Ok(Value::High(cards)),
            _ => Err("Could not match on a known hand value".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Card::{
        Ace, Eight, Five, Four, Jack, King, Nine, Queen, Seven, Six, Ten, Three, Two,
    };
    use super::*;

    #[test]
    fn test_value_from_str_1() {
        assert_eq!(
            Value::from_str("32T3K"),
            Ok(Value::Pair(vec![Three, Two, Ten, Three, King]))
        );
    }

    #[test]
    fn test_value_from_str_2() {
        assert_eq!(
            Value::from_str("T55J5"),
            Ok(Value::ThreeOfAKind(vec![Ten, Five, Five, Jack, Five]))
        );
    }

    #[test]
    fn test_value_from_str_3() {
        assert_eq!(
            Value::from_str("KK677"),
            Ok(Value::TwoPairs(vec![King, King, Six, Seven, Seven]))
        );
    }

    #[test]
    fn test_value_from_str_4() {
        assert_eq!(
            Value::from_str("KTJJT"),
            Ok(Value::TwoPairs(vec![King, Ten, Jack, Jack, Ten]))
        );
    }

    #[test]
    fn test_value_from_str_5() {
        assert_eq!(
            Value::from_str("QQQJA"),
            Ok(Value::ThreeOfAKind(vec![Queen, Queen, Queen, Jack, Ace]))
        );
    }

    #[test]
    fn test_hand_sort() {
        let mut hands = vec![
            Hand {
                value: Value::Pair(vec![Three, Two, Ten, Three, King]),
                bid: 765,
            },
            Hand {
                value: Value::ThreeOfAKind(vec![Ten, Five, Five, Jack, Five]),
                bid: 684,
            },
            Hand {
                value: Value::TwoPairs(vec![King, King, Six, Seven, Seven]),
                bid: 28,
            },
            Hand {
                value: Value::TwoPairs(vec![King, Ten, Jack, Jack, Ten]),
                bid: 220,
            },
            Hand {
                value: Value::ThreeOfAKind(vec![Queen, Queen, Queen, Jack, Ace]),
                bid: 483,
            },
        ];
        hands.sort();

        let expected_hands = vec![
            Hand {
                value: Value::Pair(vec![Three, Two, Ten, Three, King]),
                bid: 765,
            },
            Hand {
                value: Value::TwoPairs(vec![King, Ten, Jack, Jack, Ten]),
                bid: 220,
            },
            Hand {
                value: Value::TwoPairs(vec![King, King, Six, Seven, Seven]),
                bid: 28,
            },
            Hand {
                value: Value::ThreeOfAKind(vec![Ten, Five, Five, Jack, Five]),
                bid: 684,
            },
            Hand {
                value: Value::ThreeOfAKind(vec![Queen, Queen, Queen, Jack, Ace]),
                bid: 483,
            },
        ];

        assert_eq!(hands, expected_hands);
    }
}
