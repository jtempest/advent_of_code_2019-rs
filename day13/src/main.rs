//! Solution to Advent of Code 2019 [Day 13](https://adventofcode.com/2019/day/13).

use aoc::geom::Dimensions;
use aoc::intcode::Machine;
use itertools::Itertools;
use std::cmp;
use std::fmt;
use std::ops::{Index, IndexMut};

fn main() {
    println!("part1 = {}", day13_part1());
    println!("part2 = {}", day13_part2());
}

fn day13_part1() -> usize {
    let mut cabinet = ArcadeCabinet::new();
    cabinet.run();
    cabinet
        .render()
        .chars()
        .filter(|&c| c == char::from(Tile::Block))
        .count()
}

fn day13_part2() -> i64 {
    let mut cabinet = ArcadeCabinet::new();
    cabinet.play();
    cabinet.score()
}

const DAY13_INPUT: &str = include_str!("day13_input.txt");

#[derive(Debug)]
struct ArcadeCabinet {
    machine: Machine,
    screen: Screen,
    score: i64,
    ball_pos: i64,
    paddle_pos: i64,
}

impl ArcadeCabinet {
    fn new() -> ArcadeCabinet {
        ArcadeCabinet {
            machine: Machine::from_source(DAY13_INPUT),
            screen: Screen::new(),
            score: 0,
            ball_pos: 0,
            paddle_pos: 0,
        }
    }

    fn run(&mut self) {
        while let Some((x, y, value)) = self.machine.run_as_iter().next_tuple() {
            match (x, y) {
                (-1, 0) => self.score = value,
                _ => {
                    // update canvas
                    let tile = Tile::from(value);
                    let pos = (x as usize, y as usize);
                    self.screen[pos] = tile;

                    // update ball and paddle locations
                    if let Tile::Ball = tile {
                        self.ball_pos = x;
                    } else if let Tile::Paddle = tile {
                        self.paddle_pos = x;
                    }
                }
            }
        }
    }

    fn play(&mut self) {
        self.machine.write(0, 2);
        loop {
            self.run();

            if self.machine.is_awaiting_input() {
                let diff = self.ball_pos - self.paddle_pos;
                let joystick = num::clamp(diff, -1, 1);
                self.machine.input(joystick);
            } else {
                assert!(self.machine.is_halted());
                break;
            }
        }
    }

    fn render(&self) -> String {
        format!("{}", self.screen)
    }

    fn score(&self) -> i64 {
        self.score
    }
}

type ScreenPosition = (usize, usize);

#[derive(Debug)]
struct Screen {
    dimensions: Dimensions,
    canvas: Vec<Vec<Tile>>,
}

impl Screen {
    fn new() -> Screen {
        Screen {
            canvas: Vec::new(),
            dimensions: Dimensions {
                width: 0,
                height: 0,
            },
        }
    }
}

impl Index<ScreenPosition> for Screen {
    type Output = Tile;

    fn index(&self, pos: ScreenPosition) -> &Tile {
        &self.canvas[pos.1][pos.0]
    }
}

impl IndexMut<ScreenPosition> for Screen {
    fn index_mut(&mut self, pos: ScreenPosition) -> &mut Tile {
        let width = cmp::max(self.dimensions.width, pos.0 + 1);
        let height = cmp::max(self.dimensions.height, pos.1 + 1);

        self.canvas.resize_with(height, Vec::default);
        for line in self.canvas.iter_mut() {
            line.resize(width, Tile::Empty);
        }

        self.dimensions = Dimensions { width, height };
        &mut self.canvas[pos.1][pos.0]
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.canvas.iter() {
            for tile in line.iter() {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl From<i64> for Tile {
    fn from(value: i64) -> Tile {
        match value {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("Unknown tile value '{}'", value),
        }
    }
}

impl From<Tile> for char {
    fn from(tile: Tile) -> char {
        match tile {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::Block => '=',
            Tile::Paddle => '_',
            Tile::Ball => 'o',
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day13() {
        assert_eq!(day13_part1(), 173);
        assert_eq!(day13_part2(), 8942);
    }
}
