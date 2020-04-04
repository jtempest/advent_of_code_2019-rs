//! Solution to Advent of Code 2019 [Day 14](https://adventofcode.com/2019/day/14).

use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;

fn main() {
    let part1 = day14_part1();
    println!("part1 = {}", part1);

    let part2 = day14_part2();
    println!("part2 = {}", part2);
}

fn day14_part1() -> u64 {
    minimum_ore_per_fuel(DAY14_INPUT)
}

fn day14_part2() -> u64 {
    max_fuel_per_trillion_ore(DAY14_INPUT)
}

fn minimum_ore_per_fuel(factory_spec: &'static str) -> u64 {
    let mut factory = NanoFactory::from(factory_spec);
    factory.make(ChemicalQuantity::from("1 FUEL"));
    factory.ore_used
}

fn max_fuel_per_trillion_ore(factory_spec: &'static str) -> u64 {
    let trillion = 1_000_000_000_000;
    let ore_for_one_fuel = minimum_ore_per_fuel(factory_spec);
    let mut factory = NanoFactory::from(factory_spec);
    let mut lower = trillion / ore_for_one_fuel;
    let mut upper = trillion;
    loop {
        let mid = (lower + upper) / 2;
        factory.make(ChemicalQuantity {
            name: "FUEL",
            quantity: mid,
        });
        if factory.ore_used > trillion {
            upper = mid;
        } else {
            lower = mid;
        }
        if (upper - lower) == 1 {
            break lower;
        }
        factory.reset();
    }
}

const DAY14_INPUT: &str = include_str!("day14_input.txt");

#[derive(Debug)]
struct NanoFactory {
    reactions: HashMap<&'static str, Reaction>,
    to_produce: Vec<ChemicalQuantity>,
    stock: HashMap<&'static str, u64>,
    ore_used: u64,
}

#[derive(Debug)]
struct Reaction {
    inputs: Vec<ChemicalQuantity>,
    output: ChemicalQuantity,
}

#[derive(Debug, Clone, Copy)]
struct ChemicalQuantity {
    name: &'static str,
    quantity: u64,
}

impl NanoFactory {
    fn new(reactions: HashMap<&'static str, Reaction>) -> NanoFactory {
        NanoFactory {
            reactions,
            to_produce: Vec::new(),
            stock: HashMap::new(),
            ore_used: 0,
        }
    }

    fn reset(&mut self) {
        self.to_produce.clear();
        self.stock.clear();
        self.ore_used = 0;
    }

    fn make(&mut self, chemical: ChemicalQuantity) {
        self.to_produce.push(chemical);
        while let Some(needed) = self.to_produce.pop() {
            self.produce(needed);
        }
    }

    fn produce(&mut self, chemical: ChemicalQuantity) {
        let used = self.use_existing_stock(&chemical);
        let quantity = chemical.quantity - used;
        if quantity > 0 {
            let produced = self.run_reaction(ChemicalQuantity {
                name: chemical.name,
                quantity,
            });
            if produced > quantity {
                self.stock.insert(chemical.name, produced - quantity);
            }
        }
    }

    fn use_existing_stock(&mut self, chemical: &ChemicalQuantity) -> u64 {
        if chemical.name == "ORE" {
            self.ore_used += chemical.quantity;
            chemical.quantity
        } else {
            let available = *self.stock.entry(&chemical.name).or_insert(0);
            let used = cmp::min(available, chemical.quantity);
            self.stock.insert(chemical.name, available - used);
            used
        }
    }

    fn run_reaction(&mut self, chemical: ChemicalQuantity) -> u64 {
        let reaction = &self.reactions[chemical.name];
        let per_run = reaction.output.quantity;
        let num_runs = (chemical.quantity as f64 / per_run as f64).ceil() as u64;
        for &input in reaction.inputs.iter() {
            let quantity = input.quantity * num_runs;
            let required = ChemicalQuantity { quantity, ..input };
            self.to_produce.push(required);
        }
        per_run * num_runs
    }
}

impl From<&'static str> for NanoFactory {
    fn from(string: &'static str) -> NanoFactory {
        let reactions = string
            .lines()
            .map(Reaction::from)
            .map(|r| (r.output.name, r))
            .collect();
        NanoFactory::new(reactions)
    }
}

impl From<&'static str> for Reaction {
    fn from(string: &'static str) -> Reaction {
        let (input, output) = string.trim().split("=>").next_tuple().unwrap();
        let inputs = input.split(',').map(ChemicalQuantity::from).collect_vec();
        let output = ChemicalQuantity::from(output);
        Reaction { inputs, output }
    }
}

impl From<&'static str> for ChemicalQuantity {
    fn from(string: &'static str) -> ChemicalQuantity {
        let (quantity, name) = string.split_whitespace().next_tuple().unwrap();
        let quantity = quantity.trim().parse::<u64>().unwrap();
        ChemicalQuantity { name, quantity }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const DAY14_EXAMPLES: [&str; 5] = [
        include_str!("day14_example0.txt"),
        include_str!("day14_example1.txt"),
        include_str!("day14_example2.txt"),
        include_str!("day14_example3.txt"),
        include_str!("day14_example4.txt"),
    ];

    #[test]
    fn test_make_fuel() {
        check_make_fuel(DAY14_EXAMPLES[0], 31);
        check_make_fuel(DAY14_EXAMPLES[1], 165);
        check_make_fuel(DAY14_EXAMPLES[2], 13_312);
        check_make_fuel(DAY14_EXAMPLES[3], 180_697);
        check_make_fuel(DAY14_EXAMPLES[4], 2_210_736);
    }

    fn check_make_fuel(factory_spec: &'static str, expected_ore: u64) {
        assert_eq!(minimum_ore_per_fuel(factory_spec), expected_ore);
    }

    #[test]
    fn test_max_fuel_per_trillion_ore() {
        check_max_fuel_per_trillion_ore(DAY14_EXAMPLES[2], 82_892_753);
        check_max_fuel_per_trillion_ore(DAY14_EXAMPLES[3], 5_586_022);
        check_max_fuel_per_trillion_ore(DAY14_EXAMPLES[4], 460_664);
    }

    fn check_max_fuel_per_trillion_ore(factory_spec: &'static str, expected_fuel: u64) {
        assert_eq!(max_fuel_per_trillion_ore(factory_spec), expected_fuel);
    }

    #[test]
    fn test_day14() {
        assert_eq!(day14_part1(), 1_920_219);
        assert_eq!(day14_part2(), 1_330_066);
    }
}
