use crate::geom::Vector2D;
use std::convert::TryInto;

use std::cmp;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

impl Dimensions {
    pub fn new() -> Dimensions {
        Dimensions::default()
    }

    pub fn area(self) -> usize {
        self.width * self.height
    }

    pub fn iter(self) -> DimensionsIter {
        DimensionsIter {
            limits: Some(self),
            current: Vector2D::zero(),
        }
    }

    pub fn expand_to_fit(&mut self, pos: Vector2D) {
        let (x, y) = (pos.x as usize, pos.y as usize);
        self.width = cmp::max(self.width, x + 1);
        self.height = cmp::max(self.height, y + 1);
    }

    pub fn pos_to_node_index(self, pos: Vector2D) -> usize {
        let (x, y) = (pos.x as usize, pos.y as usize);
        let width = self.width;
        (y * width) + x
    }

    pub fn node_index_to_pos(self, index: usize) -> Vector2D {
        let width = self.width;
        let x = (index % width) as i64;
        let y = (index / width) as i64;
        Vector2D { x, y }
    }

    pub fn contains(self, pos: Vector2D) -> bool {
        let width = self.width as i64;
        let height = self.height as i64;
        (pos.x >= 0 && pos.x < width) && (pos.y >= 0) && (pos.y < height)
    }

    pub fn centre(self) -> Vector2D {
        Vector2D {
            x: (self.width / 2).try_into().unwrap(),
            y: (self.height / 2).try_into().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DimensionsIter {
    limits: Option<Dimensions>,
    current: Vector2D,
}

impl Iterator for DimensionsIter {
    type Item = Vector2D;

    fn next(&mut self) -> Option<Vector2D> {
        let limits = self.limits?;
        let coord = self.current;
        self.current.x += 1;
        if self.current.x == limits.width as i64 {
            self.current.x = 0;
            self.current.y += 1;
            if self.current.y == limits.height as i64 {
                self.limits = None;
            }
        }
        Some(coord)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimensions_area() {
        assert_eq!(
            Dimensions {
                width: 5,
                height: 10
            }
            .area(),
            50
        );
    }

    #[test]
    fn dimensions_iter() {
        let items = Dimensions {
            width: 3,
            height: 2,
        }
        .iter()
        .collect::<Vec<_>>();

        let expected = [(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)]
            .iter()
            .copied()
            .map(Vector2D::from)
            .collect::<Vec<_>>();
        assert_eq!(items, expected);
    }

    #[test]
    fn dimensions_contains() {
        let dims = Dimensions {
            width: 3,
            height: 5,
        };

        assert!(dims.contains(Vector2D::default()));
        assert!(dims.contains(Vector2D { x: 2, y: 4 }));

        assert!(!dims.contains(Vector2D { x: 0, y: -1 }));
        assert!(!dims.contains(Vector2D { x: -1, y: 0 }));
        assert!(!dims.contains(Vector2D { x: 2, y: 5 }));
        assert!(!dims.contains(Vector2D { x: 3, y: 4 }));
    }
}
