use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key(u32);

impl Key {
    pub fn from_mask(mask: u32) -> Key {
        Key(mask)
    }

    pub fn as_char(self) -> char {
        char::from(self)
    }

    pub fn as_mask(self) -> u32 {
        self.0
    }

    fn make_key_mask(c: char) -> Result<u32, String> {
        let index = match c {
            'a'..='z' => (c as u8) - b'a',
            '1'..='4' => 26 + (c.to_digit(10).unwrap() as u8),
            '@' => 26 + 5,
            _ => return Err(format!("Unknown key '{}'", c)),
        };
        Ok(1 << index)
    }

    fn mask_to_index(mut mask: u32) -> u8 {
        let mut index = 0;
        loop {
            mask >>= 1;
            if mask == 0 {
                break;
            }
            index += 1;
        }
        index
    }

    fn mask_to_char(mask: u32) -> char {
        let index = Key::mask_to_index(mask);
        let ascii = match index {
            0..=25 => b'a' + index,
            26..=30 => b'0' + (index - 26),
            31 => b'@',
            _ => panic!(),
        };
        ascii as char
    }
}

impl TryFrom<char> for Key {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let c = c.to_ascii_lowercase();
        let mask = Key::make_key_mask(c)?;
        Ok(Key(mask))
    }
}

impl From<Key> for char {
    fn from(k: Key) -> char {
        Key::mask_to_char(k.0)
    }
}

impl fmt::Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Key({})", char::from(*self))
    }
}
