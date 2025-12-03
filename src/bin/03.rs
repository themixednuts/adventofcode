use std::{
    iter::Sum,
    ops::{Add, AddAssign},
    str::FromStr,
};

advent_of_code::solution!(3);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct BatteryBank {
    batteries: Box<[Battery]>,
}

impl BatteryBank {
    fn new(batteries: impl Into<Box<[Battery]>>) -> Self {
        BatteryBank {
            batteries: batteries.into(),
        }
    }

    fn best_of(&self, n: usize) -> Option<Battery> {
        let len = self.batteries.len();

        if n > len || len < 2 {
            return None;
        }

        let mut result = Battery::new(0);
        let mut start = 0;

        for remaining in (1..=n).rev() {
            let end = len - remaining + 1;
            let slice = &self.batteries[start..end];

            let (local_idx, &max) = slice
                .iter()
                .enumerate()
                .reduce(|(i, a), (j, b)| if b > a { (j, b) } else { (i, a) })?;

            result += &max;
            start += local_idx + 1;
        }

        Some(result)
    }
}

impl From<Box<[Battery]>> for BatteryBank {
    fn from(batteries: Box<[Battery]>) -> Self {
        BatteryBank::new(batteries)
    }
}

impl FromStr for BatteryBank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let batteries = s
            .chars()
            .map(Battery::try_from)
            .collect::<Result<Box<_>, _>>()?;
        Ok(BatteryBank::new(batteries))
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
struct Battery {
    joltage: usize,
}

impl Battery {
    fn new(joltage: usize) -> Self {
        Battery { joltage }
    }
}

impl TryFrom<char> for Battery {
    type Error = String;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        Ok(Battery::new(
            char.to_digit(10).ok_or("Invalid character")? as usize
        ))
    }
}

impl Add for Battery {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Battery::new(self.joltage * 10 + other.joltage)
    }
}

impl AddAssign<&Battery> for Battery {
    fn add_assign(&mut self, other: &Battery) {
        self.joltage = self.joltage * 10 + other.joltage;
    }
}

impl Sum for Battery {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Battery::new(0), |acc, b| {
            Battery::new(acc.joltage + b.joltage)
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter_map(|line| BatteryBank::from_str(line).ok())
            .filter_map(|bank| bank.best_of(2))
            .sum::<Battery>()
            .joltage,
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter_map(|line| BatteryBank::from_str(line).ok())
            .filter_map(|bank| bank.best_of(12))
            .sum::<Battery>()
            .joltage,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
