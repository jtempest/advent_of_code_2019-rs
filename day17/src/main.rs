//! Solution to Advent of Code 2019 [Day 17](https://adventofcode.com/2019/day/17).

use aoc::geom::Vector2D;
use aoc::intcode::Machine;
use std::collections::{HashMap, HashSet};

fn main() {
    let part1 = day17_part1();
    println!("part1 = {}", part1);

    let part2 = day17_part2();
    println!("part2 = {}", part2);
}

fn day17_part1() -> i64 {
    let mut m = Machine::from_source(DAY17_INPUT);
    let output = m.run_as_ascii();
    let ascii = ASCIIOutput::new(&output);
    let intersections = ascii.find_intersections();
    intersections.iter().map(|p| p.x * p.y).sum()
}

fn day17_part2() -> i64 {
    // These functions were produced by inspection, but I expect that the way
    // to produce them programmtically would be to:
    //
    // - Produce a single long route by traversing the scaffolds travelling as
    //   far as possible each step.
    //
    // - Starting from the end, find the longest sequence which is repeated
    //   elsewhere in the route and replace those instructions with the function
    //   name. Repeat until you have three functions, assuming that they cover
    //   the entire sequence.

    const MAIN_SEQUENCE: &str = "A,B,A,B,C,C,B,C,B,A";
    const FUNCTIONS: [&str; 3] = ["R,12,L,8,R,12", "R,8,R,6,R,6,R,8", "R,8,L,8,R,8,R,4,R,4"];

    let mut machine = Machine::from_source(DAY17_INPUT);
    machine.write(0, 2);

    input_sequence(&mut machine, MAIN_SEQUENCE);
    for f in &FUNCTIONS {
        input_sequence(&mut machine, f);
    }
    input_sequence(&mut machine, "n");

    machine.run_as_iter().last().unwrap()
}

fn input_sequence(machine: &mut Machine, seq: &str) {
    let _prompt = machine.run_as_ascii();
    machine.input_ascii(seq);
}

const DAY17_INPUT: &str = include_str!("day17_input.txt");

#[derive(Debug)]
struct ASCIIOutput {
    image: HashMap<Vector2D, TileType>,
}

impl ASCIIOutput {
    fn new(raw_image: &str) -> ASCIIOutput {
        let image = ASCIIOutput::interpret_ascii_image(raw_image);
        ASCIIOutput { image }
    }

    fn interpret_ascii_image(raw_image: &str) -> HashMap<Vector2D, TileType> {
        let mut image = HashMap::new();
        let mut pos = Vector2D::zero();
        for c in raw_image.chars() {
            if c == '\n' {
                pos.y += 1;
                pos.x = 0;
            } else {
                image.insert(pos, TileType::from(c));
                pos.x += 1;
            }
        }
        image
    }

    fn find_intersections(&self) -> HashSet<Vector2D> {
        self.image
            .keys()
            .filter(|&&k| self.is_scaffold(k))
            .filter(|pos| pos.neighbours().all(|n| self.is_scaffold(n)))
            .copied()
            .collect()
    }

    fn is_scaffold(&self, pos: Vector2D) -> bool {
        let &tt = self.image.get(&pos).unwrap_or(&TileType::Space);
        tt == TileType::Scaffold
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TileType {
    Space,
    Scaffold,
    RobotLeft,
    RobotRight,
    RobotUp,
    RobotDown,
}

impl From<char> for TileType {
    fn from(c: char) -> TileType {
        match c {
            '.' => TileType::Space,
            '#' => TileType::Scaffold,
            '<' => TileType::RobotLeft,
            '>' => TileType::RobotRight,
            '^' => TileType::RobotUp,
            'v' => TileType::RobotDown,
            _ => panic!("Unknown TileType '{}'", c),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day17() {
        let part1 = day17_part1();
        assert_eq!(part1, 14332);

        let part2 = day17_part2();
        assert_eq!(part2, 1_034_009);
    }
}
