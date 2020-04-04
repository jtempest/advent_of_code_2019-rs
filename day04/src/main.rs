//! Solution to Advent of Code 2019 [Day 4](https://adventofcode.com/2019/day/4).

#[derive(PartialEq)]
struct Password([u8; 6]);

impl Password {
    fn new(num: u32) -> Password {
        let mut p = Password([0; 6]);
        let digits = num
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as u8)
            .collect::<Vec<_>>();
        for (n, v) in digits.into_iter().enumerate() {
            p.0[n] = v;
        }
        p
    }

    fn is_valid(&self) -> bool {
        let p = &self.0;
        (
            // two adjacent equal digits
            p[0] == p[1] || p[1] == p[2] || p[2] == p[3] || p[3] == p[4] || p[4] == p[5]
        ) && (
            // increasing digits
            p[0] <= p[1] && p[1] <= p[2] && p[2] <= p[3] && p[3] <= p[4] && p[4] <= p[5]
        )
    }

    #[rustfmt::skip]
    fn is_valid_part2(&self) -> bool {
        let p = &self.0;

           (/* no digit */  p[0] == p[1] && p[1] != p[2])
        || (p[0] != p[1] && p[1] == p[2] && p[2] != p[3])
        || (p[1] != p[2] && p[2] == p[3] && p[3] != p[4])
        || (p[2] != p[3] && p[3] == p[4] && p[4] != p[5])
        || (p[3] != p[4] && p[4] == p[5]  /* no digit */)
    }

    fn increment(&mut self) {
        self.increment_digit(5);
    }

    fn increment_digit(&mut self, digit: usize) {
        if self.0[digit] == 9 {
            self.0[digit] = 0;
            self.increment_digit(digit - 1);
        } else {
            self.0[digit] += 1;
        }
    }
}

fn day04() -> (usize, usize) {
    let mut p = Password::new(178_416);
    let mut part1 = 0;
    let mut part2 = 0;
    while p != Password::new(676_461) {
        p.increment();
        if p.is_valid() {
            part1 += 1;
            if p.is_valid_part2() {
                part2 += 1;
            }
        }
    }
    (part1, part2)
}

fn main() {
    let (p1, p2) = day04();
    println!("part1 = {}", p1);
    println!("part2 = {}", p2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_password_validity() {
        assert!(Password::new(111_111).is_valid());
        assert!(!Password::new(223_450).is_valid());
        assert!(!Password::new(123_789).is_valid());

        assert!(Password::new(112_233).is_valid_part2());
        assert!(!Password::new(123_444).is_valid_part2());
        assert!(Password::new(111_122).is_valid_part2());
    }

    #[test]
    fn test_day04() {
        let (p1, p2) = day04();
        assert_eq!(p1, 1650);
        assert_eq!(p2, 1129);
    }
}
