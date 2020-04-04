use crate::key::Key;
use std::fmt;
use std::iter::FromIterator;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeySet(u32);

impl KeySet {
    pub fn new() -> KeySet {
        KeySet(0)
    }

    pub fn insert(&mut self, key: Key) {
        self.0 |= key.as_mask();
    }

    pub fn remove(&mut self, key: Key) {
        self.0 &= !key.as_mask()
    }

    pub fn contains(self, key: Key) -> bool {
        let result = self.0 & key.as_mask();
        result > 0
    }

    pub fn contains_all(self, set: KeySet) -> bool {
        let result = self.0 & set.0;
        result == set.0
    }

    pub fn iter(self) -> impl Iterator<Item = Key> {
        (0..32)
            .map(|index| Key::from_mask(1 << index))
            .filter(move |&key| self.contains(key))
    }
}

impl From<Key> for KeySet {
    fn from(key: Key) -> Self {
        KeySet(key.as_mask())
    }
}

impl FromIterator<Key> for KeySet {
    fn from_iter<I: IntoIterator<Item = Key>>(iter: I) -> Self {
        let mut set = KeySet::new();
        for k in iter {
            set.insert(k);
        }
        set
    }
}

impl fmt::Debug for KeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "KeySet(")?;
        let mut mask = 1;
        for _ in 0..26 {
            let key = Key::from_mask(mask);
            mask <<= 1;

            if self.contains(key) {
                write!(f, "{}", char::from(key))?;
            }
        }
        write!(f, ")")
    }
}
