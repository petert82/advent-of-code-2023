use std::{cmp::Ordering, collections::HashMap};

use anyhow::Result;
use nom::{
    bytes::complete::tag, character::complete::one_of, combinator::map_res, multi::count,
    sequence::separated_pair, IResult,
};

use crate::parse::{number, parse_lines_to_vec};

pub fn part1(input: &str) -> Result<usize> {
    run_with_rules(input, ParsingRules::PartOne)
}

pub fn part2(input: &str) -> Result<usize> {
    run_with_rules(input, ParsingRules::PartTwo)
}

fn run_with_rules(input: &str, rules: ParsingRules) -> Result<usize> {
    let mut hands = parse_lines_to_vec(input, parse_hand(rules))?;
    hands.sort();
    let res = hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();
    Ok(res)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
    hand_type: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // If the hand types are different, that's all that matters and we
        // return their ordering
        let hand_type_cmp = self.hand_type.cmp(&other.hand_type);
        if hand_type_cmp != Ordering::Equal {
            return hand_type_cmp;
        }
        // Else we have to compare the cards in each position in each hand
        // until we find a pair that is different
        for (a, b) in self.cards.iter().zip(&other.cards) {
            let card_cmp = a.cmp(b);
            if card_cmp == Ordering::Equal {
                continue;
            }
            return card_cmp;
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl TryFrom<&[Card]> for HandType {
    type Error = String;

    fn try_from(cards: &[Card]) -> Result<Self, Self::Error> {
        if cards.len() != 5 {
            return Err(format!("Cannot get HandType for {} cards", cards.len()));
        }
        let mut card_count_map = cards.iter().fold(HashMap::new(), |mut acc, c| {
            acc.entry(c).and_modify(|count| *count += 1).or_insert(1);
            acc
        });
        let joker_count = card_count_map.get(&Card::Joker).unwrap_or(&0).clone();
        card_count_map.remove(&Card::Joker);
        // count of card types, excluding Jokers
        let card_counts = card_count_map.into_values().collect::<Vec<_>>();
        let has_five_same = card_counts.iter().any(|c| *c == 5);
        let has_four_same = card_counts.iter().any(|c| *c == 4);
        let has_three_same = card_counts.iter().any(|c| *c == 3);
        let has_two_same = card_counts.iter().any(|c| *c == 2);

        if has_five_same
            || joker_count == 5
            || has_four_same && joker_count == 1
            || has_three_same && joker_count == 2
            || has_two_same && joker_count == 3
            || joker_count == 4
        {
            return Ok(HandType::FiveOfAKind);
        }

        if has_four_same
            || joker_count == 4
            || has_three_same && joker_count == 1
            || has_two_same && joker_count == 2
            || joker_count == 3
        {
            return Ok(HandType::FourOfAKind);
        }
        let pair_count = card_counts.iter().filter(|c| **c == 2).count();
        if (has_three_same && has_two_same) || (has_two_same && joker_count == 1 && pair_count == 2)
        {
            return Ok(Self::FullHouse);
        }
        if has_three_same || joker_count == 2 || has_two_same && joker_count == 1 {
            return Ok(HandType::ThreeOfAKind);
        }
        if has_two_same && pair_count == 2 {
            return Ok(Self::TwoPair);
        }
        if has_two_same || joker_count == 1 {
            return Ok(Self::OnePair);
        }
        return Ok(Self::HighCard);
    }
}

impl TryFrom<(Vec<Card>, usize)> for Hand {
    type Error = String;

    fn try_from(value: (Vec<Card>, usize)) -> std::result::Result<Self, Self::Error> {
        let (cards, bid) = value;
        if cards.len() != 5 {
            return Err(format!("Cannot build a hand with {} cards", cards.len()));
        }
        let hand_type = HandType::try_from(cards.as_slice())?;
        Ok(Hand {
            cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
            bid,
            hand_type,
        })
    }
}

impl TryFrom<(ParsingRules, char)> for Card {
    type Error = String;

    fn try_from(value: (ParsingRules, char)) -> std::result::Result<Self, Self::Error> {
        let (rules, c) = value;
        let card = match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => match rules {
                ParsingRules::PartOne => Card::Jack,
                ParsingRules::PartTwo => Card::Joker,
            },
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => return Err(format!("Not a valid Card value: {}", c)),
        };
        Ok(card)
    }
}

#[derive(Clone, Copy)]
enum ParsingRules {
    PartOne,
    PartTwo,
}

fn parse_hand(rules: ParsingRules) -> impl FnMut(&str) -> IResult<&str, Hand> {
    move |input| {
        let cards = count(
            map_res(one_of("AKQJT98765432"), |c| Card::try_from((rules, c))),
            5,
        );

        map_res(separated_pair(cards, tag(" "), number), Hand::try_from)(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT).unwrap();
        assert_eq!(res, 6440);
    }

    #[test]
    fn test_part2_gives_correct_answer() {
        let res = part2(INPUT).unwrap();
        assert_eq!(res, 5905);
    }

    #[test]
    fn test_joker_hands() {
        let hands = vec![
            ("JJJJJ 1", HandType::FiveOfAKind),
            ("JJJJ8 1", HandType::FiveOfAKind),
            ("JJJ88 1", HandType::FiveOfAKind),
            ("JJ888 1", HandType::FiveOfAKind),
            ("J8888 1", HandType::FiveOfAKind),
            ("J8888 1", HandType::FiveOfAKind),
            ("88888 1", HandType::FiveOfAKind),
            ("JJJ87 1", HandType::FourOfAKind),
            ("JJ887 1", HandType::FourOfAKind),
            ("J8887 1", HandType::FourOfAKind),
            ("88887 1", HandType::FourOfAKind),
            ("J8877 1", HandType::FullHouse),
            ("88877 1", HandType::FullHouse),
            ("88876 1", HandType::ThreeOfAKind),
            ("JJ876 1", HandType::ThreeOfAKind),
            ("J8876 1", HandType::ThreeOfAKind),
            ("88776 1", HandType::TwoPair),
            ("88765 1", HandType::OnePair),
            ("J8765 1", HandType::OnePair),
            ("87654 1", HandType::HighCard),
        ];
        for (hand_str, expect) in hands.into_iter() {
            let Ok((_, hand)) = parse_hand(ParsingRules::PartTwo)(hand_str) else {
                panic!("parsing failed");
            };
            assert_eq!(
                hand.hand_type, expect,
                "expected {} to have type {:?} but got {:?}",
                hand_str, expect, hand.hand_type
            );
        }
    }
}
