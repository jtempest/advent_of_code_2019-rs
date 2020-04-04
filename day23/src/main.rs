//! Solution to Advent of Code 2019 [Day 23](https://adventofcode.com/2019/day/23).

use aoc::intcode::{Machine, Program};
use itertools::Itertools;
use std::collections::VecDeque;

const DAY23_INPUT: &str = include_str!("day23_input.txt");

fn main() {
    println!("part1 = {}", day23_part1());
    println!("part2 = {}", day23_part2());
}

fn day23_part1() -> i64 {
    run_network(NetworkMode::Part1)
}

fn day23_part2() -> i64 {
    run_network(NetworkMode::Part2)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum NetworkMode {
    Part1,
    Part2,
}

fn run_network(mode: NetworkMode) -> i64 {
    let num_machines = 50;

    let program = Program::from(DAY23_INPUT);
    let mut machines: Vec<_> = (0..num_machines)
        .map(|i| NetworkComputer::new(&program, i))
        .collect();
    let mut queue = VecDeque::new();
    let mut nat = None;
    let mut last_delivered_nat: Option<Packet> = None;

    loop {
        // empty queue => send Nones until messages are added
        if queue.is_empty() {
            for m in &mut machines {
                let msgs = m.run(None);
                if !msgs.is_empty() {
                    queue.extend(msgs);
                    break;
                }
            }
        }

        // run the queue until dry
        while let Some(msg) = queue.pop_back() {
            let address = msg.address as usize;
            if address == 255 {
                match mode {
                    NetworkMode::Part1 => return msg.y,
                    NetworkMode::Part2 => nat = Some(msg),
                }
            } else {
                let m = &mut machines[address];
                queue.extend(m.run(Some(msg)));
            }
        }

        // idle network?
        if let NetworkMode::Part2 = mode {
            if machines.iter().all(|m| m.is_idle()) {
                if let Some(msg) = nat {
                    println!("send: {:?}", msg);
                    if let Some(last) = last_delivered_nat {
                        if last.y == msg.y {
                            return msg.y;
                        }
                    }
                    queue.extend(machines[0].run(Some(msg)));
                    last_delivered_nat = nat.take();
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Packet {
    address: i64,
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct NetworkComputer {
    machine: Machine,
}

impl NetworkComputer {
    fn new(program: &Program, address: i64) -> NetworkComputer {
        NetworkComputer {
            machine: Machine::with_input(&program, address),
        }
    }

    fn is_idle(&self) -> bool {
        self.machine.is_awaiting_input()
    }

    fn run(&mut self, packet: Option<Packet>) -> Vec<Packet> {
        assert!(self.is_idle());

        match packet {
            None => self.machine.input(-1),
            Some(p) => {
                self.machine.input(p.x);
                self.machine.input(p.y);
            }
        }

        self.machine
            .run_as_iter()
            .batching(|it| {
                Some(Packet {
                    address: it.next()?,
                    x: it.next().unwrap(),
                    y: it.next().unwrap(),
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day23() {
        assert_eq!(day23_part1(), 24602);
        assert_eq!(day23_part2(), 19641);
    }
}
