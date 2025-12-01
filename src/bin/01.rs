advent_of_code::solution!(1);

use std::str::FromStr;

#[derive(Debug)]
pub struct Dial {
    position: u8,
}

impl Dial {
    const NUM_POSITIONS: u8 = 100;

    pub fn new(position: u8) -> Self {
        Dial { position }
    }

    /// Rotate the dial by the given number of steps.
    /// Returns the number of revolutions of the dial.
    pub fn rotate(&mut self, steps: i16) -> u16 {
        let position = self.position;
        let raw = position as i16 + steps;
        self.position = raw.rem_euclid(Self::NUM_POSITIONS as i16) as u8;
        let mut revolutions: u16 = (raw / Self::NUM_POSITIONS as i16).unsigned_abs();

        if position != 0 && raw <= 0 {
            revolutions += 1;
        }

        revolutions
    }
}

#[derive(Debug)]
enum Direction {
    Left(i16),
    Right(i16),
}

impl From<Direction> for i16 {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Left(n) => -n,
            Direction::Right(n) => n,
        }
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sign = &s[..1];
        let number = s[1..].parse().map_err(|_| "Invalid number".to_string())?;

        match sign {
            "R" => Ok(Direction::Right(number)),
            "L" => Ok(Direction::Left(number)),
            _ => Err("Invalid direction".to_string()),
        }
    }
}

pub fn part_one(input: &str) -> Option<u16> {
    Some(
        input
            .lines()
            .fold((Dial::new(50), 0), |(mut dial, mut times_at_zero), line| {
                if let Ok(steps) = Direction::from_str(line).map(i16::from) {
                    dial.rotate(steps);
                    if dial.position == 0 {
                        times_at_zero += 1;
                    }
                }
                (dial, times_at_zero)
            })
            .1,
    )
}

pub fn part_two(input: &str) -> Option<u16> {
    Some(
        input
            .lines()
            .fold((Dial::new(50), 0), |(mut dial, mut revolutions), line| {
                if let Ok(steps) = Direction::from_str(line).map(i16::from) {
                    revolutions += dial.rotate(steps);
                }
                (dial, revolutions)
            })
            .1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
