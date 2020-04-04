//! Solution to Advent of Code 2019 [Day 20](https://adventofcode.com/2019/day/20).

use aoc::geom::{self, Dimensions, Vector2D};
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

const DAY20_INPUT: &str = include_str!("input/day20_input.txt");

fn main() {
    println!("part1 = {}", day20_part1());
    println!("part2 = {}", day20_part2());
}

fn day20_part1() -> usize {
    Map::from(DAY20_INPUT).find_shortest_route()
}

fn day20_part2() -> usize {
    Map::from(DAY20_INPUT).find_shortest_route_recursive()
}

#[derive(Debug)]
struct Map {
    start: Vector2D,
    end: Vector2D,
    tiles: HashSet<Vector2D>,
    outer_portals: HashMap<Vector2D, Vector2D>,
    inner_portals: HashMap<Vector2D, Vector2D>,
}

impl Map {
    fn find_shortest_route(&self) -> usize {
        let mut open = BinaryHeap::new();
        open.push(Reverse((0, self.start)));

        let mut seen = HashSet::new();

        loop {
            let Reverse((distance, pos)) = open.pop().unwrap();
            if pos == self.end {
                break distance;
            }

            if !seen.insert(pos) {
                continue;
            }

            if let Some(&endpoint) = self.inner_portals.get(&pos) {
                open.push(Reverse((distance + 1, endpoint)));
            }

            if let Some(&endpoint) = self.outer_portals.get(&pos) {
                open.push(Reverse((distance + 1, endpoint)));
            }

            open.extend(
                pos.neighbours()
                    .filter(|n| self.tiles.contains(&n))
                    .map(|n| Reverse((distance + 1, n))),
            );
        }
    }

    fn find_shortest_route_recursive(&self) -> usize {
        let mut open = BinaryHeap::new();
        open.push(Reverse((0, 0, self.start)));

        let mut seen = HashSet::new();

        loop {
            let Reverse((distance, level, pos)) = open.pop().unwrap();
            if pos == self.end && level == 0 {
                break distance;
            }

            if !seen.insert((pos, level)) {
                continue;
            }

            if let Some(&endpoint) = self.inner_portals.get(&pos) {
                open.push(Reverse((distance + 1, level + 1, endpoint)));
            }

            if level > 0 {
                if let Some(&endpoint) = self.outer_portals.get(&pos) {
                    open.push(Reverse((distance + 1, level - 1, endpoint)));
                }
            }

            open.extend(
                pos.neighbours()
                    .filter(|n| self.tiles.contains(&n))
                    .map(|n| Reverse((distance + 1, level, n))),
            );
        }
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Map {
        let (tiles, portal_tiles, centre) = read_tiles(input);
        let portal_halves = build_portal_endpoints(&tiles, portal_tiles, centre);
        let (start, end, portals) = connect_portals(portal_halves);

        let outer_portals = portals.iter().copied().map(|(a, b)| (b, a)).collect();
        let inner_portals = portals.into_iter().collect();

        Map {
            start,
            end,
            tiles,
            inner_portals,
            outer_portals,
        }
    }
}

fn read_tiles(input: &str) -> (HashSet<Vector2D>, HashMap<Vector2D, char>, Vector2D) {
    let mut tiles = HashSet::new();
    let mut portal_tiles = HashMap::new();
    let mut dimensions = Dimensions::new();
    for (pos, c) in geom::cartograph(input) {
        if c == '.' {
            tiles.insert(pos);
        } else if c.is_alphabetic() {
            portal_tiles.insert(pos, c);
        }
        dimensions.expand_to_fit(pos);
    }

    let centre = Vector2D {
        x: (dimensions.width / 2) as i64,
        y: (dimensions.height / 2) as i64,
    };

    (tiles, portal_tiles, centre)
}

#[derive(Debug, Eq, PartialEq)]
enum PortalType {
    Inner,
    Outer,
}

struct PortalHalf {
    letters: (char, char),
    entry_point: Vector2D,
    portal_type: PortalType,
}

fn build_portal_endpoints(
    tiles: &HashSet<Vector2D>,
    portal_tiles: HashMap<Vector2D, char>,
    centre: Vector2D,
) -> Vec<PortalHalf> {
    let mut portals: Vec<_> = portal_tiles
        .iter()
        .filter_map(|(&pos1, &c1)| {
            let (&pos2, &c2) = pos1
                .neighbours()
                .find_map(|n| (portal_tiles.get_key_value(&n)))?;

            let &entry_point = pos1.neighbours().find_map(|n| tiles.get(&n))?;

            let mut letters = [c1, c2];
            letters.sort();
            let letters = (letters[0], letters[1]);

            let c1dist = (centre - pos1).manhattan_length();
            let c2dist = (centre - pos2).manhattan_length();
            let portal_type = if c1dist < c2dist {
                PortalType::Outer
            } else {
                PortalType::Inner
            };

            Some(PortalHalf {
                letters,
                entry_point,
                portal_type,
            })
        })
        .collect();

    portals.sort_by(|a, b| a.letters.cmp(&b.letters));

    portals
}

fn connect_portals(
    mut portal_halves: Vec<PortalHalf>,
) -> (Vector2D, Vector2D, Vec<(Vector2D, Vector2D)>) {
    let end = portal_halves.pop().unwrap().entry_point;

    let mut iter = portal_halves.into_iter();
    let start = iter.next().unwrap().entry_point;
    let portals: Vec<(Vector2D, Vector2D)> = iter
        .tuples()
        .map(|(a, b)| {
            let (pos1, pos2) = (a.entry_point, b.entry_point);
            if a.portal_type == PortalType::Inner {
                (pos1, pos2)
            } else {
                (pos2, pos1)
            }
        })
        .collect();

    (start, end, portals)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1: &str = include_str!("input/example1.txt");
    const EXAMPLE2: &str = include_str!("input/example2.txt");
    const EXAMPLE3: &str = include_str!("input/example3.txt");

    #[test]
    fn test_find_shortest_route() {
        assert_eq!(Map::from(EXAMPLE1).find_shortest_route(), 23);
        assert_eq!(Map::from(EXAMPLE2).find_shortest_route(), 58);
    }

    #[test]
    fn test_find_shortest_route_recursive() {
        assert_eq!(Map::from(EXAMPLE1).find_shortest_route_recursive(), 26);
        assert_eq!(Map::from(EXAMPLE3).find_shortest_route_recursive(), 396);
    }

    #[test]
    fn test_day20() {
        assert_eq!(day20_part1(), 522);
        assert_eq!(day20_part2(), 6300);
    }
}
