//! Solution to Advent of Code 2019 [Day 22](https://adventofcode.com/2019/day/22).
//!
//! Based on the maths in [this comment on the subreddit](https://www.reddit.com/r/adventofcode/comments/ee0rqi/2019_day_22_solutions/fbnkaju/).

mod mod_num;

use mod_num::{ModNum, Modulo};
use num::{BigInt, Integer};
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

const DAY22_INPUT: &str = include_str!("day22_input.txt");

fn main() {
    println!("part1 = {}", day22_part1());
    println!("part2 = {}", day22_part2());
}

fn day22_part1() -> usize {
    let shuffled = Deck::with_shuffles(10_007, DAY22_INPUT).unwrap();
    shuffled.find_card(2019).unwrap()
}

fn day22_part2() -> u64 {
    let size = 119_315_717_514_047;
    let n = 101_741_582_076_661;
    let shuffled = Deck::with_shuffles_n_times(size, DAY22_INPUT, n).unwrap();
    shuffled.nth_card(2020).unwrap()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Deck {
    size: u64,
    offset: ModNum,
    increment: ModNum,
}

impl Deck {
    fn new(size: u64) -> Deck {
        Deck {
            size,
            offset: 0.modulo(size),
            increment: 1.modulo(size),
        }
    }

    fn nth_card(&self, n: u64) -> Option<u64> {
        if n < self.size {
            let n = n.modulo(self.size);
            let result = self.offset.clone() + (self.increment.clone() * n);
            result.value()
        } else {
            None
        }
    }

    fn with_shuffles(size: u64, shuffles: &str) -> Result<Deck, String> {
        let mut deck = Deck::new(size);
        for t in parse_techniques(shuffles)?.into_iter() {
            deck.shuffle(t);
        }
        Ok(deck)
    }

    fn with_shuffles_n_times(size: u64, shuffles: &str, n: u64) -> Result<Deck, String> {
        let Deck {
            increment: increment_mul,
            offset: offset_diff,
            ..
        } = Deck::with_shuffles(size, shuffles)?;

        let increment_mul = increment_mul.big_value();
        let offset_diff = offset_diff.big_value();

        let n = BigInt::from(n);
        let big_size = BigInt::from(size);

        // increment = pow(increment_mul, iterations, cards)
        let increment = increment_mul.modpow(&n, &big_size);

        // offset = offset_diff * (1 - increment) * inv((1 - increment_mul) % cards)
        let inv = (BigInt::from(1) - increment_mul).mod_floor(&big_size);
        let inv = inv.modpow(&(&big_size - 2), &big_size);
        let offset = offset_diff * (BigInt::from(1) - &increment) * inv;
        let offset = offset.mod_floor(&big_size);

        Ok(Deck {
            size,
            increment: increment.modulo(size),
            offset: offset.modulo(size),
        })
    }

    fn shuffle(&mut self, technique: Technique) {
        match technique {
            Technique::Reverse => {
                self.increment *= (-1).modulo(self.size);
                self.offset += self.increment.clone();
            }
            Technique::Cut(n) => {
                self.offset += self.increment.clone() * n.modulo(self.size);
            }
            Technique::Deal(n) => {
                self.increment *= n.modulo(self.size).inv();
            }
        }
    }

    fn find_card(&self, value: u64) -> Option<usize> {
        self.iter().position(|x| x == value)
    }

    fn iter(&self) -> DeckIter {
        DeckIter {
            deck: self.clone(),
            n: 0,
        }
    }
}

struct DeckIter {
    deck: Deck,
    n: u64,
}

impl Iterator for DeckIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.deck.nth_card(self.n);
        self.n += 1;

        let iter_length = self.deck.size + 1;
        self.n = self.n.mod_floor(&iter_length);

        result
    }
}

impl TryFrom<Vec<u64>> for Deck {
    type Error = String;

    fn try_from(cards: Vec<u64>) -> Result<Self, Self::Error> {
        let size: u64 = cards.len().try_into().unwrap();
        if primes::is_prime(size) {
            let card0 = cards[0].modulo(size);
            let card1 = cards[1].modulo(size);
            let deck = Deck {
                size,
                offset: card0.clone(),
                increment: card1 - card0,
            };
            println!("{:?}", deck.iter().collect::<Vec<_>>());
            if deck.iter().eq(cards.iter().copied()) {
                Ok(deck)
            } else {
                Err("Deck cannot be represented".into())
            }
        } else {
            Err("Non-prime deck sizes are not allowed".into())
        }
    }
}

#[derive(Debug)]
enum Technique {
    Reverse,   // deal into new stack
    Cut(i64),  // cut N cards
    Deal(u64), // deal with increment N
}

impl TryFrom<&str> for Technique {
    type Error = String;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let line = line.trim();
        if line.starts_with("deal into new stack") {
            Ok(Technique::Reverse)
        } else if line.starts_with("cut ") {
            Ok(Technique::Cut(parse_number::<i64>(line)?))
        } else if line.starts_with("deal with increment") {
            Ok(Technique::Deal(parse_number::<u64>(line)?))
        } else {
            Err(format!("Unknown instruction '{}'", line))
        }
    }
}

fn parse_number<T: FromStr>(line: &str) -> Result<T, String> {
    line.split_ascii_whitespace()
        .last()
        .map(|word| word.parse::<T>())
        .unwrap()
        .map(Ok)
        .map_err(|_| "Missing N")?
}

fn parse_techniques(input: &str) -> Result<Vec<Technique>, String> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        instructions.push(Technique::try_from(line)?);
    }
    Ok(instructions)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deal_into_new_stack() {
        let mut deck = Deck::new(11);
        deck.shuffle(Technique::try_from("deal into new stack").unwrap());
        assert_eq!(
            deck,
            vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0].try_into().unwrap()
        );
    }

    #[test]
    fn test_cut_n_cards() {
        let mut deck = Deck::new(11);
        deck.shuffle(Technique::try_from("cut 3").unwrap());
        assert_eq!(
            deck,
            vec![3, 4, 5, 6, 7, 8, 9, 10, 0, 1, 2].try_into().unwrap()
        );

        let mut deck = Deck::new(11);
        deck.shuffle(Technique::try_from("cut -4").unwrap());
        assert_eq!(
            deck,
            vec![7, 8, 9, 10, 0, 1, 2, 3, 4, 5, 6].try_into().unwrap()
        );
    }

    #[test]
    fn test_deal_with_increment() {
        let mut deck = Deck::new(11);
        deck.shuffle(Technique::try_from("deal with increment 3").unwrap());
        assert_eq!(
            deck,
            vec![0, 4, 8, 1, 5, 9, 2, 6, 10, 3, 7].try_into().unwrap()
        );
    }

    #[test]
    fn test_day22() {
        assert_eq!(day22_part1(), 3939);
        assert_eq!(day22_part2(), 55_574_110_161_534);
    }
}
