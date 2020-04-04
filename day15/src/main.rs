//! Solution to Advent of Code 2019 [Day 15](https://adventofcode.com/2019/day/15).

// Notes:
// - Path appears to be one tile wide
// - There are multiple paths with dead ends, so will need to backtrack

use aoc::geom::{Dimensions, Vector2D};
use aoc::graph::{Edge, Graph};
use aoc::intcode::Machine;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const RENDER_FINAL_STATE: bool = false;

fn main() {
    let (part1, part2) = day15();
    println!("part1 = {}", part1);
    println!("part2 = {}", part2);
}

fn day15() -> (usize, usize) {
    let mut droid = RepairDroid::new();
    while !droid.explored_everything() {
        droid.explore_one_tile();
    }

    if RENDER_FINAL_STATE {
        clear_console();
        println!("{}", droid.render());
    }

    let part1 = droid.distance_of_oxygen_from_start().unwrap();
    let part2 = droid.time_for_oxygen_to_percolate().unwrap();

    (part1, part2)
}

fn clear_console() {
    print!("\x1B[2J");
}

const DAY15_INPUT: &str = include_str!("day15_input.txt");

#[derive(Debug)]
struct RepairDroid {
    machine: Machine,
    position: Vector2D,
    world_map: WorldMap,
}

impl RepairDroid {
    fn new() -> RepairDroid {
        let mut droid = RepairDroid {
            machine: Machine::from_source(DAY15_INPUT),
            position: Vector2D::zero(),
            world_map: WorldMap::new(),
        };
        droid.record_location(droid.position, LocationType::Start);
        droid.record_move(droid.position);
        droid
    }

    fn explored_everything(&self) -> bool {
        self.world_map.explored_everything()
    }

    fn distance_of_oxygen_from_start(&self) -> Option<usize> {
        let oxygen_pos = self.oxygen_system_pos()?;
        Some(self.find_path_to(Vector2D::zero(), oxygen_pos).len())
    }

    fn oxygen_system_pos(&self) -> Option<Vector2D> {
        self.world_map.oxygen_system_pos()
    }

    fn time_for_oxygen_to_percolate(&self) -> Option<usize> {
        let oxygen_pos = self.oxygen_system_pos()?;
        let start = self.world_map.vector2d_to_node_index(oxygen_pos);
        let dist = self.world_map.farthest_distance_from(start);
        Some(dist)
    }

    fn explore_one_tile(&mut self) {
        if let Some(dest) = self.world_map.next_unexplored_tile() {
            for c in self.find_path_to(self.position, dest) {
                self.execute_command(c);
            }
        }
    }

    fn execute_command(&mut self, command: MovementCommand) {
        let direction = Vector2D::from(command);
        self.machine.input(i64::from(command));
        let status = self.machine.run().unwrap();

        let location = self.position + direction;
        let location_type = LocationType::from(status);
        self.record_location(location, location_type);

        match location_type {
            LocationType::Wall => (),
            LocationType::Empty => self.record_move(location),
            LocationType::OxygenSystem => self.record_move(location),
            _ => panic!("Err..."),
        }
    }

    fn record_move(&mut self, location: Vector2D) {
        self.position = location;
        for n in self.position.neighbours() {
            self.record_location(n, LocationType::Reachable);
        }
    }

    fn record_location(&mut self, location: Vector2D, location_type: LocationType) {
        self.world_map.record_location(location, location_type);
    }

    fn render(&self) -> String {
        self.world_map.render(self.position)
    }

    fn find_path_to(&self, start: Vector2D, destination: Vector2D) -> Vec<MovementCommand> {
        self.world_map
            .find_shortest_path(start, destination)
            .into_iter()
            .tuple_windows::<(_, _)>()
            .map(|(pos, next)| next - pos)
            .map(MovementCommand::from)
            .collect()
    }
}

#[derive(Debug)]
struct WorldMap {
    map: HashMap<Vector2D, LocationType>,
    top_left: Vector2D,
    bottom_right: Vector2D,
    oxygen_system_pos: Option<Vector2D>,
    unknown_locations: HashSet<Vector2D>,
}

impl WorldMap {
    fn new() -> WorldMap {
        WorldMap {
            map: HashMap::new(),
            top_left: Vector2D::zero(),
            bottom_right: Vector2D::zero(),
            oxygen_system_pos: None,
            unknown_locations: HashSet::new(),
        }
    }

    fn explored_everything(&self) -> bool {
        self.unknown_locations.is_empty()
    }

    fn next_unexplored_tile(&self) -> Option<Vector2D> {
        self.unknown_locations.iter().copied().next()
    }

    fn record_location(&mut self, location: Vector2D, location_type: LocationType) {
        let is_known = location_type != LocationType::Reachable;
        let should_record = is_known || !self.map.contains_key(&location);

        if should_record {
            self.map.insert(location, location_type);

            if is_known {
                self.unknown_locations.remove(&location);
            } else {
                self.unknown_locations.insert(location);
            }

            if location_type == LocationType::OxygenSystem {
                self.oxygen_system_pos = Some(location);
            }

            self.ensure_dimensions_contain(location);
        }
    }

