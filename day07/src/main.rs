//! Solution to Advent of Code 2019 [Day 7](https://adventofcode.com/2019/day/7).

use aoc::intcode::{Machine, Program};
use itertools::Itertools;
use std::cmp;

struct Amplifier(Vec<Machine>);

impl Amplifier {
    fn new(program: &Program, settings: &[i64]) -> Amplifier {
        Amplifier(
            settings
                .iter()
                .map(|&s| Machine::with_input(&program, s))
                .collect(),
        )
    }

    fn run(&mut self) -> i64 {
        self.run_with_amplitude(0)
    }

    fn run_feedback(&mut self) -> i64 {
        let mut amplitude = 0;
        while !self.is_halted() {
            amplitude = self.run_with_amplitude(amplitude);
        }
        amplitude
    }

    fn run_with_amplitude(&mut self, initial_amplitude: i64) -> i64 {
        self.0.iter_mut().fold(initial_amplitude, |amp, m| {
            m.run_with_input(amp).unwrap_or(amp)
        })
    }

    fn is_halted(&self) -> bool {
        self.0.last().unwrap().is_halted()
    }
}

fn max_signal<R: Iterator<Item = i64>, F: Fn(&mut Amplifier) -> i64>(
    program: &Program,
    settings: R,
    run_func: F,
) -> i64 {
    let num_settings = settings.size_hint().1.unwrap();
    (settings)
        .permutations(num_settings)
        .fold(0, |max, settings| {
            cmp::max(max, run_func(&mut Amplifier::new(&program, &settings)))
        })
}

fn max_thruster_signal(program: &Program) -> i64 {
    max_signal(&program, 0..=4, Amplifier::run)
}

fn max_feedback_thruster_signal(program: &Program) -> i64 {
    max_signal(&program, 5..=9, Amplifier::run_feedback)
}

fn day07() -> (i64, i64) {
    const DAY07_INPUT: &str = include_str!("day07_input.txt");
    let program = Program::from(DAY07_INPUT);
    (
        max_thruster_signal(&program),
        max_feedback_thruster_signal(&program),
    )
}

fn main() {
    let (part1, part2) = day07();
    println!("part1 = {}", part1);
    println!("part2 = {}", part2);
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_max_signal(program: &str, expected_amplitude: i64) {
        let program = Program::from(program);
        let signal = max_thruster_signal(&program);
        assert_eq!(signal, expected_amplitude);
    }

    #[test]
    fn test_max_thruster_signal() {
        check_max_signal("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", 43210);

        check_max_signal(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,\
             101,5,23,23,1,24,23,23,4,23,99,0,0",
            54321,
        );

        check_max_signal(
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,\
             1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
            65210,
        );
    }

    fn check_max_feedback_signal(program: &str, expected_amplitude: i64) {
        let program = Program::from(program);
        let signal = max_feedback_thruster_signal(&program);
        assert_eq!(signal, expected_amplitude);
    }

    #[test]
    fn test_max_feedback_thruster_signal() {
        check_max_feedback_signal(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\
             27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
            139_629_729,
        );

        check_max_feedback_signal(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,\
             -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,\
             53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
            18216,
        );
    }

    #[test]
    fn test_day07() {
        let (part1, part2) = day07();
        assert_eq!(part1, 46014);
        assert_eq!(part2, 19_581_200);
    }
}
