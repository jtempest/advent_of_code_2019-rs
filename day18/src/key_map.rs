use crate::key::Key;
use crate::key_set::KeySet;
use crate::tunnel_map::{TunnelMap, TunnelPath};
use fnv::{FnvHashMap, FnvHashSet};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct KeyMap {
    edges: FnvHashMap<Key, Vec<TunnelPath>>,
    all_keys: KeySet,
}

impl From<&TunnelMap> for KeyMap {
    fn from(map: &TunnelMap) -> Self {
        let edges = map.find_all_paths_from_keys();
        let all_keys = map.all_keys();
        KeyMap { edges, all_keys }
    }
}

impl TryFrom<&str> for KeyMap {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let tunnels = TunnelMap::try_from(input)?;
        Ok(KeyMap::from(&tunnels))
    }
}

impl KeyMap {
    pub fn make_quadrants(input: &str) -> Result<KeyMap, String> {
        let tunnels = TunnelMap::make_quadrants(input)?;
        Ok(KeyMap::from(&tunnels))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct SearchState {
    location: KeySet,
    collected_keys: KeySet,
    distance: usize,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl KeyMap {
    pub fn find_quickest_path_to_all_keys(&self) -> Option<usize> {
        let location = self.start_location();

        let mut open = BinaryHeap::new();
        open.push(SearchState {
            location,
            collected_keys: location,
            distance: 0,
        });

        let mut seen = FnvHashSet::default();

        while let Some(state) = open.pop() {
            let SearchState {
                location,
                collected_keys,
                distance,
            } = state;

            if !seen.insert((location, collected_keys)) {
                continue;
            }

            if collected_keys == self.all_keys {
                return Some(state.distance);
            }

            for key in location.iter() {
                open.extend(
                    self.edges[&key]
                        .iter()
                        .filter(|path| !collected_keys.contains(path.dest))
                        .filter(|path| collected_keys.contains_all(path.doors))
                        .map(|path| {
                            let mut location = location;
                            location.remove(key);
                            location.insert(path.dest);

                            let mut collected_keys = collected_keys;
                            collected_keys.insert(path.dest);

                            SearchState {
                                location,
                                collected_keys,
                                distance: distance + path.distance,
                            }
                        }),
                );
            }
        }

        None
    }

    fn start_location(&self) -> KeySet {
        let one_robot_key: Key = Key::try_from('@').unwrap();
        if self.edges.contains_key(&one_robot_key) {
            KeySet::from(one_robot_key)
        } else {
            ['1', '2', '3', '4']
                .iter()
                .map(|&c| Key::try_from(c).unwrap())
                .collect()
        }
    }
}
