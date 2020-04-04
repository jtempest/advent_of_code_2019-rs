//! Solution to Advent of Code 2019 [Day 18](https://adventofcode.com/2019/day/18).

mod key;
mod key_map;
mod key_set;
mod tunnel_map;
mod tunnel_tile;

use key_map::KeyMap;
use std::convert::TryFrom;

const DAY18_INPUT: &str = include_str!("input/day18_input.txt");

fn main() {
    println!("part1 = {}", day18_part1());
    println!("part1 = {}", day18_part2());
}

fn day18_part1() -> usize {
    find_quickest_route(DAY18_INPUT).unwrap()
}

fn day18_part2() -> usize {
    find_quickest_route_in_quadrants(DAY18_INPUT).unwrap()
}

fn find_quickest_route(input: &str) -> Result<usize, String> {
    KeyMap::try_from(input)?
        .find_quickest_path_to_all_keys()
        .ok_or_else(|| "Failed to find a route".into())
}

fn find_quickest_route_in_quadrants(input: &str) -> Result<usize, String> {
    KeyMap::make_quadrants(input)?
        .find_quickest_path_to_all_keys()
        .ok_or_else(|| "Failed to find a route".into())
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1: &str = include_str!("input/example1.txt");
    const EXAMPLE2: &str = include_str!("input/example2.txt");
    const EXAMPLE3: &str = include_str!("input/example3.txt");
    const EXAMPLE4: &str = include_str!("input/example4.txt");
    const EXAMPLE5: &str = include_str!("input/example5.txt");

    #[test]
    fn test_quickest_route() {
        check_quickest_route(EXAMPLE1, 8);
        check_quickest_route(EXAMPLE2, 86);
        check_quickest_route(EXAMPLE3, 132);
        check_quickest_route(EXAMPLE4, 136);
        check_quickest_route(EXAMPLE5, 81);
    }

    fn check_quickest_route(input: &str, expected_steps: usize) {
        assert_eq!(find_quickest_route(input), Ok(expected_steps));
    }

    const QUADRANT_EXAMPLE1: &str = include_str!("input/quadrant_example1.txt");
    const QUADRANT_EXAMPLE2: &str = include_str!("input/quadrant_example2.txt");
    const QUADRANT_EXAMPLE3: &str = include_str!("input/quadrant_example3.txt");
    const QUADRANT_EXAMPLE4: &str = include_str!("input/quadrant_example4.txt");

    #[test]
    fn test_quckest_route_in_quadrants() {
        check_quickest_route_in_quadrants(QUADRANT_EXAMPLE1, 8);
        check_quickest_route_in_quadrants(QUADRANT_EXAMPLE2, 24);
        check_quickest_route_in_quadrants(QUADRANT_EXAMPLE3, 32);
        check_quickest_route_in_quadrants(QUADRANT_EXAMPLE4, 72);
    }

    fn check_quickest_route_in_quadrants(input: &str, expected_steps: usize) {
        assert_eq!(find_quickest_route_in_quadrants(input), Ok(expected_steps));
    }

    #[test]
    fn test_day18() {
        assert_eq!(day18_part1(), 3862);
        assert_eq!(day18_part2(), 1626);
    }
}
