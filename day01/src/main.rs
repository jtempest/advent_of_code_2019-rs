//! Solution to Advent of Code 2019 [Day 1](https://adventofcode.com/2019/day/1).

use once_cell::sync::Lazy;

static DAY01_INPUT: Lazy<Vec<i32>> = Lazy::new(|| {
    let input = include_str!("day01_input.txt");
    input.lines().map(|s| s.parse::<i32>().unwrap()).collect()
});

fn fuel_required(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn day01_part1() -> i32 {
    DAY01_INPUT.iter().copied().map(fuel_required).sum()
}

fn total_fuel_required(mass: i32) -> i32 {
    let fuel_mass = fuel_required(mass);
    if fuel_mass > 0 {
        fuel_mass + total_fuel_required(fuel_mass)
    } else {
        0
    }
}

fn day01_part2() -> i32 {
    DAY01_INPUT.iter().copied().map(total_fuel_required).sum()
}

fn main() {
    println!("part1 = {}", day01_part1());
    println!("part2 = {}", day01_part2());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fuel_required() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100_756), 33583);
    }

    #[test]
    fn test_total_fuel_required() {
        assert_eq!(total_fuel_required(14), 2);
        assert_eq!(total_fuel_required(1969), 966);
        assert_eq!(total_fuel_required(100_756), 50346);
    }

    #[test]
    fn test_day01() {
        assert_eq!(day01_part1(), 3_325_342);
        assert_eq!(day01_part2(), 4_985_158);
    }
}
