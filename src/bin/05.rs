use itertools::Itertools;

advent_of_code::solution!(5);

fn parse(input: &str) -> (Box<[(usize, usize)]>, Box<[usize]>) {
    let (ranges, numbers) = input
        .replace("\r\n", "\n")
        .split_once("\n\n")
        .map(|(ranges, numbers)| {
            let ranges = ranges
                .lines()
                .map(|line| {
                    line.split_once('-')
                        .map(|(start, end)| {
                            (
                                start.parse::<usize>().unwrap(),
                                end.parse::<usize>().unwrap(),
                            )
                        })
                        .unwrap()
                })
                .collect::<Box<_>>();
            let numbers = numbers
                .lines()
                .map(|line| line.parse::<usize>().unwrap())
                .collect::<Box<_>>();
            (ranges, numbers)
        })
        .unwrap();
    (ranges, numbers)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (ranges, numbers) = parse(input);
    let total = numbers
        .iter()
        .filter(|&n| ranges.iter().any(|(start, end)| n > start && n <= end))
        .count();
    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (ranges, ..) = parse(input);
    let intervals = ranges
        .iter()
        .sorted_unstable_by_key(|r| r.0)
        .collect::<Box<_>>();

    let mut merged: Vec<(usize, usize)> = Vec::with_capacity(ranges.len());

    for (start, end) in intervals {
        if let Some((.., me)) = merged.last_mut() {
            // Overlap or touching
            if *start <= *me + 1 {
                *me = (*me).max(*end);
                continue;
            }
        }
        merged.push((*start, *end))
    }

    Some(merged.iter().map(|(s, e)| e - s + 1).sum())
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
        assert_eq!(result, Some(14));
    }
}
