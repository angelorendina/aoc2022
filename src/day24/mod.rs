use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Cell {
    Wall,
    Blizzards(Vec<(isize, isize)>),
    Ground,
}

struct Atlas {
    by_time: BTreeMap<usize, BTreeMap<(isize, isize), Cell>>,
    max_row: isize,
    max_column: isize,
}
impl Atlas {
    fn get(&mut self, time: usize) -> &BTreeMap<(isize, isize), Cell> {
        let (&last_time, last_map) = self.by_time.last_key_value().unwrap();
        if last_time < time {
            let mut new_map = last_map
                .iter()
                .map(|(position, cell)| {
                    let new_cell = match cell {
                        Cell::Wall => Cell::Wall,
                        Cell::Blizzards(_) => Cell::Ground,
                        Cell::Ground => Cell::Ground,
                    };
                    (*position, new_cell)
                })
                .collect::<BTreeMap<_, _>>();
            for (coords, cell) in last_map {
                if let Cell::Blizzards(blizzards) = cell {
                    for blizzard in blizzards {
                        let mut new_coords = (coords.0 + blizzard.0, coords.1 + blizzard.1);
                        if new_coords.0 == 0 {
                            new_coords.0 = self.max_row - 1;
                        }
                        if new_coords.0 == self.max_row {
                            new_coords.0 = 1;
                        }
                        if new_coords.1 == 0 {
                            new_coords.1 = self.max_column - 1;
                        }
                        if new_coords.1 == self.max_column {
                            new_coords.1 = 1;
                        }
                        match new_map.get_mut(&new_coords) {
                            None => unreachable!(),
                            Some(new_cell) => match new_cell {
                                Cell::Wall => unreachable!(),
                                Cell::Blizzards(new_blizzards) => {
                                    new_blizzards.push(*blizzard);
                                }
                                Cell::Ground => {
                                    *new_cell = Cell::Blizzards(vec![*blizzard]);
                                }
                            },
                        }
                    }
                }
            }
            self.by_time.insert(last_time + 1, new_map);
            self.get(time)
        } else {
            self.by_time.get(&time).unwrap()
        }
    }
}

pub fn star_one() -> usize {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut map = BTreeMap::<(isize, isize), Cell>::new();
    for (r, line) in values.lines().enumerate() {
        for (c, x) in line.chars().enumerate() {
            map.insert(
                (r as isize, c as isize),
                match x {
                    '#' => Cell::Wall,
                    '.' => Cell::Ground,
                    '>' => Cell::Blizzards(vec![(0, 1)]),
                    '<' => Cell::Blizzards(vec![(0, -1)]),
                    '^' => Cell::Blizzards(vec![(-1, 0)]),
                    'v' => Cell::Blizzards(vec![(1, 0)]),
                    _ => unreachable!(),
                },
            );
        }
    }

    let (&start, _) = map
        .iter()
        .find(|(_, cell)| matches!(cell, Cell::Ground))
        .unwrap();
    let (&end, _) = map
        .iter()
        .rev()
        .find(|(_, cell)| matches!(cell, Cell::Ground))
        .unwrap();
    let (&(max_row, max_column), _) = map.iter().last().unwrap();

    let mut atlas = Atlas {
        by_time: BTreeMap::<usize, BTreeMap<(isize, isize), Cell>>::from([(0, map)]),
        max_row,
        max_column,
    };

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct State {
        position: (isize, isize),
        time: usize,
        distance: usize,
    }

    impl std::cmp::PartialOrd for State {
        fn partial_cmp(&self, rhs: &State) -> Option<std::cmp::Ordering> {
            Some(self.distance.cmp(&rhs.distance).reverse())
        }
    }

    impl std::cmp::Ord for State {
        fn cmp(&self, rhs: &State) -> std::cmp::Ordering {
            unsafe { self.partial_cmp(rhs).unwrap_unchecked() }
        }
    }

    let mut visited_states = BTreeSet::<((isize, isize), usize)>::new();

    let mut min_steps = usize::MAX;
    let mut queue = BinaryHeap::<State>::new();
    queue.push(State {
        position: start,
        time: 0,
        distance: isize::abs_diff(start.0, end.0) + isize::abs_diff(start.1, end.1),
    });
    while let Some(state) = queue.pop() {
        let State { position, time, .. } = state;
        if time >= min_steps {
            continue;
        }
        if !visited_states.insert((position, time)) {
            continue;
        }
        let new_map = atlas.get(time + 1);
        for mov in [(1isize, 0isize), (0, 1), (0, -1), (0, 0), (-1, 0)] {
            let new_position = (position.0 + mov.0, position.1 + mov.1);
            if matches!(new_map.get(&new_position), Some(Cell::Ground)) {
                let new_distance =
                    isize::abs_diff(new_position.0, end.0) + isize::abs_diff(new_position.1, end.1);
                if new_distance == 0 {
                    if time + 1 < min_steps {
                        min_steps = time + 1;
                    }
                    continue;
                }
                queue.push(State {
                    position: new_position,
                    time: time + 1,
                    distance: new_distance,
                });
            }
        }
    }
    min_steps
}

