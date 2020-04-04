//! Solution to Advent of Code 2019 [Day 16](https://adventofcode.com/2019/day/16).

mod profiling;

use profiling::Timer;
use std::iter::repeat;

fn main() {
    let part1 = day16_part1();
    println!("part1 = {}", part1);

    let part2 = day16_part2();
    println!("part2 = {}", part2);
}

fn day16_part1() -> String {
    let _part1_timer = Timer::new("part1");
    first_eight_after_100_phases(DAY16_INPUT)
}

fn day16_part2() -> String {
    let _part1_timer = Timer::new("part2");

    let offset = DAY16_INPUT[..7].parse::<usize>().unwrap();
    let mut components = DAY16_INPUT
        .repeat(10_000)
        .chars()
        .skip(offset)
        .map(|d| d.to_digit(10).unwrap() as Digit)
        .collect::<Vec<_>>();

    components.reverse();

    let len = components.len();
    for _ in 0..100 {
        let mut sum = 0;
        let mut next = Vec::<Digit>::with_capacity(len);
        for c in &components {
            sum += c;
            sum %= 10;
            next.push(sum);
        }
        components = next;
    }

    components.reverse();
    components
        .into_iter()
        .take(8)
        .map(|d| std::char::from_digit(d as u32, 10).unwrap())
        .collect()
}

const DAY16_INPUT: &str = include_str!("day16_input.txt");

fn first_eight_after_100_phases(signal: &str) -> String {
    let mut transform = Transform::new(signal);
    for _ in 0..100 {
        transform.advance();
    }
    let out = transform.signal();
    String::from(&out[..8])
}

type Digit = i8;

#[derive(Debug)]
struct Transform {
    components: Vec<Digit>,
    patterns: Vec<Pattern>,
}

impl Transform {
    fn new(signal: &str) -> Transform {
        let components = signal
            .chars()
            .map(|d| d.to_digit(10).unwrap() as Digit)
            .collect::<Vec<_>>();

        let signal_length = components.len();

        let patterns = (0..signal_length)
            .map(|i| Pattern::new(i, signal_length))
            .collect();

        Transform {
            components,
            patterns,
        }
    }

    fn advance(&mut self) {
        self.components = self
            .patterns
            .iter()
            .map(|p| p.multiply(&self.components))
            .collect();
    }

    fn signal(&self) -> String {
        self.components
            .iter()
            .map(|&d| std::char::from_digit(d as u32, 10).unwrap())
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Pattern {
    digit_index: usize,
    values: Box<[Digit]>,
}

impl Pattern {
    fn new(digit_index: usize, length: usize) -> Pattern {
        const BASE_PATTERN: [Digit; 4] = [0, 1, 0, -1];

        let values = BASE_PATTERN
            .iter()
            .copied()
            .cycle()
            .map(repeat)
            .flat_map(|it| it.take(digit_index + 1))
            .skip(digit_index + 1)
            .take(length - digit_index)
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Pattern {
            digit_index,
            values,
        }
    }

    fn multiply(&self, components: &[Digit]) -> Digit {
        // all of the initial sequence to index digit_index are zeros,
        // so we can optimise by skipping them
        let offset = self.digit_index;
        let end = self.values.len();

        let mut sum = 0;
        let mut i = 0;
        while i < end {
            sum += (self.values[i] * components[i + offset]) as i64;
            i += 1;
        }

        let result = sum.abs() % 10;
        result as Digit
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transform() {
        let mut transform = Transform::new("12345678");
        transform.advance();
        assert_eq!(transform.signal(), "48226158");
        transform.advance();
        assert_eq!(transform.signal(), "34040438");
        transform.advance();
        assert_eq!(transform.signal(), "03415518");
        transform.advance();
        assert_eq!(transform.signal(), "01029498");

        assert_eq!(
            first_eight_after_100_phases("80871224585914546619083218645595"),
            String::from("24176176")
        );

        assert_eq!(
            first_eight_after_100_phases("19617804207202209144916044189917"),
            String::from("73745418")
        );

        assert_eq!(
            first_eight_after_100_phases("69317163492948606335995924319873"),
            String::from("52432133")
        );
    }

    #[test]
    fn test_day16() {
        let part1 = day16_part1();
        assert_eq!(part1, "12541048");

        let part2 = day16_part2();
        assert_eq!(part2, "62858988");
    }
}
