use std::{cmp::Ordering, collections::HashMap};

use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, one_of},
    combinator::map_res,
    multi::many_m_n,
    sequence::separated_pair,
    IResult,
};

use crate::puzzle::parse_lines_to_vec;

pub fn part1(input: &str) -> Result<usize> {
    let mut hands = parse_lines_to_vec(input, parse_hand)?;
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
        let card_counts = cards
            .iter()
            .fold(HashMap::new(), |mut acc, c| {
                acc.entry(c).and_modify(|count| *count += 1).or_insert(1);
                acc
            })
            .into_values()
            .collect::<Vec<_>>();
        if card_counts.iter().any(|c| *c == 5) {
            return Ok(HandType::FiveOfAKind);
        }
        if card_counts.iter().any(|c| *c == 4) {
            return Ok(HandType::FourOfAKind);
        }
        let has_pair = card_counts.iter().any(|c| *c == 2);
        if card_counts.iter().any(|c| *c == 3) {
            if has_pair {
                return Ok(HandType::FullHouse);
            }
            return Ok(HandType::ThreeOfAKind);
        }
        if has_pair {
            if card_counts.iter().filter(|c| **c == 2).count() == 2 {
                return Ok(Self::TwoPair);
            }
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

impl TryFrom<char> for Card {
    type Error = String;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        let card = match value {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => return Err(format!("Not a valid Card value: {}", value)),
        };
        Ok(card)
    }
}

fn number(digits: &str) -> IResult<&str, usize> {
    map_res(digit1, |n: &str| n.parse::<usize>())(digits)
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let cards = many_m_n(5, 5, map_res(one_of("AKQJT98765432"), Card::try_from));

    map_res(separated_pair(cards, tag(" "), number), Hand::try_from)(input)
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

    // #[test]
    // fn test_part2_gives_correct_answer() {
    //     let res = part2(INPUT).unwrap();
    //     assert_eq!(res, 71503);
    // }
}
