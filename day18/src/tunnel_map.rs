use crate::key::Key;
use crate::key_set::KeySet;
use crate::tunnel_tile::TunnelTile;
use aoc::geom::{self, Dimensions, Vector2D};
use fnv::{FnvHashMap, FnvHashSet};
use std::convert::TryFrom;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TunnelMap {
    dimensions: Dimensions,
    tiles: Vec<TunnelTile>,
    keys: FnvHashMap<Key, Vector2D>,
    doors: FnvHashMap<Vector2D, Key>,
}

impl TunnelMap {
    pub fn make_quadrants(input: &str) -> Result<TunnelMap, String> {
        let mut map = TunnelMap::try_from(input)?;

        // fill in walls
        let &player = map.key_pos('@').unwrap();
        map[player] = TunnelTile::Wall;
        player.neighbours().for_each(|n| map[n] = TunnelTile::Wall);
        map.keys.remove(&Key::try_from('@').unwrap());

        // fill in new start positions
        [(-1, -1), (1, -1), (-1, 1), (1, 1)]
            .iter()
            .map(|&pos| player + pos.into())
            .enumerate()
            .map(|(i, pos)| (b'1' + (i as u8), pos))
            .map(|(c, pos)| (Key::try_from(c as char).unwrap(), pos))
            .for_each(|(key, pos)| {
                map[pos] = TunnelTile::Player(key);
                map.keys.insert(key, pos);
            });

        Ok(map)
    }

    pub fn get(&self, pos: Vector2D) -> Option<&TunnelTile> {
        self.tiles.get(self.index(pos))
    }

    pub fn get_mut(&mut self, pos: Vector2D) -> Option<&mut TunnelTile> {
        let index = self.index(pos);
        self.tiles.get_mut(index)
    }

    pub fn all_keys(&self) -> KeySet {
        self.keys.keys().copied().collect()
    }

    fn key_pos(&self, c: char) -> Option<&Vector2D> {
        let key = Key::try_from(c).unwrap();
        self.keys.get(&key)
    }

    fn index(&self, pos: Vector2D) -> usize {
        let (x, y) = (pos.x as usize, pos.y as usize);
        (y * self.dimensions.width) + x
    }
}

impl Index<Vector2D> for TunnelMap {
    type Output = TunnelTile;

    fn index(&self, pos: Vector2D) -> &Self::Output {
        self.get(pos).unwrap()
    }
}

impl IndexMut<Vector2D> for TunnelMap {
    fn index_mut(&mut self, pos: Vector2D) -> &mut Self::Output {
        self.get_mut(pos).unwrap()
    }
}

impl TryFrom<&str> for TunnelMap {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut tiles = Vec::new();
        let mut dimensions = Dimensions::new();
        let mut keys = FnvHashMap::default();
        let mut doors = FnvHashMap::default();

        for (pos, c) in geom::cartograph(input) {
            dimensions.expand_to_fit(pos);

            let tile = TunnelTile::try_from(c).map_err(|e| format!("{}: {}", pos, e))?;

            if let TunnelTile::Player(key) = tile {
                keys.insert(key, pos);
            } else if let TunnelTile::Key(key) = tile {
                keys.insert(key, pos);
            } else if let TunnelTile::Door(key) = tile {
                doors.insert(pos, key);
            }

            tiles.push(tile);
        }

        Ok(TunnelMap {
            dimensions,
            tiles,
            keys,
            doors,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TunnelPath {
    pub dest: Key,
    pub distance: usize,
    pub doors: KeySet,
}

impl TunnelMap {
    pub fn find_all_paths_from_keys(&self) -> FnvHashMap<Key, Vec<TunnelPath>> {
        self.keys
            .iter()
            .map(|(&k, &pos)| (k, self.find_all_paths_from_pos(pos)))
            .collect()
    }

    fn find_all_paths_from_pos(&self, start: Vector2D) -> Vec<TunnelPath> {
        let mut destinations = Vec::new();

        let mut seen = FnvHashSet::default();
        let mut open = vec![(start, KeySet::new(), 0)];

        while let Some((pos, doors, distance)) = open.pop() {
            if !seen.insert(pos) {
                continue;
            }

            let tile = self[pos];
            if distance > 0 {
                if let TunnelTile::Key(key) = tile {
                    destinations.push(TunnelPath {
                        dest: key,
                        distance,
                        doors,
                    });
                }
            }

            let mut doors = doors;
            if let TunnelTile::Door(key) = tile {
                doors.insert(key);
            }

            let next = pos
                .neighbours()
                .filter(|n| !seen.contains(&n))
                .map(|n| (n, self[n]))
                .filter(|(_, t)| !t.is_wall());

            for (neighbour, _) in next {
                open.push((neighbour, doors, distance + 1));
            }

            open.sort_by(|a, b| a.2.cmp(&b.2).reverse())
        }

        destinations
    }
}

impl fmt::Display for TunnelMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for pos in self.dimensions.iter() {
            if pos.x == 0 && pos.y > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", self[pos].as_char())?;
        }
        Ok(())
    }
}
