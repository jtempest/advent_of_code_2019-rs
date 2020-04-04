use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Vector2D {
    pub x: i64,
    pub y: i64,
}

impl<T, U> From<(T, U)> for Vector2D
where
    T: Into<i64>,
    U: Into<i64>,
{
    fn from((x, y): (T, U)) -> Vector2D {
        Vector2D {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl Default for Vector2D {
    fn default() -> Vector2D {
        Vector2D { x: 0, y: 0 }
    }
}

impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, rhs: Vector2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vector2D {
    fn sub_assign(&mut self, rhs: Vector2D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Vector2D {
    pub fn zero() -> Vector2D {
        Vector2D::default()
    }

    pub fn manhattan_length(self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    pub fn min_components(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn max_components(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    pub fn neighbours(self) -> Neighbours {
        Neighbours::new(self)
    }
}

impl fmt::Display for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{},{}}}", self.x, self.y)
    }
}

const CARDINAL_DIRECTIONS: [Vector2D; 4] = [
    Vector2D { x: -1, y: 0 },
    Vector2D { x: 1, y: 0 },
    Vector2D { x: 0, y: -1 },
    Vector2D { x: 0, y: 1 },
];

pub struct Neighbours {
    centre: Vector2D,
    index: usize,
}

impl Neighbours {
    fn new(centre: Vector2D) -> Neighbours {
        Neighbours { centre, index: 0 }
    }
}

impl Iterator for Neighbours {
    type Item = Vector2D;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < CARDINAL_DIRECTIONS.len() {
            let v = self.centre + CARDINAL_DIRECTIONS[self.index];
            self.index += 1;
            Some(v)
        } else {
            None
        }
    }
}

pub fn cartograph<'a>(input: &'a str) -> impl Iterator<Item = (Vector2D, char)> + 'a {
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, c)| {
            let x = x as i64;
            let y = y as i64;
            (Vector2D { x, y }, c)
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector2d_add() {
        assert_eq!(
            Vector2D { x: 1, y: 2 } + Vector2D { x: -10, y: 15 },
            Vector2D { x: -9, y: 17 }
        );
    }

    #[test]
    fn vector2d_add_assign() {
        let mut v = Vector2D { x: -6, y: -10 };
        v += Vector2D { x: 1, y: -5 };
        assert_eq!(v, Vector2D { x: -5, y: -15 });
    }

    #[test]
    fn vector2d_sub() {
        assert_eq!(
            Vector2D { x: 1, y: 2 } - Vector2D { x: -10, y: 15 },
            Vector2D { x: 11, y: -13 }
        );
    }

    #[test]
    fn vector2d_sub_assign() {
        let mut v = Vector2D { x: -6, y: -10 };
        v -= Vector2D { x: 1, y: -5 };
        assert_eq!(v, Vector2D { x: -7, y: -5 });
    }

    #[test]
    fn vector2d_zero() {
        assert_eq!(Vector2D::zero(), Vector2D { x: 0, y: 0 });
    }

    #[test]
    fn vector2d_manhattan_length() {
        assert_eq!(Vector2D::zero().manhattan_length(), 0);
        assert_eq!(Vector2D { x: 1, y: 2 }.manhattan_length(), 3);
        assert_eq!(Vector2D { x: -5, y: 3 }.manhattan_length(), 8);
        assert_eq!(Vector2D { x: 5, y: -3 }.manhattan_length(), 8);
        assert_eq!(Vector2D { x: -5, y: -3 }.manhattan_length(), 8);
    }

    #[test]
    fn vector2d_min_components() {
        assert_eq!(
            Vector2D::zero().min_components(Vector2D::zero()),
            Vector2D::zero()
        );
        assert_eq!(
            Vector2D { x: 0, y: 1 }.min_components(Vector2D { x: 2, y: 3 }),
            Vector2D { x: 0, y: 1 }
        );
        assert_eq!(
            Vector2D { x: 0, y: 3 }.min_components(Vector2D { x: 1, y: 2 }),
            Vector2D { x: 0, y: 2 }
        );
        assert_eq!(
            Vector2D { x: 4, y: 2 }.min_components(Vector2D { x: 1, y: 3 }),
            Vector2D { x: 1, y: 2 }
        );
        assert_eq!(
            Vector2D { x: 4, y: 3 }.min_components(Vector2D { x: 2, y: 1 }),
            Vector2D { x: 2, y: 1 }
        );
    }

    #[test]
    fn vector2d_max_components() {
        assert_eq!(
            Vector2D::zero().max_components(Vector2D::zero()),
            Vector2D::zero()
        );
        assert_eq!(
            Vector2D { x: 0, y: 1 }.max_components(Vector2D { x: 2, y: 3 }),
            Vector2D { x: 2, y: 3 }
        );
        assert_eq!(
            Vector2D { x: 0, y: 3 }.max_components(Vector2D { x: 1, y: 2 }),
            Vector2D { x: 1, y: 3 }
        );
        assert_eq!(
            Vector2D { x: 4, y: 2 }.max_components(Vector2D { x: 1, y: 3 }),
            Vector2D { x: 4, y: 3 }
        );
        assert_eq!(
            Vector2D { x: 4, y: 3 }.max_components(Vector2D { x: 2, y: 1 }),
            Vector2D { x: 4, y: 3 }
        );
    }

    #[test]
    fn vector2d_neighbours() {
        use std::collections::HashSet;

        let neighbours = Vector2D { x: 5, y: -2 }
            .neighbours()
            .collect::<HashSet<_>>();

        assert_eq!(neighbours.len(), 4);
        assert!(neighbours.contains(&Vector2D { x: 4, y: -2 }));
        assert!(neighbours.contains(&Vector2D { x: 6, y: -2 }));
        assert!(neighbours.contains(&Vector2D { x: 5, y: -1 }));
        assert!(neighbours.contains(&Vector2D { x: 5, y: -3 }));
    }

    #[test]
    fn test_cartograph() {
        let map = cartograph("123\r\n45\n6789\n").collect::<Vec<_>>();
        assert_eq!(
            map,
            vec![
                (Vector2D { x: 0, y: 0 }, '1'),
                (Vector2D { x: 1, y: 0 }, '2'),
                (Vector2D { x: 2, y: 0 }, '3'),
                (Vector2D { x: 0, y: 1 }, '4'),
                (Vector2D { x: 1, y: 1 }, '5'),
                (Vector2D { x: 0, y: 2 }, '6'),
                (Vector2D { x: 1, y: 2 }, '7'),
                (Vector2D { x: 2, y: 2 }, '8'),
                (Vector2D { x: 3, y: 2 }, '9'),
            ]
        );
    }
}
