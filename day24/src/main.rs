//! Solution to Advent of Code 2019 [Day 24](https://adventofcode.com/2019/day/24).

use aoc::geom::{Dimensions, Vector2D};
use std::collections::HashSet;
use std::fmt;
use std::ops::Index;

const DAY24_INPUT: &str = include_str!("day24_input.txt");

fn main() {
    println!("part1 = {}", day24_part1());
    println!("part2 = {}", day24_part2());
}

fn day24_part1() -> usize {
    first_repeat_biodiversity(DAY24_INPUT)
}

fn day24_part2() -> u64 {
    repeat_recursive_n_times(DAY24_INPUT, 200).count_bugs()
}

fn first_repeat_biodiversity(input: &str) -> usize {
    let mut grid = Grid::from(input);
    let mut seen = HashSet::new();
    while seen.insert(grid.clone()) {
        grid = grid.next();
    }
    grid.biodiversity()
}

fn repeat_recursive_n_times(input: &str, n: usize) -> RecursiveGrid {
    let mut grid = RecursiveGrid::from(input);
    for _ in 0..n {
        grid = grid.next();
    }
    grid
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Location {
    Empty,
    Infested,
    AnotherGrid,
}

impl From<char> for Location {
    fn from(c: char) -> Location {
        match c {
            '.' => Location::Empty,
            '#' => Location::Infested,
            _ => panic!("Unknown location type {}", c),
        }
    }
}

impl Location {
    fn is_infested(self) -> bool {
        if let Location::Infested = self {
            true
        } else {
            false
        }
    }

    fn next(self, adjacent_bugs: usize) -> Location {
        match self {
            Location::Empty => {
                // An empty space becomes infested with a bug if exactly one or two bugs are adjacent to it.
                if adjacent_bugs == 1 || adjacent_bugs == 2 {
                    Location::Infested
                } else {
                    Location::Empty
                }
            }
            Location::Infested => {
                // A bug dies (becoming an empty space) unless there is exactly one bug adjacent to it.
                if adjacent_bugs == 1 {
                    Location::Infested
                } else {
                    Location::Empty
                }
            }
            Location::AnotherGrid => Location::AnotherGrid,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Location::Empty => write!(f, "."),
            Location::Infested => write!(f, "#"),
            Location::AnotherGrid => write!(f, "?"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid {
    locations: Vec<Location>,
    dimensions: Dimensions,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Grid {
        let locations: Vec<_> = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(Location::from)
            .collect();

        assert_eq!(locations.len(), 5 * 5);

        Grid {
            locations,
            dimensions: Dimensions {
                width: 5,
                height: 5,
            },
        }
    }
}

impl Grid {
    fn new_recursive(dimensions: Dimensions) -> Grid {
        let mut grid = Grid {
            locations: vec![Location::Empty; dimensions.area()],
            dimensions,
        };
        grid.make_recursive();
        grid
    }

    fn get(&self, pos: Vector2D) -> Option<&Location> {
        if !self.dimensions.contains(pos) {
            None
        } else {
            let (x, y) = (pos.x as usize, pos.y as usize);
            let index = (y * self.dimensions.width) + x;
            self.locations.get(index)
        }
    }

    fn next(&self) -> Grid {
        let locations = self
            .dimensions
            .iter()
            .zip(self.locations.iter())
            .map(|(pos, &loc)| loc.next(self.adjacent_bugs(pos)))
            .collect();

        Grid {
            locations,
            ..(*self)
        }
    }

    fn adjacent_bugs(&self, pos: Vector2D) -> usize {
        pos.neighbours()
            .filter_map(|n| self.get(n))
            .filter(|loc| loc.is_infested())
            .count()
    }

    fn next_recursive(&self, above: Option<&Grid>, below: Option<&Grid>) -> Grid {
        let locations = self
            .dimensions
            .iter()
            .zip(self.locations.iter())
            .map(|(pos, &loc)| loc.next(self.adjacent_bugs_recursive(pos, above, below)))
            .collect();

        Grid {
            locations,
            ..(*self)
        }
    }

    fn adjacent_bugs_recursive(
        &self,
        pos: Vector2D,
        above: Option<&Grid>,
        below: Option<&Grid>,
    ) -> usize {
        pos.neighbours()
            .map(|n| {
                if n == self.centre() {
                    if let Some(below) = below {
                        below.get_recursive_below(pos)
                    } else {
                        0
                    }
                } else {
                    let loc = self
                        .get(n)
                        .or_else(|| above?.get_recursive_above(n))
                        .unwrap_or(&Location::Empty);
                    if loc.is_infested() {
                        1
                    } else {
                        0
                    }
                }
            })
            .sum()
    }

    fn centre(&self) -> Vector2D {
        self.dimensions.centre()
    }

    fn get_recursive_above(&self, pos: Vector2D) -> Option<&Location> {
        let width = self.dimensions.width as i64;
        let height = self.dimensions.height as i64;
        let relative = if pos.x == -1 {
            Vector2D { x: -1, y: 0 }
        } else if pos.x == width {
            Vector2D { x: 1, y: 0 }
        } else if pos.y == -1 {
            Vector2D { x: 0, y: -1 }
        } else if pos.y == height {
            Vector2D { x: 0, y: 1 }
        } else {
            unreachable!();
        };
        self.get(self.centre() + relative)
    }

    fn get_recursive_below(&self, query_dir: Vector2D) -> usize {
        let relative = query_dir - self.centre();

        let w = self.dimensions.width as i64;
        let h = self.dimensions.height as i64;

        match relative {
            Vector2D { x: 0, y: -1 } => {
                // top
                let range = (0..w).map(|n| Vector2D { x: n, y: 0 });
                self.count_range(range)
            }
            Vector2D { x: 0, y: 1 } => {
                // bottom
                let range = (0..w).map(|n| Vector2D { x: n, y: h - 1 });
                self.count_range(range)
            }
            Vector2D { x: -1, y: 0 } => {
                // left
                let range = (0..h).map(|n| Vector2D { x: 0, y: n });
                self.count_range(range)
            }
            Vector2D { x: 1, y: 0 } => {
                // right
                let range = (0..h).map(|n| Vector2D { x: w - 1, y: n });
                self.count_range(range)
            }
            _ => unreachable!(),
        }
    }

    fn count_range(&self, range: impl Iterator<Item = Vector2D>) -> usize {
        range
            .filter_map(|pos| self.get(pos))
            .filter(|loc| loc.is_infested())
            .count()
    }

    fn biodiversity(&self) -> usize {
        self.locations
            .iter()
            .enumerate()
            .map(|(i, loc)| (2_usize.pow(i as u32), loc))
            .filter_map(|(i, loc)| if loc.is_infested() { Some(i) } else { None })
            .sum()
    }

    fn count_bugs(&self) -> u64 {
        self.locations
            .iter()
            .filter(|loc| loc.is_infested())
            .count() as u64
    }

    fn has_bugs_on_outside(&self) -> bool {
        let w = self.dimensions.width as i64;
        let h = self.dimensions.height as i64;
        (0..w).any(|n| self[Vector2D { x: n, y: 0 }].is_infested())
            || (0..w).any(|n| self[Vector2D { x: n, y: h - 1 }].is_infested())
            || (0..h).any(|n| self[Vector2D { x: 0, y: n }].is_infested())
            || (0..h).any(|n| self[Vector2D { x: w - 1, y: n }].is_infested())
    }

    fn has_bugs_on_inside(&self) -> bool {
        Vector2D {
            x: (self.dimensions.width / 2) as i64,
            y: (self.dimensions.height / 2) as i64,
        }
        .neighbours()
        .any(|n| self[n].is_infested())
    }

    fn make_recursive(&mut self) {
        let centre = self.centre();
        let index = self.dimensions.pos_to_node_index(centre);
        self.locations[index] = Location::AnotherGrid;
    }
}

impl Index<Vector2D> for Grid {
    type Output = Location;

    fn index(&self, pos: Vector2D) -> &Self::Output {
        &self.get(pos).expect("Out of bounds")
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for pos in self.dimensions.iter() {
            if pos.x == 0 && pos.y > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", self[pos])?;
        }
        Ok(())
    }
}

struct RecursiveGrid {
    grids: Vec<Grid>,
    depth: i64,
    dimensions: Dimensions,
}

impl From<&str> for RecursiveGrid {
    fn from(input: &str) -> RecursiveGrid {
        let mut grid = Grid::from(input);
        grid.make_recursive();
        let dimensions = grid.dimensions;
        RecursiveGrid {
            grids: vec![grid],
            dimensions,
            depth: 0,
        }
    }
}

impl RecursiveGrid {
    fn count_bugs(&self) -> u64 {
        self.grids.iter().map(Grid::count_bugs).sum()
    }

    fn next(&self) -> RecursiveGrid {
        let depth = if self.grids[0].has_bugs_on_outside()
            || self.grids[self.grids.len() - 1].has_bugs_on_inside()
        {
            self.depth + 1
        } else {
            self.depth
        };

        let grids = depths_iter(depth)
            .map(|d| {
                let above = self.get_grid(d - 1);
                let below = self.get_grid(d + 1);
                if let Some(grid) = self.get_grid(d) {
                    grid.next_recursive(above, below)
                } else {
                    Grid::new_recursive(self.dimensions).next_recursive(above, below)
                }
            })
            .collect();

        RecursiveGrid {
            grids,
            depth,
            ..(*self)
        }
    }

    fn get_grid(&self, depth: i64) -> Option<&Grid> {
        if depth.abs() > self.depth.abs() {
            None
        } else {
            let index = (self.grids.len() / 2) as i64 + depth;
            Some(&self.grids[index as usize])
        }
    }

    fn depths(&self) -> impl Iterator<Item = i64> {
        depths_iter(self.depth)
    }
}

fn depths_iter(depth: i64) -> impl Iterator<Item = i64> {
    let (min, max) = (-depth, depth);
    min..=max
}

impl fmt::Display for RecursiveGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (n, grid) in self.depths().zip(self.grids.iter()) {
            writeln!(f, "Depth {}", n)?;
            writeln!(f, "{}", grid)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn test_first_repeat_biodiversity() {
        assert_eq!(first_repeat_biodiversity(EXAMPLE), 2_129_920);
    }

    #[test]
    fn test_repeat_recursive_n_times() {
        let grid = repeat_recursive_n_times(EXAMPLE, 10);
        assert_eq!(grid.count_bugs(), 99);
    }

    #[test]
    fn test_day24() {
        assert_eq!(day24_part1(), 18_401_265);
        assert_eq!(day24_part2(), 2078);
    }
}