pub fn star_two() -> usize {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut map = BTreeMap::<(isize, isize), Cell>::new();
    for (r, line) in values.lines().enumerate() {
        for (c, x) in line.chars().enumerate() {
            map.insert(
                (r as isize, c as isize),
                match x {
                    '#' => Cell::Wall,
                    '.' => Cell::Ground,
                    '>' => Cell::Blizzards(vec![(0, 1)]),
                    '<' => Cell::Blizzards(vec![(0, -1)]),
                    '^' => Cell::Blizzards(vec![(-1, 0)]),
                    'v' => Cell::Blizzards(vec![(1, 0)]),
                    _ => unreachable!(),
                },
            );
        }
    }

    let (&start, _) = map
        .iter()
        .find(|(_, cell)| matches!(cell, Cell::Ground))
        .unwrap();
    let (&end, _) = map
        .iter()
        .rev()
        .find(|(_, cell)| matches!(cell, Cell::Ground))
        .unwrap();
    let (&(max_row, max_column), _) = map.iter().last().unwrap();

    let mut atlas = Atlas {
        by_time: BTreeMap::<usize, BTreeMap<(isize, isize), Cell>>::from([(0, map)]),
        max_row,
        max_column,
    };

    #[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
    enum Target {
        A((isize, isize)),
        B((isize, isize)),
        C((isize, isize)),
    }

    impl Target {
        fn coords(&self) -> (isize, isize) {
            match self {
                Target::A(coords) | Target::B(coords) | Target::C(coords) => *coords,
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq, Copy, Hash)]
    struct State {
        position: (isize, isize),
        time: usize,
        target: Target,
    }

    impl std::cmp::PartialOrd for State {
        fn partial_cmp(&self, rhs: &State) -> Option<std::cmp::Ordering> {
            Some(match (&self.target, &rhs.target) {
                (Target::A(_), Target::B(_)) => std::cmp::Ordering::Less,
                (Target::A(_), Target::C(_)) => std::cmp::Ordering::Less,
                (Target::B(_), Target::A(_)) => std::cmp::Ordering::Greater,
                (Target::B(_), Target::C(_)) => std::cmp::Ordering::Less,
                (Target::C(_), Target::A(_)) => std::cmp::Ordering::Greater,
                (Target::C(_), Target::B(_)) => std::cmp::Ordering::Greater,
                (Target::A(t1), Target::A(t2))
                | (Target::B(t1), Target::B(t2))
                | (Target::C(t1), Target::C(t2)) => {
                    let d1 = isize::abs_diff(self.position.0, t1.0)
                        + isize::abs_diff(self.position.1, t1.1);
                    let d2 = isize::abs_diff(rhs.position.0, t2.0)
                        + isize::abs_diff(rhs.position.1, t2.1);
                    d1.cmp(&d2).reverse()
                }
            })
        }
    }

    impl std::cmp::Ord for State {
        fn cmp(&self, rhs: &State) -> std::cmp::Ordering {
            unsafe { self.partial_cmp(rhs).unwrap_unchecked() }
        }
    }

    let mut visited_states = HashSet::<State>::new();

    let mut min_steps = usize::MAX;
    let mut queue = BinaryHeap::<State>::new();
    queue.push(State {
        position: start,
        time: 0,
        target: Target::A(end),
    });
    while let Some(state) = queue.pop() {
        if !visited_states.insert(state) {
            continue;
        }
        let State {
            position,
            time,
            target,
        } = state;
        if time >= min_steps {
            continue;
        }
        let new_map = atlas.get(time + 1);
        for mov in [(1isize, 0isize), (0, 1), (0, -1), (0, 0), (-1, 0)] {
            let new_position = (position.0 + mov.0, position.1 + mov.1);
            if matches!(new_map.get(&new_position), Some(Cell::Ground)) {
                let mut new_target = target;
                if new_position == target.coords() {
                    new_target = match target {
                        Target::A(_) => Target::B(start),
                        Target::B(_) => Target::C(end),
                        Target::C(_) => {
                            if time + 1 < min_steps {
                                min_steps = time + 1;
                            }
                            continue;
                        }
                    }
                }
                let new_state = State {
                    position: new_position,
                    time: time + 1,
                    target: new_target,
                };
                queue.push(new_state);
            }
        }
    }
    min_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 18);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 54);
    }
}
