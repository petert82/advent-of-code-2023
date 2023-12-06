use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::{bail, Result};
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{digit1, line_ending},
    combinator::{all_consuming, map_res, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

pub fn part1(input: &str) -> Result<usize> {
    let Ok((_, cards)) = parse_cards(input) else {
        bail!("could not parse input")
    };
    let res = cards
        .iter()
        .map(Card::get_match_count)
        .filter(|c| *c > 0)
        .map(|c| 2_usize.pow((c - 1) as u32))
        .sum();
    Ok(res)
}

pub fn part2(input: &str) -> Result<usize> {
    let Ok((_, cards)) = parse_cards(input) else {
        bail!("could not parse input")
    };
    let mut to_process: VecDeque<CardRef> = cards.iter().map(Card::card_ref).collect();
    let card_index: HashMap<usize, Card> = cards.into_iter().map(|c| (c.id, c)).collect();
    let mut card_count = 0;

    loop {
        if to_process.len() == 0 {
            break;
        }

        let Some(card_ref) = to_process.pop_front() else {
            break;
        };
        card_count += 1;

        let card = card_index.get(&card_ref.id).unwrap();
        let mut extra_cards = card.get_match_count();
        while extra_cards > 0 {
            let idx = card_ref.id + extra_cards;
            if let Some(dup_card) = card_index.get(&idx) {
                to_process.push_back(dup_card.card_ref());
            }
            extra_cards -= 1;
        }
    }

    Ok(card_count)
}

#[derive(Debug)]
struct Card {
    id: usize,
    winners: HashSet<usize>,
    numbers: HashSet<usize>,
}

struct CardRef {
    id: usize,
}

impl Card {
    fn get_match_count(&self) -> usize {
        self.winners.intersection(&self.numbers).count()
    }

    fn card_ref(&self) -> CardRef {
        CardRef { id: self.id }
    }
}

fn number(digits: &str) -> IResult<&str, usize> {
    map_res(digit1, |n: &str| n.parse::<usize>())(digits)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = is_a(" ")(input)?;
    let (input, card_id) = number(input)?;
    let (input, _) = is_a(": ")(input)?;
    let (input, (winners, numbers)) = separated_pair(
        separated_list1(is_a(" "), number),
        terminated(tag(" |"), is_a(" ")),
        separated_list1(is_a(" "), number),
    )(input)?;
    let card = Card {
        id: card_id,
        winners: HashSet::from_iter(winners),
        numbers: HashSet::from_iter(numbers),
    };
    Ok((input, card))
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    all_consuming(terminated(
        separated_list1(line_ending, parse_card),
        opt(line_ending),
    ))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT).unwrap();
        assert_eq!(res, 13);
    }

    #[test]
    fn test_part2_gives_correct_answer() {
        let res = part2(INPUT).unwrap();
        assert_eq!(res, 30);
    }
}
