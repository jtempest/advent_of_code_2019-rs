//! Solution to Advent of Code 2019 [Day 9](https://adventofcode.com/2019/day/9).

use aoc::intcode::{Machine, Program};

const DAY09_INPUT: &str = include_str!("day09_input.txt");

fn day09() -> (i64, i64) {
    let program = Program::from(DAY09_INPUT);
    let part1 = Machine::new(&program).run_with_input(1).unwrap();
    let part2 = Machine::new(&program).run_with_input(2).unwrap();
    (part1, part2)
}

#[test]
fn test_day09() {
    let (part1, part2) = day09();
    assert_eq!(part1, 2_351_176_124);
    assert_eq!(part2, 73_110);
}

fn main() {
    let (part1, part2) = day09();
    println!("part1 = {}", part1);
    println!("part2 = {}", part2);
}
