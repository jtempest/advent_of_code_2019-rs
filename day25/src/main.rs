//! Solution to Advent of Code 2019 [Day 25](https://adventofcode.com/2019/day/25).

use aoc::intcode::Machine;
use itertools::Itertools;
use regex::Regex;
use std::env;
use std::io;

fn main() {
    let args = env::args().collect_vec();
    if args.len() >= 2 && args[1] == "--interactive" {
        let mut droid = Droid::new();
        droid.interactive_loop();
    } else {
        println!("{}", day25_part1())
    }
}

fn day25_part1() -> u64 {
    let mut droid = Droid::new();
    droid.pick_up_items();
    let output = droid.find_correctly_weighted_items().unwrap();

    let re = Regex::new(r"\d+").unwrap();
    let caps = re.captures(&output).unwrap();
    let password = caps.get(0).unwrap();
    password.as_str().parse::<u64>().unwrap()
}

struct Droid {
    machine: Machine,
}

impl Droid {
    fn new() -> Droid {
        const DAY25_INPUT: &str = include_str!("day25_input.txt");
        Droid {
            machine: Machine::from_source(DAY25_INPUT),
        }
    }

    fn run_one_command(&mut self, input: &str) -> String {
        self.machine.input_ascii(input.trim());
        self.machine.run_as_ascii()
    }

    fn run_commands(&mut self, commands: &str) -> Option<String> {
        let mut out = None;
        for line in commands.trim().lines() {
            out = Some(self.run_one_command(line));
        }
        out
    }

    fn pick_up_items(&mut self) {
        const _PROGRAM: &str = include_str!("pick_up_all_items.txt");
        self.run_commands(_PROGRAM);
    }

    fn find_correctly_weighted_items(&mut self) -> Option<String> {
        let all_items = vec![
            "asterisk",
            "ornament",
            "cake",
            "space heater",
            "festive hat",
            "semiconductor",
            "food ration",
            "sand",
        ];

        for item in &all_items {
            self.run_one_command(&format!("drop {}", item));
        }

        for n in 1..all_items.len() {
            for items in all_items.iter().combinations(n) {
                for item in &items {
                    self.run_one_command(&format!("take {}", item));
                }

                let output = self.run_one_command("west");
                if !(output.contains("lighter") || output.contains("heavier")) {
                    return Some(output);
                }

                for item in items.into_iter() {
                    self.run_one_command(&format!("drop {}", item));
                }
            }
        }

        None
    }

    fn interactive_loop(&mut self) {
        loop {
            print!("{}", self.machine.run_as_ascii());
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();

            if buffer.starts_with("exit") {
                break;
            }
            self.machine.input_ascii(buffer.trim());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day25() {
        assert_eq!(day25_part1(), 25_165_890);
    }
}
