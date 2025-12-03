use std::{ops::RangeInclusive, str::FromStr};

advent_of_code::solution!(2);

#[derive(Debug)]
struct ProductIdRange(RangeInclusive<usize>);

impl ProductIdRange {
    pub fn new(start: usize, end: usize) -> Self {
        ProductIdRange(start..=end)
    }

    /// Validates a product ID range by checking if each number is a palindrome.
    /// Returns an iterator over the valid product IDs.
    pub fn validate(&self, predicate: fn(&usize) -> bool) -> impl Iterator<Item = usize> {
        let range = self.0.clone().filter(predicate);
        range
    }

    pub fn palindrome(n: &usize) -> bool {
        let s = n.to_string();
        let h = s.len() / 2;

        &s[..h] == &s[h..]
    }

    pub fn repeating(n: &usize) -> bool {
        let s = n.to_string();
        let b = s.as_bytes();
        let len = b.len();
        let h = len / 2;

        (1..=h).any(|k| len % k == 0 && b.chunks(k).all(|c| c == &b[..k]))
    }

    pub fn palindrome_or_repeating(n: &usize) -> bool {
        let is_palindrome = ProductIdRange::palindrome(n);
        let is_repeating = ProductIdRange::repeating(n);

        if is_repeating {
            dbg!(&n);
        }

        is_palindrome || is_repeating
    }
}

impl FromStr for ProductIdRange {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once('-').ok_or("Invalid range format")?;
        let start = parts.0.parse().map_err(|_| "Invalid start value")?;
        let end = parts.1.parse().map_err(|_| "Invalid end value")?;
        Ok(ProductIdRange::new(start, end))
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .split(',')
            .map(ProductIdRange::from_str)
            .filter_map(Result::ok)
            .map(|p| p.validate(ProductIdRange::palindrome).sum::<usize>())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .split(',')
            .map(ProductIdRange::from_str)
            .filter_map(Result::ok)
            .map(|p| {
                p.validate(ProductIdRange::palindrome_or_repeating)
                    .sum::<usize>()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
