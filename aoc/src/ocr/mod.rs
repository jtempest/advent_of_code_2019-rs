use crate::geom::Dimensions;
use std::cmp::Ordering;
use std::fmt;

const LETTER_IMAGE_DATA: [(char, &str); 9] = [
    ('A', include_str!("letters/A.txt")),
    ('C', include_str!("letters/C.txt")),
    ('E', include_str!("letters/E.txt")),
    ('F', include_str!("letters/F.txt")),
    ('G', include_str!("letters/G.txt")),
    ('H', include_str!("letters/H.txt")),
    ('P', include_str!("letters/P.txt")),
    ('R', include_str!("letters/R.txt")),
    ('U', include_str!("letters/U.txt")),
];

pub const LETTER_IMAGE_DIMENSIONS: Dimensions = Dimensions {
    width: 4,
    height: 6,
};

pub struct LetterImage(pub Vec<bool>);

impl LetterImage {
    pub fn new(data: &[bool]) -> LetterImage {
        assert_eq!(data.len(), LETTER_IMAGE_DIMENSIONS.area());
        LetterImage(Vec::from(data))
    }

    fn score_similarity(&self, other: &LetterImage) -> f64 {
        let sum: f64 = self
            .0
            .iter()
            .copied()
            .zip(other.0.iter().copied())
            .map(|(a, b)| if a == b { 1.0 } else { 0.0 })
            .sum();
        sum as f64 / LETTER_IMAGE_DIMENSIONS.area() as f64
    }
}

impl From<&str> for LetterImage {
    fn from(s: &str) -> LetterImage {
        let data = s
            .lines()
            .flat_map(|line| line.chars().map(|c| !c.is_whitespace()))
            .collect::<Vec<_>>();
        assert!(data.len() == LETTER_IMAGE_DIMENSIONS.area());
        LetterImage(data)
    }
}

impl fmt::Display for LetterImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (pos, pixel) in LETTER_IMAGE_DIMENSIONS.iter().zip(self.0.iter().copied()) {
            if pos.x == 0 {
                writeln!(f)?;
            }
            let c = if pixel { '@' } else { ' ' };
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OcrResult {
    pub character: char,
    pub confidence: f64,
}

pub fn ocr(img: LetterImage) -> OcrResult {
    LETTER_IMAGE_DATA
        .iter()
        .copied()
        .map(|(c, s)| OcrResult {
            character: c,
            confidence: img.score_similarity(&LetterImage::from(s)),
        })
        .max_by(|a, b| {
            a.confidence
                .partial_cmp(&b.confidence)
                .unwrap_or(Ordering::Equal)
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ocr() {
        for (c, img_data) in LETTER_IMAGE_DATA.iter().copied() {
            let img = LetterImage::from(img_data);
            assert_eq!(ocr(img).character, c);
        }
    }
}
