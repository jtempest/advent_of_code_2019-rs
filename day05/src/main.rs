//! Solution to Advent of Code 2019 [Day 5](https://adventofcode.com/2019/day/5).

use aoc::intcode::Machine;

const DAY05_INPUT: &str = include_str!("day05_input.txt");

fn day05_part1() -> i64 {
    let output = Machine::from_source_with_input(DAY05_INPUT, 1)
        .run_as_iter()
        .collect::<Vec<_>>();
    assert!(!output.is_empty());
    let (last, rest) = output.split_last().unwrap();
    assert!(rest.iter().all(|o| *o == 0), "Failed a TEST");
    *last
}

fn day05_part2() -> i64 {
    Machine::from_source_with_input(DAY05_INPUT, 5)
        .run()
        .unwrap()
}

#[test]
fn test_day05() {
    assert_eq!(day05_part1(), 13_933_662);
    assert_eq!(day05_part2(), 2_369_720);
}

fn main() {
    println!("part1 = {}", day05_part1());
    println!("part2 = {}", day05_part2());
}
