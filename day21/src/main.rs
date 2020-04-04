//! Solution to Advent of Code 2019 [Day 21](https://adventofcode.com/2019/day/21).

use aoc::intcode::Machine;

const DAY21_INPUT: &str = include_str!("day21_input.txt");
const PART1_PROGRAM: &str = include_str!("day21_part1_program.txt");
const PART2_PROGRAM: &str = include_str!("day21_part2_program.txt");

fn main() {
    println!("part1 = {}", day21_part1());
    println!("part2 = {}", day21_part2());
}

fn day21_part1() -> i64 {
    run_program(PART1_PROGRAM)
}

fn day21_part2() -> i64 {
    run_program(PART2_PROGRAM)
}

fn run_program(program: &str) -> i64 {
    let mut machine = Machine::from_source(DAY21_INPUT);
    let _prompt = machine.run_as_ascii();
    program
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| machine.input_ascii(line));
    machine.run_as_iter().last().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day21() {
        assert_eq!(day21_part1(), 19_362_259);
        assert_eq!(day21_part2(), 1_141_066_762);
    }
}
