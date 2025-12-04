advent_of_code::solution!(4);

const DELTAS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug)]
struct Cell {
    col: u8,
    row: u8,
}

#[derive(Debug)]
struct PaperRoll {
    position: Cell,
}

impl PaperRoll {
    fn new(col: u8, row: u8) -> Self {
        PaperRoll {
            position: Cell { col, row },
        }
    }

    fn is_removeable(&self, slice: &Box<[Box<[Option<PaperRoll>]>]>) -> bool {
        let row = self.position.row as isize;
        let col = self.position.col as isize;

        let surrounding = DELTAS
            .iter()
            .filter_map(|(dr, dc)| {
                let r = row + dr;
                let c = col + dc;

                if r < 0 || c < 0 {
                    return None;
                }
                slice.get(r as usize).and_then(|row| row.get(c as usize))
            })
            .filter_map(|p| p.as_ref());

        surrounding.count() < 4
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let slice: Box<[Box<[Option<PaperRoll>]>]> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(col, c)| match c {
                    b'@' => Some(PaperRoll::new(col as u8, row as u8)),
                    _ => None,
                })
                .collect()
        })
        .collect();

    Some(
        slice
            .iter()
            .map(|row| {
                row.iter()
                    .filter_map(|p| p.as_ref().map(|p| p.is_removeable(&slice)))
            })
            .flat_map(|p| p)
            .filter(|&p| p)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut slice: Box<[Box<[Option<PaperRoll>]>]> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(col, c)| match c {
                    b'@' => Some(PaperRoll::new(col as u8, row as u8)),
                    _ => None,
                })
                .collect()
        })
        .collect();

    let mut total = 0;

    loop {
        let mut removed = 0;
        for row in 0..slice.len() {
            for col in 0..slice[0].len() {
                if let Some(paper_roll) = &slice[row][col]
                    && paper_roll.is_removeable(&slice)
                {
                    slice[row][col] = None;
                    removed += 1;
                }
            }
        }
        if removed == 0 {
            break;
        }

        total += removed;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
