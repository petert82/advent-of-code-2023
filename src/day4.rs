use std::collections::{HashMap, HashSet};

use anyhow::Result;
use nom::{
    bytes::complete::{is_a, tag},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

use crate::parse::{number, parse_lines_to_vec};

pub fn part1(input: &str) -> Result<usize> {
    let cards = parse_lines_to_vec::<Card>(input, parse_card)?;
    let res = cards
        .iter()
        .map(Card::get_match_count)
        .filter(|c| *c > 0)
        .map(|c| 2_usize.pow((c - 1) as u32))
        .sum();
    Ok(res)
}

pub fn part2(input: &str) -> Result<usize> {
    let cards = parse_lines_to_vec::<Card>(input, parse_card)?;
    // how many of each card we have, indexed by ID
    let mut card_instances: HashMap<usize, usize> = cards.iter().map(|c| (c.id, 1)).collect();

    for card in cards.iter() {
        let mut extra_cards = card.get_match_count();
        let multiplier = *card_instances.get(&card.id()).unwrap();
        while extra_cards > 0 {
            let idx = card.id() + extra_cards;
            card_instances
                .entry(idx)
                .and_modify(|instance_count| *instance_count += multiplier);
            extra_cards -= 1;
        }
    }

    Ok(card_instances.values().sum())
}

#[derive(Debug)]
struct Card {
    id: usize,
    winners: HashSet<usize>,
    numbers: HashSet<usize>,
}

impl Card {
    pub fn get_match_count(&self) -> usize {
        self.winners.intersection(&self.numbers).count()
    }

    pub fn id(&self) -> usize {
        self.id
    }
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
