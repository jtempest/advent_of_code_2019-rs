//! Solution to Advent of Code 2019 [Day 10](https://adventofcode.com/2019/day/10).

use aoc::geom::{Dimensions, Vector2D};
use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
struct AsteroidField {
    asteroids: HashSet<Vector2D>,
    dimensions: Dimensions,
}

impl AsteroidField {
    fn new(input: &str) -> AsteroidField {
        let lines = input.trim().lines();
        let dimensions = Dimensions {
            width: lines.clone().next().unwrap().len(),
            height: lines.clone().count(),
        };
        let asteroids = lines
            .enumerate()
            .flat_map(|(y, li)| {
                assert_eq!(li.len(), dimensions.width);
                li.trim()
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(x, _)| Vector2D {
                        x: x as i64,
                        y: y as i64,
                    })
            })
            .collect();
        AsteroidField {
            asteroids,
            dimensions,
        }
    }

    fn find_best_monitoring_asteroid(&self) -> (Vector2D, usize) {
        self.asteroids
            .iter()
            .copied()
            .map(|a| (a, self.num_visible_asteroids(a)))
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
    }

    fn num_visible_asteroids(&self, pos: Vector2D) -> usize {
        self.asteroids
            .iter()
            .copied()
            .map(|t| t - pos)
            .filter(|offset| *offset != Vector2D::zero())
            .map(clock_position)
            .collect::<HashSet<_>>()
            .len()
    }

    fn vaporisation_order(&self, station_pos: Vector2D) -> Vec<Vector2D> {
        assert!(self.asteroids.contains(&station_pos));

        // Sort by angle from the centrepoint, closer objects first when
        // they share an angle of attack.
        let mut offsets = self
            .asteroids
            .iter()
            .map(|a| *a - station_pos)
            .filter(|o| *o != Vector2D::zero())
            .map(|o| (clock_position(o) as u32, o))
            .collect::<Vec<_>>();

        offsets.sort_by(|a, b| {
            a.0.cmp(&b.0)
                .then(a.1.manhattan_length().cmp(&b.1.manhattan_length()))
        });

        // Make sure each asteroid has a unique angle - if any two are at
        // the same angle, the more distant one has a full rotation added.
        const FULL_ROTATION: u32 = std::u16::MAX as u32;
        let mut all_angles = HashSet::new();
        for (angle, _) in offsets.iter_mut() {
            while all_angles.contains(&angle) {
                *angle += FULL_ROTATION;
            }
            all_angles.insert(angle);
        }

        // Sort once again by angle, which are now all unique
        offsets.sort_by(|a, b| a.0.cmp(&b.0));

        // Done, convert back to original positions
        offsets.into_iter().map(|(_, o)| o + station_pos).collect()
    }
}

impl fmt::Display for AsteroidField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for coord in self.dimensions.iter() {
            if coord.x == 0 {
                writeln!(f)?;
            }
            let is_roid = self.asteroids.contains(&coord);
            let c = if is_roid { '#' } else { '.' };
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

// 0->65535, where 0 is straight up, 32768 is straight down, etc.
fn clock_position(offset: Vector2D) -> u16 {
    use std::f64::consts::PI;
    const TWO_PI: f64 = PI * 2.0;

    let angle = (-offset.x as f64).atan2(offset.y as f64);
    let dist = (angle + PI) / TWO_PI;
    ((dist * std::u16::MAX as f64) + 1.0) as u16
}

fn day10() -> (usize, usize) {
    const DAY10_INPUT: &str = include_str!("day10_input.txt");
    let field = AsteroidField::new(DAY10_INPUT);
    let best = field.find_best_monitoring_asteroid();
    let part1 = best.1;
    let order = field.vaporisation_order(best.0);
    let target = order[199];
    let part2 = ((target.x * 100) + target.y) as usize;
    (part1, part2)
}

fn main() {
    let (part1, part2) = day10();
    println!("part1 = {}", part1);
    println!("part1 = {}", part2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_clock_position() {
        assert_eq!(clock_position(Vector2D { x: 0, y: -1 }), 0);
        assert_eq!(clock_position(Vector2D { x: 1, y: 0 }), 16384);
        assert_eq!(clock_position(Vector2D { x: 0, y: 1 }), 32768);
        assert_eq!(clock_position(Vector2D { x: -1, y: 0 }), 49152);

        let clockwise = [
            Vector2D { x: 0, y: -1 },
            Vector2D { x: 1, y: -1 },
            Vector2D { x: 1, y: 0 },
            Vector2D { x: 1, y: 1 },
            Vector2D { x: 0, y: 1 },
            Vector2D { x: -1, y: 1 },
            Vector2D { x: -1, y: 0 },
            Vector2D { x: -1, y: -1 },
        ];

        for i in 0..(clockwise.len() - 1) {
            assert!(clock_position(clockwise[i]) < clock_position(clockwise[i + 1]));
        }
    }

    const EXAMPLE_FIELDS: [&str; 5] = [
        include_str!("day10_example1.txt"),
        include_str!("day10_example2.txt"),
        include_str!("day10_example3.txt"),
        include_str!("day10_example4.txt"),
        include_str!("day10_example5.txt"),
    ];

    #[test]
    fn test_find_best_monitoring_asteroid() {
        check_find_best_monitoring_asteroid(EXAMPLE_FIELDS[0], (Vector2D { x: 3, y: 4 }, 8));
        check_find_best_monitoring_asteroid(EXAMPLE_FIELDS[1], (Vector2D { x: 5, y: 8 }, 33));
        check_find_best_monitoring_asteroid(EXAMPLE_FIELDS[2], (Vector2D { x: 1, y: 2 }, 35));
        check_find_best_monitoring_asteroid(EXAMPLE_FIELDS[3], (Vector2D { x: 6, y: 3 }, 41));
        check_find_best_monitoring_asteroid(EXAMPLE_FIELDS[4], (Vector2D { x: 11, y: 13 }, 210));
    }

    fn check_find_best_monitoring_asteroid(input: &str, expected: (Vector2D, usize)) {
        let best = AsteroidField::new(input).find_best_monitoring_asteroid();
        assert_eq!(best, expected);
    }

    #[test]
    fn test_vaporisation_order() {
        let field = AsteroidField::new(EXAMPLE_FIELDS[4]);
        let pos = field.find_best_monitoring_asteroid().0;
        let order = field.vaporisation_order(pos);

        assert_eq!(order.len(), 299);
        assert_eq!(order[0], Vector2D { x: 11, y: 12 });
        assert_eq!(order[1], Vector2D { x: 12, y: 1 });
        assert_eq!(order[2], Vector2D { x: 12, y: 2 });
        assert_eq!(order[9], Vector2D { x: 12, y: 8 });
        assert_eq!(order[19], Vector2D { x: 16, y: 0 });
        assert_eq!(order[49], Vector2D { x: 16, y: 9 });
        assert_eq!(order[99], Vector2D { x: 10, y: 16 });
        assert_eq!(order[198], Vector2D { x: 9, y: 6 });
        assert_eq!(order[199], Vector2D { x: 8, y: 2 });
        assert_eq!(order[200], Vector2D { x: 10, y: 9 });
        assert_eq!(order[298], Vector2D { x: 11, y: 1 });
    }

    #[test]
    fn test_day10() {
        let (part1, part2) = day10();
        assert_eq!(part1, 292);
        assert_eq!(part2, 317);
    }
}