    fn ensure_dimensions_contain(&mut self, location: Vector2D) {
        self.top_left = self.top_left.min_components(location);
        self.bottom_right = self
            .bottom_right
            .max_components(location + Vector2D { x: 1, y: 1 });
    }

    fn oxygen_system_pos(&self) -> Option<Vector2D> {
        self.oxygen_system_pos
    }

    fn dimensions(&self) -> Dimensions {
        let diff = self.bottom_right - self.top_left + Vector2D { x: 1, y: 1 };
        Dimensions {
            width: diff.x as usize,
            height: diff.y as usize,
        }
    }

    fn find_shortest_path(&self, start: Vector2D, destination: Vector2D) -> Vec<Vector2D> {
        let start = self.vector2d_to_node_index(start);
        let destination = self.vector2d_to_node_index(destination);
        let path = self.find_shortest_path_indices(start, destination).unwrap();
        path.into_iter()
            .map(|i| self.node_index_to_vector2d(i))
            .collect()
    }

    fn render(&self, droid_position: Vector2D) -> String {
        let mut canvas = String::new();
        for pos in self.dimensions().iter() {
            if pos.y > 0 && pos.x == 0 {
                canvas.push('\n');
            }

            let pos = pos + self.top_left;
            let loc = *self.map.get(&pos).unwrap_or(&LocationType::Unknown);
            let c = if pos == droid_position {
                'D'
            } else {
                char::from(loc)
            };
            canvas.push(c);
        }
        canvas
    }

    fn vector2d_to_node_index(&self, v: Vector2D) -> usize {
        let abs_pos = v - self.top_left;
        let (x, y) = (abs_pos.x as usize, abs_pos.y as usize);
        let dims = self.dimensions();
        (y * dims.width) + x
    }

    fn node_index_to_vector2d(&self, node_index: usize) -> Vector2D {
        let width = self.dimensions().width;
        let x = (node_index % width) as i64;
        let y = (node_index / width) as i64;
        Vector2D { x, y } + self.top_left
    }
}

impl Graph for WorldMap {
    fn num_nodes(&self) -> usize {
        self.vector2d_to_node_index(self.bottom_right) + 1
    }

    fn node_edges(&self, node_index: usize) -> Vec<Edge> {
        let v = self.node_index_to_vector2d(node_index);
        v.neighbours()
            .map(|n| (n, self.map.get(&n)))
            .filter(|(_, lt)| lt.is_some() && lt.unwrap().is_traversible())
            .map(|(n, _)| Edge {
                dest_index: self.vector2d_to_node_index(n),
                cost: 1,
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
enum MovementCommand {
    North,
    South,
    West,
    East,
}

impl From<char> for MovementCommand {
    fn from(c: char) -> MovementCommand {
        match c {
            'N' => MovementCommand::North,
            'S' => MovementCommand::South,
            'W' => MovementCommand::West,
            'E' => MovementCommand::East,
            _ => panic!("Unknown command '{}'", c),
        }
    }
}

impl From<MovementCommand> for i64 {
    fn from(command: MovementCommand) -> i64 {
        match command {
            MovementCommand::North => 1,
            MovementCommand::South => 2,
            MovementCommand::West => 3,
            MovementCommand::East => 4,
        }
    }
}

impl From<MovementCommand> for Vector2D {
    fn from(command: MovementCommand) -> Vector2D {
        match command {
            MovementCommand::North => Vector2D { x: 0, y: -1 },
            MovementCommand::South => Vector2D { x: 0, y: 1 },
            MovementCommand::West => Vector2D { x: -1, y: 0 },
            MovementCommand::East => Vector2D { x: 1, y: 0 },
        }
    }
}

impl From<Vector2D> for MovementCommand {
    fn from(diff: Vector2D) -> MovementCommand {
        match diff {
            Vector2D { x: 0, y: -1 } => MovementCommand::North,
            Vector2D { x: 0, y: 1 } => MovementCommand::South,
            Vector2D { x: -1, y: 0 } => MovementCommand::West,
            Vector2D { x: 1, y: 0 } => MovementCommand::East,
            _ => panic!("Unknown movement command from vector {}", diff),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LocationType {
    Wall,
    Empty,
    OxygenSystem,
    Start,
    Reachable,
    Unknown,
}

impl LocationType {
    fn is_traversible(self) -> bool {
        match self {
            LocationType::Wall => false,
            LocationType::Empty => true,
            LocationType::OxygenSystem => true,
            LocationType::Start => true,
            LocationType::Reachable => true,
            LocationType::Unknown => false,
        }
    }
}

impl From<i64> for LocationType {
    fn from(value: i64) -> LocationType {
        match value {
            0 => LocationType::Wall,
            1 => LocationType::Empty,
            2 => LocationType::OxygenSystem,
            _ => panic!("Unknown LocationType '{}'", value),
        }
    }
}

impl From<LocationType> for char {
    fn from(loc_type: LocationType) -> char {
        match loc_type {
            LocationType::Wall => '#',
            LocationType::Empty => '.',
            LocationType::OxygenSystem => 'o',
            LocationType::Start => 's',
            LocationType::Reachable => '?',
            LocationType::Unknown => ' ',
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day15() {
        let (part1, part2) = day15();
        assert_eq!(part1, 424);
        assert_eq!(part2, 446);
    }
}
