//! Solution to Advent of Code 2019 [Day 6](https://adventofcode.com/2019/day/6).

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct OrbitMap<'a> {
    objects: Vec<&'a str>,
    orbits: HashMap<&'a str, &'a str>,
}

impl<'a> OrbitMap<'a> {
    fn new(map: &str) -> OrbitMap {
        let mut objects = Vec::new();
        let orbits = map
            .lines()
            .map(|s| {
                let mid = s.find(')').unwrap();
                let primary = &s[..mid];
                let satellite = &s[(mid + 1)..];
                assert!(!objects.contains(&satellite));
                objects.push(satellite);
                (satellite, primary)
            })
            .collect::<HashMap<_, _>>();
        OrbitMap { objects, orbits }
    }

    fn find_primary(&self, satellite: &str) -> Option<&'a str> {
        self.orbits.get(satellite).copied()
    }

    fn walk_orbits(&'a self, object: &'a str) -> WalkOrbits {
        WalkOrbits::new(self, object)
    }

    fn total_orbits(&self) -> usize {
        self.objects
            .iter()
            .map(|o| self.walk_orbits(o).count())
            .sum()
    }

    fn find_num_transits(&self, object_a: &str, object_b: &str) -> usize {
        let path_a = self.walk_orbits(object_a).collect::<HashSet<_>>();
        let (distance_b, common) = self
            .walk_orbits(object_b)
            .enumerate()
            .find(|(_, o)| path_a.contains(o))
            .unwrap();
        // +1 because we lose an orbit count walking from the common point
        let distance_common = self.walk_orbits(common).count() + 1;
        let distance_a = path_a.len() - distance_common;
        distance_a + distance_b
    }
}

struct WalkOrbits<'a> {
    map: &'a OrbitMap<'a>,
    object: Option<&'a str>,
}

impl<'a> WalkOrbits<'a> {
    fn new(map: &'a OrbitMap, object: &'a str) -> WalkOrbits<'a> {
        let object = Some(object);
        WalkOrbits { map, object }
    }
}

impl<'a> Iterator for WalkOrbits<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        self.object = self.map.find_primary(self.object?);
        self.object
    }
}

const DAY06_INPUT: &str = include_str!("day06_input.txt");

fn day06_part1() -> usize {
    OrbitMap::new(DAY06_INPUT).total_orbits()
}

fn day06_part2() -> usize {
    OrbitMap::new(DAY06_INPUT).find_num_transits("YOU", "SAN")
}

fn main() {
    println!("part1 = {}", day06_part1());
    println!("part2 = {}", day06_part2());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        const DAY06_EXAMPLE: &str = include_str!("day06_example.txt");
        let map = OrbitMap::new(DAY06_EXAMPLE);
        assert_eq!(map.total_orbits(), 42);

        const DAY06_EXAMPLE_TRANSIT: &str = include_str!("day06_example_transit.txt");
        let transit_map = OrbitMap::new(DAY06_EXAMPLE_TRANSIT);
        assert_eq!(transit_map.find_num_transits("YOU", "SAN"), 4);
    }

    #[test]
    fn test_day06() {
        assert_eq!(day06_part1(), 315_757);
        assert_eq!(day06_part2(), 481);
    }
}
