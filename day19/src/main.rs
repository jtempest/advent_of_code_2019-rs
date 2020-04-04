//! Solution to Advent of Code 2019 [Day 19](https://adventofcode.com/2019/day/19).

use aoc::intcode::{Machine, Program};

fn main() {
    println!("part1 = {}", day19_part1());
    println!("part2 = {}", day19_part2());
}

fn day19_part1() -> usize {
    let mut locator = TractorBeamLocator::default();
    (0..50)
        .flat_map(|x| (0..50).map(move |y| (x, y)))
        .filter(|&(x, y)| locator.has_beam(x, y))
        .count()
}

fn day19_part2() -> usize {
    const SIDE_LENGTH: usize = 100;

    // lines before y=4 have gaps in
    let mut locator = TractorBeamLocator::default();
    let mut row_start = 0;
    for y in 4.. {
        // find first location horizontally in the beam
        row_start = (row_start..).find(|&x| locator.has_beam(x, y)).unwrap();

        // search this row until we can't contain the square horizontally
        for x in row_start.. {
            if !locator.has_beam(x + SIDE_LENGTH - 1, y) {
                break;
            }
            if locator.has_beam(x, y + SIDE_LENGTH - 1) {
                return (x * 10_000) + y;
            }
        }
    }
    unreachable!();
}

#[derive(Debug)]
struct TractorBeamLocator {
    program: Program,
}

impl Default for TractorBeamLocator {
    fn default() -> Self {
        const DAY19_INPUT: &str = include_str!("day19_input.txt");
        TractorBeamLocator {
            program: Program::from(DAY19_INPUT),
        }
    }
}

impl TractorBeamLocator {
    fn has_beam(&mut self, x: usize, y: usize) -> bool {
        let mut machine = Machine::new(&self.program);
        machine.input(x as i64);
        machine.input(y as i64);
        machine.run().unwrap() == 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day19() {
        assert_eq!(day19_part1(), 181);
        assert_eq!(day19_part2(), 424_0964);
    }
}
