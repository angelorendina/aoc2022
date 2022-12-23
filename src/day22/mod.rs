use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
struct V {
    x: isize,
    y: isize,
    z: isize,
}

impl std::ops::Mul<V> for V {
    type Output = isize;

    fn mul(self, rhs: V) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl V {
    const fn x() -> Self {
        V {
            x: 1,
            ..Default::default()
        }
    }

    const fn y() -> Self {
        V {
            y: 1,
            ..Default::default()
        }
    }

    const fn z() -> Self {
        V {
            z: 1,
            ..Default::default()
        }
    }

    const fn xneg() -> Self {
        V {
            x: -1,
            ..Default::default()
        }
    }

    const fn yneg() -> Self {
        V {
            y: -1,
            ..Default::default()
        }
    }

    const fn zneg() -> Self {
        V {
            z: -1,
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
struct M(V, V, V);

impl std::ops::Mul<V> for M {
    type Output = V;

    fn mul(self, rhs: V) -> Self::Output {
        let M(r1, r2, r3) = self.transpose();
        V {
            x: r1 * rhs,
            y: r2 * rhs,
            z: r3 * rhs,
        }
    }
}

impl std::ops::Mul<M> for M {
    type Output = M;

    fn mul(self, rhs: M) -> Self::Output {
        let M(r1, r2, r3) = self.transpose();
        let M(c1, c2, c3) = rhs;
        M(
            V {
                x: r1 * c1,
                y: r1 * c2,
                z: r1 * c3,
            },
            V {
                x: r2 * c1,
                y: r2 * c2,
                z: r2 * c3,
            },
            V {
                x: r3 * c1,
                y: r3 * c2,
                z: r3 * c3,
            },
        )
    }
}

impl M {
    const fn identity() -> Self {
        M(V::x(), V::y(), V::z())
    }

    const fn x() -> Self {
        M(V::x(), V::z(), V::yneg())
    }

    const fn y() -> Self {
        M(V::zneg(), V::y(), V::x())
    }

    const fn z() -> Self {
        M(V::y(), V::xneg(), V::z())
    }

    const fn xneg() -> Self {
        M(V::x(), V::zneg(), V::y())
    }

    const fn yneg() -> Self {
        M(V::z(), V::y(), V::xneg())
    }

    const fn zneg() -> Self {
        M(V::yneg(), V::x(), V::z())
    }

    const fn transpose(self) -> Self {
        M(
            V {
                x: self.0.x,
                y: self.1.x,
                z: self.2.x,
            },
            V {
                x: self.0.y,
                y: self.1.y,
                z: self.2.y,
            },
            V {
                x: self.0.z,
                y: self.1.z,
                z: self.2.z,
            },
        )
    }
}

#[derive(Clone, Copy)]
enum Cell {
    Floor,
    Wall,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Face {
    Top,
    Bottom,
    Far,
    Near,
    Right,
    Left,
}

pub fn star_one() -> isize {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut map = BTreeMap::new();
    let lines = values.lines();
    let mut lines = lines.rev();
    let code = lines.next().unwrap();
    let lines = lines.rev();
    let mut start = (false, 0, 0);
    let mut max_row = 0;
    let mut max_column = 0;
    for (row, line) in lines.enumerate() {
        for (column, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    if !start.0 {
                        start = (true, row as isize, column as isize);
                    }
                    map.insert((row as isize, column as isize), Cell::Floor);
                    if row as isize > max_row {
                        max_row = row as isize;
                    }
                    if column as isize > max_column {
                        max_column = column as isize;
                    }
                }
                '#' => {
                    map.insert((row as isize, column as isize), Cell::Wall);
                    if row as isize > max_row {
                        max_row = row as isize;
                    }
                    if column as isize > max_column {
                        max_column = column as isize;
                    }
                }
                _ => {}
            }
        }
    }

    let mut position = (start.1, start.2);
    let mut direction = Direction::Right;

    let mut code_index = 0;
    loop {
        let mut rotation = None;
        let mut steps = 0;

        let mut token_end = code_index + 1;
        while token_end < code.len() {
            let rot = &code[token_end..=token_end];
            if rot == "R" || rot == "L" {
                rotation = Some(rot);
                break;
            }
            token_end += 1;
        }
        if token_end == code.len() {
            steps = code[code_index..=token_end - 1].parse::<usize>().unwrap();
        } else {
            steps = code[code_index..token_end].parse::<usize>().unwrap();
        }
        code_index = token_end + 1;

        'walk: for _ in 0..steps {
            match direction {
                Direction::Left => match map.get(&(position.0, position.1 - 1)) {
                    Some(Cell::Floor) => {
                        position = (position.0, position.1 - 1);
                    }
                    Some(Cell::Wall) => {
                        break 'walk;
                    }
                    None => {
                        'wrap: for c in (isize::MIN..=max_column).rev() {
                            match map.get(&(position.0, c)) {
                                Some(Cell::Floor) => {
                                    position = (position.0, c);
                                    break 'wrap;
                                }
                                Some(Cell::Wall) => {
                                    break 'walk;
                                }
                                None => {}
                            }
                        }
                    }
                },
                Direction::Right => match map.get(&(position.0, position.1 + 1)) {
                    Some(Cell::Floor) => {
                        position = (position.0, position.1 + 1);
                    }
                    Some(Cell::Wall) => {
                        break 'walk;
                    }
                    None => {
                        'wrap: for c in 0.. {
                            match map.get(&(position.0, c)) {
                                Some(Cell::Floor) => {
                                    position = (position.0, c);
                                    break 'wrap;
                                }
                                Some(Cell::Wall) => {
                                    break 'walk;
                                }
                                None => {}
                            }
                        }
                    }
                },
                Direction::Up => match map.get(&(position.0 - 1, position.1)) {
                    Some(Cell::Floor) => {
                        position = (position.0 - 1, position.1);
                    }
                    Some(Cell::Wall) => {
                        break 'walk;
                    }
                    None => {
                        'wrap: for r in (isize::MIN..=max_row).rev() {
                            match map.get(&(r, position.1)) {
                                Some(Cell::Floor) => {
                                    position = (r, position.1);
                                    break 'wrap;
                                }
                                Some(Cell::Wall) => {
                                    break 'walk;
                                }
                                None => {}
                            }
                        }
                    }
                },
                Direction::Down => match map.get(&(position.0 + 1, position.1)) {
                    Some(Cell::Floor) => {
                        position = (position.0 + 1, position.1);
                    }
                    Some(Cell::Wall) => {
                        break 'walk;
                    }
                    None => {
                        'wrap: for r in 0.. {
                            match map.get(&(r, position.1)) {
                                Some(Cell::Floor) => {
                                    position = (r, position.1);
                                    break 'wrap;
                                }
                                Some(Cell::Wall) => {
                                    break 'walk;
                                }
                                None => {}
                            }
                        }
                    }
                },
            }
        }

        if let Some(rotation) = rotation {
            direction = match (rotation, direction) {
                ("L", Direction::Right) => Direction::Up,
                ("L", Direction::Left) => Direction::Down,
                ("L", Direction::Up) => Direction::Left,
                ("L", Direction::Down) => Direction::Right,
                ("R", Direction::Right) => Direction::Down,
                ("R", Direction::Left) => Direction::Up,
                ("R", Direction::Up) => Direction::Right,
                ("R", Direction::Down) => Direction::Left,
                _ => unreachable!(),
            }
        } else {
            break;
        }
    }

    1000 * (position.0 + 1)
        + 4 * (position.1 + 1)
        + match direction {
            Direction::Left => 2,
            Direction::Right => 0,
            Direction::Up => 3,
            Direction::Down => 1,
        }
}

