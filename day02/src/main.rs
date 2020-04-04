//! Solution to Advent of Code 2019 [Day 2](https://adventofcode.com/2019/day/2).

use aoc::intcode::{Machine, Program};
use once_cell::sync::Lazy;

static DAY02_PROGRAM: Lazy<Program> = Lazy::new(|| {
    let input = include_str!("day02_input.txt");
    Program::from(input)
});

fn run_machine(program: &Program, noun: i64, verb: i64) -> i64 {
    let mut p = (*program).clone();
    p.write(1, noun);
    p.write(2, verb);
    let mut m = Machine::new(&p);
    m.run();
    m.read(0)
}

fn day02_part1() -> i64 {
    run_machine(&DAY02_PROGRAM, 12, 2)
}

fn day02_part2() -> i64 {
    let target = 19_690_720;
    for n in 0..100 {
        for v in 0..100 {
            let out = run_machine(&DAY02_PROGRAM, n, v);
            if out == target {
                return (100 * n) + v;
            }
        }
    }
    panic!("Failed to find answer");
}

#[test]
fn test_day02() {
    assert_eq!(day02_part1(), 11_590_668);
    assert_eq!(day02_part2(), 2254);
}

fn main() {
    println!("part1 = {}", day02_part1());
    println!("part2 = {}", day02_part2());
}
