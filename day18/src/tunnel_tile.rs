use crate::key::Key;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TunnelTile {
    Wall,
    Empty,
    Player(Key),
    Key(Key),
    Door(Key),
}

impl TunnelTile {
    pub fn as_char(self) -> char {
        char::from(self)
    }

    pub fn is_wall(self) -> bool {
        if let TunnelTile::Wall = self {
            true
        } else {
            false
        }
    }
}

impl TryFrom<char> for TunnelTile {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(TunnelTile::Wall),
            '.' => Ok(TunnelTile::Empty),
            '@' => Ok(TunnelTile::Player(Key::try_from(c)?)),
            'a'..='z' => Ok(TunnelTile::Key(Key::try_from(c)?)),
            'A'..='Z' => Ok(TunnelTile::Door(Key::try_from(c)?)),
            _ => Err(format!("Unknown character '{}'", c)),
        }
    }
}

impl From<TunnelTile> for char {
    fn from(tile: TunnelTile) -> char {
        match tile {
            TunnelTile::Wall => '#',
            TunnelTile::Empty => '.',
            TunnelTile::Player(key) => key.as_char(),
            TunnelTile::Key(key) => key.as_char(),
            TunnelTile::Door(key) => key.as_char().to_ascii_uppercase(),
        }
    }
}
