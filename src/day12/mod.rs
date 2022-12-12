use std::collections::BTreeMap;
use std::collections::VecDeque;

struct GridMap<T> {
    data: BTreeMap<(usize, usize), T>,
    rows: usize,
    columns: usize,
}

impl<T> GridMap<T> {
    fn new() -> Self {
        Self {
            data: BTreeMap::new(),
            rows: 0,
            columns: 0,
        }
    }

    fn get(&self, row: usize, column: usize) -> Option<&T> {
        self.data.get(&(row, column))
    }

    fn set(&mut self, row: usize, column: usize, value: T) {
        self.data.insert((row, column), value);
        self.rows = usize::max(self.rows, row + 1);
        self.columns = usize::max(self.columns, column + 1);
    }
}

pub fn star_one() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut map = GridMap::<u8>::new();
    let mut start = (0usize, 0usize, 0u64);
    let mut end = (0usize, 0usize);

    for (row, line) in values.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    map.set(row, column, 0);
                    start = (row, column, 0u64);
                }
                'E' => {
                    map.set(row, column, 25);
                    end = (row, column);
                }
                '\n' => {}
                _ => map.set(row, column, c as u8 - 97),
            }
        }
    }

    let mut paths = GridMap::<u64>::new();
    paths.set(start.0, start.1, 0);

    let mut queue = VecDeque::from([start]);
    while let Some((row, column, steps)) = queue.pop_front() {
        let mut neighbours = vec![];
        if row > 0 {
            neighbours.push((row - 1, column));
        }
        if row + 1 < map.rows {
            neighbours.push((row + 1, column));
        }
        if column > 0 {
            neighbours.push((row, column - 1));
        }
        if column + 1 < map.columns {
            neighbours.push((row, column + 1));
        }
        neighbours.retain(|(to_row, to_column)| {
            let from_height = *map.get(row, column).unwrap();
            let to_height = *map.get(*to_row, *to_column).unwrap();
            if to_height <= from_height + 1 {
                if let Some(&to_steps) = paths.get(*to_row, *to_column) {
                    steps + 1 < to_steps
                } else {
                    true
                }
            } else {
                false
            }
        });
        for (to_row, to_column) in neighbours {
            paths.set(to_row, to_column, steps + 1);
            queue.push_back((to_row, to_column, steps + 1));
        }
    }

    *paths.get(end.0, end.1).unwrap()
}

pub fn star_two() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut map = GridMap::<u8>::new();
    let mut end = (0usize, 0usize);

    for (row, line) in values.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    map.set(row, column, 0);
                }
                'E' => {
                    map.set(row, column, 25);
                    end = (row, column);
                }
                '\n' => {}
                _ => map.set(row, column, c as u8 - 97),
            }
        }
    }

    let mut paths = GridMap::<u64>::new();

    let mut queue = VecDeque::from([(end.0, end.1, 0)]);
    while let Some((row, column, steps)) = queue.pop_front() {
        let mut neighbours = vec![];
        if row > 0 {
            neighbours.push((row - 1, column));
        }
        if row + 1 < map.rows {
            neighbours.push((row + 1, column));
        }
        if column > 0 {
            neighbours.push((row, column - 1));
        }
        if column + 1 < map.columns {
            neighbours.push((row, column + 1));
        }
        neighbours.retain(|(to_row, to_column)| {
            let from_height = *map.get(row, column).unwrap();
            let to_height = *map.get(*to_row, *to_column).unwrap();
            if to_height + 1 >= from_height {
                if let Some(&to_steps) = paths.get(*to_row, *to_column) {
                    steps + 1 < to_steps
                } else {
                    true
                }
            } else {
                false
            }
        });
        for (to_row, to_column) in neighbours {
            paths.set(to_row, to_column, steps + 1);
            queue.push_back((to_row, to_column, steps + 1));
        }
    }

    *map.data
        .into_iter()
        .filter_map(|((row, column), height)| {
            if height == 0 {
                paths.get(row, column)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 31);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 29);
    }
}