pub fn star_two() -> isize {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(test)]
    let face_size = 4;
    #[cfg(not(test))]
    let values = include_str!("input.txt");
    #[cfg(not(test))]
    let face_size = 50;

    let mut map = BTreeMap::new();
    let lines = values.lines();
    let mut lines = lines.rev();
    let code = lines.next().unwrap();
    let lines = lines.rev();
    let mut start = (false, 0, 0);
    let mut max_row = 0;
    let mut max_column = 0;
    for (row, line) in lines.enumerate() {
        for (column, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    if !start.0 {
                        start = (true, row as isize, column as isize);
                    }
                    map.insert((row as isize, column as isize), Cell::Floor);
                    if row as isize > max_row {
                        max_row = row as isize;
                    }
                    if column as isize > max_column {
                        max_column = column as isize;
                    }
                }
                '#' => {
                    map.insert((row as isize, column as isize), Cell::Wall);
                    if row as isize > max_row {
                        max_row = row as isize;
                    }
                    if column as isize > max_column {
                        max_column = column as isize;
                    }
                }
                _ => {}
            }
        }
    }
    let faces_across = (max_column + 1) / face_size;
    let faces_down = (max_row + 1) / face_size;

    let mut corners = BTreeSet::<(isize, isize)>::new();
    for r in 0..faces_down {
        for c in 0..faces_across {
            let coords = (face_size * r, face_size * c);
            if map.get(&coords).is_some() {
                corners.insert(coords);
            }
        }
    }
    let mut cube = BTreeMap::<(isize, isize), (V, M)>::new();
    let first_face = corners.iter().next().unwrap().clone();
    cube.insert(first_face, (V::zneg(), M::identity()));
    while cube.len() < 6 {
        let mut x = None;
        for (&coords, &(side, m)) in &cube {
            let neighbour = (coords.0, coords.1 + face_size);
            if corners.contains(&neighbour) && !cube.contains_key(&neighbour) {
                // TODO!
                x = Some((neighbour, nei));
                break;
            }
            let neighbour = (coords.0, coords.1 - face_size);
            if corners.contains(&neighbour) && !cube.contains_key(&neighbour) {
                let nei = match side {
                    Face::Top => todo!(),
                    Face::Bottom => todo!(),
                    Face::Far => todo!(),
                    Face::Near => (Face::Left, GM::from_rotation_z(TAU)),
                    Face::Right => (Face::Near, GM::from_rotation_z(TAU)),
                    Face::Left => (Face::Far, GM::from_rotation_z(TAU)),
                };
                x = Some((neighbour, nei));
                break;
            }
            let neighbour = (coords.0 + face_size, coords.1);
            if corners.contains(&neighbour) && !cube.contains_key(&neighbour) {
                let nei = match side {
                    Face::Top => (Face::Near, GM::from_rotation_z(TAU)),
                    Face::Bottom => todo!(),
                    Face::Far => todo!(),
                    Face::Near => (Face::Right, GM::from_rotation_z(TAU)),
                    Face::Right => todo!(),
                    Face::Left => todo!(),
                };
                x = Some((neighbour, nei));
                break;
            }
            let neighbour = (coords.0 - face_size, coords.1);
            if corners.contains(&neighbour) && !cube.contains_key(&neighbour) {
                let nei = match side {
                    Face::Top => todo!(),
                    Face::Bottom => todo!(),
                    Face::Far => todo!(),
                    Face::Near => todo!(),
                    Face::Right => todo!(),
                    Face::Left => todo!(),
                };
                x = Some((neighbour, nei));
                break;
            }
        }
        let x = x.unwrap();
        cube.insert(x.0, x.1);
    }
    println!("{:#?}", cube);

    let mut position = (start.1, start.2);
    let mut direction = Direction::Right;

    let mut code_index = 0;
    loop {
        let mut rotation = None;
        let mut steps = 0;

        let mut token_end = code_index + 1;
        while token_end < code.len() {
            let rot = &code[token_end..=token_end];
            if rot == "R" || rot == "L" {
                rotation = Some(rot);
                break;
            }
            token_end += 1;
        }
        if token_end == code.len() {
            steps = code[code_index..=token_end - 1].parse::<usize>().unwrap();
        } else {
            steps = code[code_index..token_end].parse::<usize>().unwrap();
        }
        code_index = token_end + 1;

        'walk: for _ in 0..steps {
            match direction {
                Direction::Left => match map.get(&(position.0, position.1 - 1)) {
                    Some(Cell::Floor) => {
                        position = (position.0, position.1 - 1);
                    }
                    Some(Cell::Wall) => {
                        break 'walk;
                    }
                    None => {
                        let corner_at = (
                            position.0 / face_size * face_size,
                            position.1 / face_size * face_size,
                        );
                        let face_at = cube[&corner_at];
                        // let face_to = match face_at {
                        //     Face::Top => Face::Left;
                        //     Face::Bottom => todo!(),
                        //     Face::Far => todo!(),
                        //     Face::Near => todo!(),
                        //     Face::Right => todo!(),
                        //     Face::Left => todo!(),
                        // };
                        'wrap: for c in (isize::MIN..=max_column).rev() {
                            match map.get(&(position.0, c)) {
                                Some(Cell::Floor) => {
                                    position = (position.0, c);
                                    break 'wrap;
                                }
                                Some(Cell::Wall) => {
                                    break 'walk;
                                }
                                None => {}
                            }
                        }
                    }
                },
                Direction::Right => match map.get(&(position.0, position.1 + 1)) {
                    Some(Cell::Floor) => {
                        position = (position.0, position.1 + 1);
                    }
                    Some(Cell::Wall) => {
                        break 'walk;
                    }
                    None => {
                        'wrap: for c in 0.. {
                            match map.get(&(position.0, c)) {
                                Some(Cell::Floor) => {
                                    position = (position.0, c);
                                    break 'wrap;
                                }
                                Some(Cell::Wall) => {
                                    break 'walk;
                                }
                                None => {}
                            }
                        }
                    }
                },
                Direction::Up => match map.get(&(position.0 - 1, position.1)) {
                    Some(Cell::Floor) => {
                        position = (position.0 - 1, position.1);
                    }
                    Some(Cell::Wall) => {
                        break 'walk;
                    }
                    None => {
                        'wrap: for r in (isize::MIN..=max_row).rev() {
                            match map.get(&(r, position.1)) {
                                Some(Cell::Floor) => {
                                    position = (r, position.1);
                                    break 'wrap;
                                }
                                Some(Cell::Wall) => {
                                    break 'walk;
                                }
                                None => {}
                            }
                        }
                    }
                },
                Direction::Down => match map.get(&(position.0 + 1, position.1)) {
                    Some(Cell::Floor) => {
                        position = (position.0 + 1, position.1);
                    }
                    Some(Cell::Wall) => {
                        break 'walk;
                    }
                    None => {
                        'wrap: for r in 0.. {
                            match map.get(&(r, position.1)) {
                                Some(Cell::Floor) => {
                                    position = (r, position.1);
                                    break 'wrap;
                                }
                                Some(Cell::Wall) => {
                                    break 'walk;
                                }
                                None => {}
                            }
                        }
                    }
                },
            }
        }

        if let Some(rotation) = rotation {
            direction = match (rotation, direction) {
                ("L", Direction::Right) => Direction::Up,
                ("L", Direction::Left) => Direction::Down,
                ("L", Direction::Up) => Direction::Left,
                ("L", Direction::Down) => Direction::Right,
                ("R", Direction::Right) => Direction::Down,
                ("R", Direction::Left) => Direction::Up,
                ("R", Direction::Up) => Direction::Right,
                ("R", Direction::Down) => Direction::Left,
                _ => unreachable!(),
            }
        } else {
            break;
        }
    }

    1000 * (position.0 + 1)
        + 4 * (position.1 + 1)
        + match direction {
            Direction::Left => 2,
            Direction::Right => 0,
            Direction::Up => 3,
            Direction::Down => 1,
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 6032);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 5031);
    }
}
