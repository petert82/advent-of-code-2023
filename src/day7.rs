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
    let hands = parse_lines_to_vec(input, parse_hand)?;
    dbg!(hands);
    Ok(1)
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug)]
struct Hand {
    cards: (Card, Card, Card, Card, Card),
    bid: usize,
}

impl Card {
    pub fn strength(&self) -> usize {
        *self as usize
    }
}

impl TryFrom<(Vec<Card>, usize)> for Hand {
    type Error = String;

    fn try_from(value: (Vec<Card>, usize)) -> std::result::Result<Self, Self::Error> {
        let (cards, bid) = value;
        if cards.len() != 5 {
            return Err(format!("Cannot build a hand with {} cards", cards.len()));
        }
        Ok(Hand {
            cards: (cards[0], cards[1], cards[2], cards[3], cards[4]),
            bid,
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
