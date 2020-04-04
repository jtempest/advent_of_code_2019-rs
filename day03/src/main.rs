//! Solution to Advent of Code 2019 [Day 3](https://adventofcode.com/2019/day/3).

use aoc::geom::Vector2D;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
enum PathDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct PathSegment {
    direction: PathDirection,
    length: usize,
}

impl PathSegment {
    fn new(input: &str) -> PathSegment {
        let (first, rest) = input.split_at(1);
        let direction = match first {
            "U" => PathDirection::Up,
            "D" => PathDirection::Down,
            "L" => PathDirection::Left,
            "R" => PathDirection::Right,
            _ => panic!("Unknown direction {}", first),
        };
        let length = rest.parse::<usize>().unwrap();
        assert!(length > 0);
        PathSegment { direction, length }
    }
}

#[derive(Clone)]
struct Path(Vec<PathSegment>);

impl Path {
    fn new(segment: &str) -> Path {
        let segments = segment.trim().split(',').map(PathSegment::new).collect();
        Path(segments)
    }

    fn walk(&self) -> PathWalker {
        PathWalker {
            position: Vector2D::zero(),
            path_iter: self.0.iter(),
            step: Vector2D::zero(),
            steps_left: 0,
        }
    }
}

struct PathWalker<'a> {
    position: Vector2D,
    path_iter: std::slice::Iter<'a, PathSegment>,
    step: Vector2D,
    steps_left: usize,
}

impl Iterator for PathWalker<'_> {
    type Item = Vector2D;

    fn next(&mut self) -> Option<Vector2D> {
        if self.steps_left == 0 {
            let segment = self.path_iter.next()?;
            self.step = match segment.direction {
                PathDirection::Up => Vector2D { x: 0, y: 1 },
                PathDirection::Down => Vector2D { x: 0, y: -1 },
                PathDirection::Left => Vector2D { x: 1, y: 0 },
                PathDirection::Right => Vector2D { x: -1, y: 0 },
            };
            self.steps_left = segment.length;
        }

        self.position += self.step;
        self.steps_left -= 1;
        Some(self.position)
    }
}

fn find_closest_intersection_distance(wire1: Path, wire2: Path) -> usize {
    find_intersections(wire1, wire2)
        .into_iter()
        .map(Vector2D::manhattan_length)
        .min()
        .unwrap()
}

fn find_intersections(wire1: Path, wire2: Path) -> HashSet<Vector2D> {
    let wire1_positions = wire1.walk().collect::<HashSet<_>>();
    wire2
        .walk()
        .filter(|p| wire1_positions.contains(p))
        .collect()
}

fn find_shortest_intersection_walk(wire1: Path, wire2: Path) -> usize {
    let mut wire1_positions = HashMap::new();
    for (n, p) in wire1.walk().enumerate() {
        let steps = n + 1;
        wire1_positions.entry(p).or_insert(steps);
    }

    let mut intersections = HashMap::new();
    for (n, p) in wire2.walk().enumerate() {
        if let Some(s1) = wire1_positions.get(&p) {
            let steps = n + 1;
            intersections.entry(p).or_insert(steps + s1);
        }
    }

    intersections.values().copied().min().unwrap()
}

static DAY03_INPUT: Lazy<(Path, Path)> = Lazy::new(|| {
    let input = include_str!("day03_input.txt");
    let mut lines = input.trim().lines();
    let p1 = Path::new(lines.next().unwrap());
    let p2 = Path::new(lines.next().unwrap());
    (p1, p2)
});

fn day03_part1() -> usize {
    let (p1, p2) = DAY03_INPUT.clone();
    find_closest_intersection_distance(p1, p2)
}

fn day03_part2() -> usize {
    let (p1, p2) = DAY03_INPUT.clone();
    find_shortest_intersection_walk(p1, p2)
}

fn main() {
    println!("part1 = {}", day03_part1());
    println!("part2 = {}", day03_part2());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_closest_intersection_distance_examples() {
        let check = |wire1, wire2, expected_distance| {
            let p1 = Path::new(wire1);
            let p2 = Path::new(wire2);
            assert_eq!(
                find_closest_intersection_distance(p1, p2),
                expected_distance
            );
        };

        check("R8,U5,L5,D3", "U7,R6,D4,L4", 6);
        check(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
            159,
        );
        check(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            135,
        );
    }

    #[test]
    fn find_shortest_intersection_walk_examples() {
        let check = |wire1, wire2, expected_distance| {
            let p1 = Path::new(wire1);
            let p2 = Path::new(wire2);
            assert_eq!(find_shortest_intersection_walk(p1, p2), expected_distance);
        };

        check("R8,U5,L5,D3", "U7,R6,D4,L4", 30);
        check(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
            610,
        );
        check(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            410,
        );
    }

    #[test]
    fn test_day03() {
        assert_eq!(day03_part1(), 731);
        assert_eq!(day03_part2(), 5672);
    }
}
