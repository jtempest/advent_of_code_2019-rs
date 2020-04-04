//! Solution to Advent of Code 2019 [Day 12](https://adventofcode.com/2019/day/12).

use itertools::Itertools;
use num::Integer;
use once_cell::sync::Lazy;
use regex::Regex;
use std::ops::Index;

fn main() {
    let (part1, part2) = day12();
    println!("part1 = {}", part1);
    println!("part2 = {}", part2);
}

fn day12() -> (i64, u64) {
    let vectors = parse_vectors(DAY12_INPUT);

    let mut data = SystemData::new(&vectors);
    for _ in 0..1000 {
        data.step();
    }
    let part1 = data.energy();

    let part2 = find_cycle_length(&vectors);

    (part1, part2)
}

const DAY12_INPUT: &str = "<x=-7, y=17, z=-11>\n\
                           <x=9, y=12, z=5>\n\
                           <x=-9, y=0, z=-4>\n\
                           <x=4, y=6, z=0>\n";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector3D([i64; 3]);

impl Index<usize> for Vector3D {
    type Output = i64;
    fn index(&self, idx: usize) -> &i64 {
        &self.0[idx]
    }
}

impl Vector3D {
    fn energy(self) -> i64 {
        self.0.iter().map(|x| x.abs()).sum()
    }
}

fn parse_vectors(input: &str) -> Vec<Vector3D> {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"<x=\s*(-?\d+),\s*y=\s*(-?\d+),\s*z=\s*(-?\d+)>").unwrap());

    RE.captures_iter(input)
        .map(|cap| {
            Vector3D([
                cap[1].parse::<i64>().unwrap(),
                cap[2].parse::<i64>().unwrap(),
                cap[3].parse::<i64>().unwrap(),
            ])
        })
        .collect_vec()
}

const NUM_BODIES: usize = 4;

#[derive(Debug)]
struct SystemData {
    axes: [AxisData; 3],
}

impl SystemData {
    fn new(initial_positions: &[Vector3D]) -> SystemData {
        let axes = [
            AxisData::new(&initial_positions, 0),
            AxisData::new(&initial_positions, 1),
            AxisData::new(&initial_positions, 2),
        ];
        SystemData { axes }
    }

    fn step(&mut self) {
        for a in &mut self.axes {
            a.step();
        }
    }

    fn state(&self) -> Vec<Vector3D> {
        (0..NUM_BODIES)
            .flat_map(|i| {
                vec![
                    Vector3D([
                        self.axes[0].positions[i],
                        self.axes[1].positions[i],
                        self.axes[2].positions[i],
                    ]),
                    Vector3D([
                        self.axes[0].velocities[i],
                        self.axes[1].velocities[i],
                        self.axes[2].velocities[i],
                    ]),
                ]
            })
            .collect()
    }

    fn energy(&self) -> i64 {
        let state = self.state();
        state
            .into_iter()
            .batching(|it| {
                let pos = it.next()?;
                let vel = it.next()?;
                Some(pos.energy() * vel.energy())
            })
            .sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct AxisData {
    positions: [i64; NUM_BODIES],
    velocities: [i64; NUM_BODIES],
}

impl AxisData {
    fn new(initial_positions: &[Vector3D], axis: usize) -> AxisData {
        AxisData {
            positions: [
                initial_positions[0][axis],
                initial_positions[1][axis],
                initial_positions[2][axis],
                initial_positions[3][axis],
            ],
            velocities: [0, 0, 0, 0],
        }
    }

    fn step(&mut self) {
        // gravity
        for i in 0..NUM_BODIES {
            for j in (i + 1)..NUM_BODIES {
                let pi = self.positions[i];
                let pj = self.positions[j];
                let to_i = num::clamp(pj - pi, -1, 1);
                self.velocities[i] += to_i;
                self.velocities[j] -= to_i;
            }
        }

        // velocity
        for i in 0..NUM_BODIES {
            self.positions[i] += self.velocities[i];
        }
    }
}

fn find_cycle_length(initial_positions: &[Vector3D]) -> u64 {
    let cycles = (0..=2)
        .map(|i| {
            let mut data = AxisData::new(initial_positions, i);
            let initial = data;
            let mut count = 0;
            loop {
                data.step();
                count += 1;
                if data == initial {
                    break count;
                }
            }
        })
        .collect_vec();
    cycles.iter().fold(1, |acc, x| acc.lcm(x))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        let vectors = parse_vectors(
            "<x=-1, y=0, z=2>\n\
             <x=2, y=-10, z=-7>\n\
             <x=4, y=-8, z=8>\n\
             <x=3, y=5, z=-1>",
        );
        let mut system = SystemData::new(&vectors);
        assert_eq!(
            system.state(),
            parse_vectors(
                "pos=<x=-1, y=  0, z= 2>, vel=<x= 0, y= 0, z= 0>\n\
                 pos=<x= 2, y=-10, z=-7>, vel=<x= 0, y= 0, z= 0>\n\
                 pos=<x= 4, y= -8, z= 8>, vel=<x= 0, y= 0, z= 0>\n\
                 pos=<x= 3, y=  5, z=-1>, vel=<x= 0, y= 0, z= 0>\n"
            )
        );

        system.step();
        assert_eq!(
            system.state(),
            parse_vectors(
                "pos=<x= 2, y=-1, z= 1>, vel=<x= 3, y=-1, z=-1>\n\
                 pos=<x= 3, y=-7, z=-4>, vel=<x= 1, y= 3, z= 3>\n\
                 pos=<x= 1, y=-7, z= 5>, vel=<x=-3, y= 1, z=-3>\n\
                 pos=<x= 2, y= 2, z= 0>, vel=<x=-1, y=-3, z= 1>\n"
            )
        );

        for _ in 0..9 {
            system.step();
        }
        assert_eq!(
            system.state(),
            parse_vectors(
                "pos=<x= 2, y= 1, z=-3>, vel=<x=-3, y=-2, z= 1>\n\
                 pos=<x= 1, y=-8, z= 0>, vel=<x=-1, y= 1, z= 3>\n\
                 pos=<x= 3, y=-6, z= 1>, vel=<x= 3, y= 2, z=-3>\n\
                 pos=<x= 2, y= 0, z= 4>, vel=<x= 1, y=-1, z=-1>\n"
            )
        );

        assert_eq!(system.energy(), 179);

        assert_eq!(find_cycle_length(&vectors), 2772);
    }

    #[test]
    fn test_example_data_2() {
        let vectors = parse_vectors(
            "<x=-8, y=-10, z=0>\n\
             <x=5, y=5, z=10>\n\
             <x=2, y=-7, z=3>\n\
             <x=9, y=-8, z=-3>\n",
        );

        let mut system = SystemData::new(&vectors);

        for _ in 0..100 {
            system.step();
        }

        assert_eq!(system.energy(), 1940);

        assert_eq!(find_cycle_length(&vectors), 4_686_774_924);
    }

    #[test]
    fn test_day12() {
        let (part1, part2) = day12();
        assert_eq!(part1, 7013);
        assert_eq!(part2, 324_618_307_124_784);
    }
}
