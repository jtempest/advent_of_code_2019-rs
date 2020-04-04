//! Solution to Advent of Code 2019 [Day 8](https://adventofcode.com/2019/day/8).

use aoc::geom::{Dimensions, Vector2D};
use aoc::ocr::{ocr, LetterImage, LETTER_IMAGE_DIMENSIONS};
use std::fmt;
use std::iter;
use std::ops::Index;

#[derive(Debug)]
struct Image {
    layers: Vec<Layer>,
    dimensions: Dimensions,
}

impl Image {
    fn new(data: &str, dimensions: Dimensions) -> Image {
        let layers = layers(data.trim(), dimensions).collect();
        Image { layers, dimensions }
    }

    fn render(&self) -> Image {
        Image {
            layers: vec![self.render_to_layer()],
            dimensions: self.dimensions,
        }
    }

    fn render_to_layer(&self) -> Layer {
        if self.layers.len() > 1 {
            let mut canvas = iter::repeat(2).take(self.dimensions.area()).collect();
            for layer in &self.layers {
                layer.render(&mut canvas);
            }
            Layer {
                data: canvas,
                dimensions: self.dimensions,
            }
        } else {
            self.layers[0].clone()
        }
    }

    fn sub_image(&self, top_left: Vector2D, dimensions: Dimensions) -> Image {
        let layer = self.render_to_layer();
        Image {
            layers: vec![layer.sub_layer(top_left, dimensions)],
            dimensions,
        }
    }

    fn layer(&self, n: usize) -> &Layer {
        &self.layers[n]
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render_to_layer())
    }
}

#[derive(Debug, Clone)]
struct Layer {
    data: Vec<u8>,
    dimensions: Dimensions,
}

impl Layer {
    fn count(&self, digit: u8) -> usize {
        self.data.iter().copied().filter(|d| (*d) == digit).count()
    }

    fn render(&self, canvas: &mut Vec<u8>) {
        assert_eq!(self.data.len(), canvas.len());
        for (n, &colour) in self.data.iter().enumerate() {
            if canvas[n] == 2 {
                canvas[n] = colour;
            }
        }
    }

    fn sub_layer(&self, top_left: Vector2D, dimensions: Dimensions) -> Layer {
        let data = dimensions
            .iter()
            .map(|offset| self[top_left + offset])
            .collect();
        Layer { data, dimensions }
    }

    fn iter(&self) -> impl Iterator<Item = (Vector2D, &u8)> {
        self.dimensions.iter().zip(self.data.iter())
    }
}

impl Index<Vector2D> for Layer {
    type Output = u8;

    fn index(&self, pos: Vector2D) -> &u8 {
        let (x, y) = (pos.x as usize, pos.y as usize);
        let index = (y * self.dimensions.width) + x;
        &self.data[index]
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for pos in self.dimensions.iter() {
            if pos.x == 0 && pos.y != 0 {
                writeln!(f)?;
            }
            let c = if self[pos] == 1 { '@' } else { ' ' };
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

fn layers(data: &str, dimensions: Dimensions) -> Layers {
    Layers {
        remaining: data,
        dimensions,
        layer_length: dimensions.area(),
    }
}

struct Layers<'a> {
    remaining: &'a str,
    dimensions: Dimensions,
    layer_length: usize,
}

impl Iterator for Layers<'_> {
    type Item = Layer;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            None
        } else {
            let (layer, rest) = self.remaining.split_at(self.layer_length);
            let layer = layer
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>();
            assert_eq!(layer.len(), self.layer_length);
            self.remaining = rest;
            Some(Layer {
                data: layer,
                dimensions: self.dimensions,
            })
        }
    }
}

fn day08() -> (usize, String) {
    const DAY08_INPUT: &str = include_str!("day08_input.txt");
    let img = Image::new(
        DAY08_INPUT,
        Dimensions {
            width: 25,
            height: 6,
        },
    );
    (day08_part1(&img), day08_part2(&img))
}

fn day08_part1(img: &Image) -> usize {
    let layer = img
        .layers
        .iter()
        .map(|x| (x, x.count(0)))
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0;

    layer.count(1) * layer.count(2)
}

fn day08_part2(img: &Image) -> String {
    let rendered = img.render();
    iter::successors(Some(0), |x| Some(x + 5))
        .take_while(|x| (*x) < rendered.dimensions.width)
        .map(|x| Vector2D { x: x as i64, y: 0 })
        .map(|offset| rendered.sub_image(offset, LETTER_IMAGE_DIMENSIONS))
        .map(|sub| sub.layer(0).iter().map(|(_, c)| (*c) > 0).collect())
        .map(LetterImage)
        .map(|letter| ocr(letter).character)
        .collect()
}

fn main() {
    let (part1, part2) = day08();
    println!("part1 = {}", part1);
    println!("part2 = {}", part2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day08() {
        let (part1, part2) = day08();
        assert_eq!(part1, 1703);
        assert_eq!(part2, "HCGFE");
    }
}
