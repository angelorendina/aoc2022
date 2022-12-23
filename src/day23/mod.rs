use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn offset(&self, (r, c): (isize, isize)) -> [(isize, isize); 3] {
        match self {
            Direction::North => [(r - 1, c - 1), (r - 1, c), (r - 1, c + 1)],
            Direction::South => [(r + 1, c - 1), (r + 1, c), (r + 1, c + 1)],
            Direction::West => [(r - 1, c - 1), (r, c - 1), (r + 1, c - 1)],
            Direction::East => [(r - 1, c + 1), (r, c + 1), (r + 1, c + 1)],
        }
    }

    fn shift(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

pub fn star_one() -> isize {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut map = BTreeSet::<(isize, isize)>::new();
    for (row, line) in values.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert((row as isize, column as isize));
            }
        }
    }

    let mut first_direction = Direction::North;
    for _ in 0..10 {
        let mut proposals = BTreeMap::<(isize, isize), Vec<(isize, isize)>>::new();
        for &elf in &map {
            if !map.contains(&(elf.0 - 1, elf.1 - 1))
                && !map.contains(&(elf.0 - 1, elf.1))
                && !map.contains(&(elf.0 - 1, elf.1 + 1))
                && !map.contains(&(elf.0, elf.1 - 1))
                && !map.contains(&(elf.0, elf.1 + 1))
                && !map.contains(&(elf.0 + 1, elf.1 - 1))
                && !map.contains(&(elf.0 + 1, elf.1))
                && !map.contains(&(elf.0 + 1, elf.1 + 1))
            {
                continue;
            }

            let mut proposed = false;
            for _ in 0..4 {
                let neighbours = first_direction.offset(elf);
                if !proposed && !neighbours.iter().any(|nei| map.contains(nei)) {
                    proposals.entry(neighbours[1]).or_default().push(elf);
                    proposed = true;
                }
                first_direction = first_direction.shift();
            }
        }
        first_direction = first_direction.shift();

        for (target, candidates) in proposals {
            if let &[elf] = &candidates.as_slice() {
                map.remove(elf);
                map.insert(target);
            }
        }
    }

    let mut top_left = (isize::MAX, isize::MAX);
    let mut bottom_right = (isize::MIN, isize::MIN);

    for elf in &map {
        top_left = (isize::min(top_left.0, elf.0), isize::min(top_left.1, elf.1));
        bottom_right = (
            isize::max(bottom_right.0, elf.0),
            isize::max(bottom_right.1, elf.1),
        );
    }

    let height = bottom_right.0 - top_left.0 + 1;
    let width = bottom_right.1 - top_left.1 + 1;

    width * height - map.len() as isize
}

pub fn star_two() -> isize {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut map = BTreeSet::<(isize, isize)>::new();
    for (row, line) in values.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert((row as isize, column as isize));
            }
        }
    }

    let mut first_direction = Direction::North;
    for first_unmoved_round in 1.. {
        let mut proposals = BTreeMap::<(isize, isize), Vec<(isize, isize)>>::new();
        for &elf in &map {
            if !map.contains(&(elf.0 - 1, elf.1 - 1))
                && !map.contains(&(elf.0 - 1, elf.1))
                && !map.contains(&(elf.0 - 1, elf.1 + 1))
                && !map.contains(&(elf.0, elf.1 - 1))
                && !map.contains(&(elf.0, elf.1 + 1))
                && !map.contains(&(elf.0 + 1, elf.1 - 1))
                && !map.contains(&(elf.0 + 1, elf.1))
                && !map.contains(&(elf.0 + 1, elf.1 + 1))
            {
                continue;
            }

            let mut proposed = false;
            for _ in 0..4 {
                let neighbours = first_direction.offset(elf);
                if !proposed && !neighbours.iter().any(|nei| map.contains(nei)) {
                    proposals.entry(neighbours[1]).or_default().push(elf);
                    proposed = true;
                }
                first_direction = first_direction.shift();
            }
        }
        first_direction = first_direction.shift();

        let mut moved = false;
        for (target, candidates) in proposals {
            if let &[elf] = &candidates.as_slice() {
                map.remove(elf);
                map.insert(target);
                moved = true;
            }
        }
        if !moved {
            return first_unmoved_round;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 110);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 20);
    }
}
