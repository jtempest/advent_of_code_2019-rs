//! Solution to Advent of Code 2019 [Day 11](https://adventofcode.com/2019/day/11).

use aoc::geom::Vector2D;
use aoc::intcode::{Machine, Program};
use aoc::ocr::{ocr, LetterImage, LETTER_IMAGE_DIMENSIONS};
use std::collections::HashMap;
use std::iter;

#[derive(Debug, Clone, Copy)]
enum TurnDirection {
    TurnLeft,
    TurnRight,
}

impl From<i64> for TurnDirection {
    fn from(value: i64) -> TurnDirection {
        match value {
            0 => TurnDirection::TurnLeft,
            1 => TurnDirection::TurnRight,
            _ => panic!("Unknown TurnDirection '{}'", value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn as_vector2d(self) -> Vector2D {
        match self {
            Direction::Up => Vector2D { x: 0, y: 1 },
            Direction::Down => Vector2D { x: 0, y: -1 },
            Direction::Right => Vector2D { x: 1, y: 0 },
            Direction::Left => Vector2D { x: -1, y: 0 },
        }
    }

    fn turn(self, turn_dir: TurnDirection) -> Direction {
        match turn_dir {
            TurnDirection::TurnLeft => match self {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            },
            TurnDirection::TurnRight => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
        }
    }
}

struct HullPaintingRobot {
    machine: Machine,
    position: Vector2D,
    direction: Direction,
    panels: HashMap<Vector2D, i64>,
}

impl HullPaintingRobot {
    fn new(program: &Program) -> HullPaintingRobot {
        HullPaintingRobot {
            machine: Machine::new(&program),
            position: Vector2D::zero(),
            direction: Direction::Up,
            panels: HashMap::new(),
        }
    }

    fn run_to_completion(&mut self, initial_colour: i64) {
        self.machine.input(initial_colour);
        loop {
            let paint_colour = self.machine.run();
            if paint_colour.is_none() {
                assert!(self.machine.is_halted());
                break;
            }
            self.panels.insert(self.position, paint_colour.unwrap());

            let turn_dir = self.machine.run().unwrap();
            let turn_dir = TurnDirection::from(turn_dir);
            self.direction = self.direction.turn(turn_dir);
            self.position += self.direction.as_vector2d();

            let colour = self.panels.entry(self.position).or_insert(0);
            self.machine.input(*colour);
        }
    }

    fn panels(&self) -> &HashMap<Vector2D, i64> {
        &self.panels
    }

    fn render_panels(&self) -> String {
        let panels = &self.panels;

        let left = panels.keys().map(|p| p.x).min().unwrap();
        let right = panels.keys().map(|p| p.x).max().unwrap();

        let bottom = panels.keys().map(|p| p.y).min().unwrap();
        let top = panels.keys().map(|p| p.y).max().unwrap();

        let mut canvas = String::new();
        for y in (bottom..=top).rev() {
            for x in left..=right {
                let colour = panels.get(&Vector2D { x, y });
                let colour = match colour {
                    Some(&value) => value,
                    None => 0,
                };
                let c = if colour == 1 { '@' } else { ' ' };
                canvas.push(c);
            }
            canvas.push('\n');
        }

        canvas
    }
}

fn day11() -> (usize, String) {
    const DAY11_INPUT: &str = include_str!("day11_input.txt");
    let program = Program::from(DAY11_INPUT);
    let part1 = day11_part1(&program);
    let part2 = day11_part2(&program);
    (part1, part2)
}

fn day11_part1(program: &Program) -> usize {
    let mut robot = HullPaintingRobot::new(&program);
    robot.run_to_completion(0);
    robot.panels().len()
}

fn day11_part2(program: &Program) -> String {
    let mut robot = HullPaintingRobot::new(&program);
    robot.run_to_completion(1);

    let rendered = robot.render_panels();
    let width = rendered.find('\n').unwrap();

    // Image begins at index 1 from inspection of output
    let letter_width = LETTER_IMAGE_DIMENSIONS.width;
    iter::successors(Some(1), |x| Some(x + letter_width + 1))
        .take_while(|x| ((*x) + LETTER_IMAGE_DIMENSIONS.width) < width)
        .map(|x| {
            rendered
                .lines()
                .map(move |line| line[x..(x + letter_width)].chars())
                .flatten()
                .map(|c| c != ' ')
                .collect()
        })
        .map(LetterImage)
        .map(ocr)
        .map(|result| result.character)
        .collect()
}

#[test]
fn test_day11() {
    let (part1, part2) = day11();
    assert_eq!(part1, 1883);
    assert_eq!(part2, "APUGURFH");
}

fn main() {
    let (part1, part2) = day11();
    println!("part1 = {}", part1);
    println!("part2 = {}", part2);
}
