use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
struct V3([isize; 3]);

impl std::ops::Deref for V3 {
    type Target = [isize; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Mul<V3> for V3 {
    type Output = isize;

    fn mul(self, rhs: V3) -> Self::Output {
        (0..3).map(|i| self[i] * rhs[i]).sum()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
struct M3([V3; 3]);

impl std::ops::Deref for M3 {
    type Target = [V3; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl M3 {
    fn transposed(self) -> Self {
        M3([
            V3([self[0][0], self[0][1], self[0][2]]),
            V3([self[1][0], self[1][1], self[1][2]]),
            V3([self[2][0], self[2][1], self[2][2]]),
        ])
    }

    fn complement(self, row: usize, column: usize) -> M2 {
        let M3(columns) = self;
        let mut new_columns = columns
            .into_iter()
            .enumerate()
            .filter(|(c, _)| *c != column)
            .map(|(_, c)| {
                let V3(rows) = c;
                let mut new_rows = rows
                    .into_iter()
                    .enumerate()
                    .filter(|(r, _)| *r != row)
                    .map(|(_, x)| x);
                V2([new_rows.next().unwrap(), new_rows.next().unwrap()])
            });
        M2([new_columns.next().unwrap(), new_columns.next().unwrap()])
    }
}

impl std::ops::Mul<V3> for M3 {
    type Output = V3;

    fn mul(self, rhs: V3) -> Self::Output {
        let M3([r1, r2, r3]) = self.transposed();
        V3([r1 * rhs, r2 * rhs, r3 * rhs])
    }
}

impl std::ops::Mul<M3> for M3 {
    type Output = M3;

    fn mul(self, rhs: M3) -> Self::Output {
        let M3([c1, c2, c3]) = rhs;
        M3([self * c1, self * c2, self * c3])
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
struct V2([isize; 2]);

impl std::ops::Deref for V2 {
    type Target = [isize; 2];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Mul<V2> for V2 {
    type Output = isize;

    fn mul(self, rhs: V2) -> Self::Output {
        (0..2).map(|i| self[i] * rhs[i]).sum()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
struct M2([V2; 2]);

impl std::ops::Deref for M2 {
    type Target = [V2; 2];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl M2 {
    fn transposed(self) -> Self {
        M2([V2([self[0][0], self[0][1]]), V2([self[1][0], self[1][1]])])
    }

    fn inverted(self) -> Self {
        let det = self[0][0] * self[1][1] - self[0][1] * self[1][0];
        M2([
            V2([self[1][1] / det, -self[0][1] / det]),
            V2([-self[1][0] / det, self[0][0] / det]),
        ])
    }
}

impl std::ops::Mul<V2> for M2 {
    type Output = V2;

    fn mul(self, rhs: V2) -> Self::Output {
        let M2([r1, r2]) = self.transposed();
        V2([r1 * rhs, r2 * rhs])
    }
}

impl std::ops::Mul<M2> for M2 {
    type Output = M2;

    fn mul(self, rhs: M2) -> Self::Output {
        let M2([c1, c2]) = rhs;
        M2([self * c1, self * c2])
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
        let steps;

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
    let mut cube = BTreeMap::<(isize, isize), M3>::new();
    let first_face = corners.iter().next().unwrap().clone();
    cube.insert(
        first_face,
        M3([V3([0, -1, 0]), V3([1, 0, 0]), V3([0, 0, 1])]),
    );

    let rot_south = M3([V3([0, 0, -1]), V3([0, 1, 0]), V3([1, 0, 0])]);
    let rot_north = M3([V3([0, 0, 1]), V3([0, 1, 0]), V3([-1, 0, 0])]);
    let rot_east = M3([V3([1, 0, 0]), V3([0, 0, -1]), V3([0, 1, 0])]);
    let rot_west = M3([V3([1, 0, 0]), V3([0, 0, 1]), V3([0, -1, 0])]);

    while cube.len() < 6 {
        let mut x = None;
        for (&coords, &m) in &cube {
            let neighbour = (coords.0, coords.1 + face_size);
            if corners.contains(&neighbour) && !cube.contains_key(&neighbour) {
                let mm = m * rot_east;
                x = Some((neighbour, mm));
                break;
            }
            let neighbour = (coords.0, coords.1 - face_size);
            if corners.contains(&neighbour) && !cube.contains_key(&neighbour) {
                let mm = m * rot_west;
                x = Some((neighbour, mm));
                break;
            }
            let neighbour = (coords.0 + face_size, coords.1);
            if corners.contains(&neighbour) && !cube.contains_key(&neighbour) {
                let mm = m * rot_south;
                x = Some((neighbour, mm));
                break;
            }
            let neighbour = (coords.0 - face_size, coords.1);
            if corners.contains(&neighbour) && !cube.contains_key(&neighbour) {
                let mm = m * rot_north;
                x = Some((neighbour, mm));
                break;
            }
        }
        let x = x.unwrap();
        cube.insert(x.0, x.1);
    }

    let mut position = (start.1, start.2);
    let mut direction = Direction::Right;

    let mut code_index = 0;
    loop {
        let mut rotation = None;
        let steps;

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
            println!("{steps} {rotation:?}");
            for r in -10..10 {
                let r = position.0 + r;
                for c in -10..10 {
                    let c = position.1 + c;
                    print!("{}", match map.get(&(r,c)) {
                        _ if (r == position.0 && c == position.1) => match direction {
                            Direction::Down => "v",
                            Direction::Up => "^",
                            Direction::Left => "<",
                            Direction::Right => ">",
                        }
                        Some(Cell::Floor) => ".",
                        Some(Cell::Wall) => "#",
                        None => " ",
                    });
                }
                println!()
            }
            println!();
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
                        let spatial_rotation_at = cube[&corner_at];
                        let spatial_turn = spatial_rotation_at * rot_west;
                        let (&corner_there, &matching_spatial) = cube
                            .iter()
                            .find(|(_, m)| m[2] == spatial_turn[2])
                            .unwrap();
                        let (complement_row, _) = spatial_turn[2]
                            .iter()
                            .enumerate()
                            .find(|(_, x)| **x != 0)
                            .unwrap();
                        let local_turn = spatial_turn.complement(complement_row, 2);
                        let local_there = matching_spatial.complement(complement_row, 2);
                        let transition = local_there * local_turn.inverted();
                        let position_and_direction =
                            M2([V2([position.0 - corner_at.0, 0]), V2([0, -1])]);
                        let M2([local_position_there, new_direction]) =
                            transition * position_and_direction;
                        let position_there = (
                            local_position_there[0] + corner_there.0,
                            local_position_there[1] + corner_there.1,
                        );
                        match map.get(&position_there) {
                            Some(Cell::Floor) => {
                                position = position_there;
                                direction = match new_direction.0 {
                                    [0, 1] => Direction::Right,
                                    [0, -1] => Direction::Left,
                                    [1, 0] => Direction::Down,
                                    [-1, 0] => Direction::Up,
                                    _ => unreachable!(),
                                };
                            }
                            Some(Cell::Wall) => {
                                break 'walk;
                            }
                            None => unreachable!(),
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
                        let corner_at = (
                            position.0 / face_size * face_size,
                            position.1 / face_size * face_size,
                        );
                        let spatial_rotation_at = cube[&corner_at];
                        let spatial_turn = spatial_rotation_at * rot_east;
                        let (&corner_there, &matching_spatial) = cube
                            .iter()
                            .find(|(_, m)| m[2] == spatial_turn[2])
                            .unwrap();
                        let (complement_row, _) = spatial_turn[2]
                            .iter()
                            .enumerate()
                            .find(|(_, x)| **x != 0)
                            .unwrap();
                        let local_turn = spatial_turn.complement(complement_row, 2);
                        let local_there = matching_spatial.complement(complement_row, 2);
                        let transition = local_there * local_turn.inverted();
                        let position_and_direction =
                            M2([V2([position.0 - corner_at.0, 0]), V2([0, 1])]);
                        let M2([local_position_there, new_direction]) =
                            transition * position_and_direction;
                        let position_there = (
                            local_position_there[0] + corner_there.0,
                            local_position_there[1] + corner_there.1,
                        );
                        match map.get(&position_there) {
                            Some(Cell::Floor) => {
                                position = position_there;
                                direction = match new_direction.0 {
                                    [0, 1] => Direction::Right,
                                    [0, -1] => Direction::Left,
                                    [1, 0] => Direction::Down,
                                    [-1, 0] => Direction::Up,
                                    _ => unreachable!(),
                                };
                            }
                            Some(Cell::Wall) => {
                                break 'walk;
                            }
                            None => unreachable!(),
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
                        let corner_at = (
                            position.0 / face_size * face_size,
                            position.1 / face_size * face_size,
                        );
                        let spatial_rotation_at = cube[&corner_at];
                        let spatial_turn = spatial_rotation_at * rot_north;
                        let (&corner_there, &matching_spatial) = cube
                            .iter()
                            .find(|(_, m)| m[2] == spatial_turn[2])
                            .unwrap();
                        let (complement_row, _) = spatial_turn[2]
                            .iter()
                            .enumerate()
                            .find(|(_, x)| **x != 0)
                            .unwrap();
                        let local_turn = spatial_turn.complement(complement_row, 2);
                        let local_there = matching_spatial.complement(complement_row, 2);
                        let transition = local_there * local_turn.inverted();
                        let position_and_direction =
                            M2([V2([0, position.1 - corner_at.1]), V2([-1, 0])]);
                        let M2([local_position_there, new_direction]) =
                            transition * position_and_direction;
                        let position_there = (
                            local_position_there[0] + corner_there.0,
                            local_position_there[1] + corner_there.1,
                        );
                        match map.get(&position_there) {
                            Some(Cell::Floor) => {
                                position = position_there;
                                direction = match new_direction.0 {
                                    [0, 1] => Direction::Right,
                                    [0, -1] => Direction::Left,
                                    [1, 0] => Direction::Down,
                                    [-1, 0] => Direction::Up,
                                    _ => unreachable!(),
                                };
                            }
                            Some(Cell::Wall) => {
                                break 'walk;
                            }
                            None => unreachable!(),
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
                        let corner_at = (
                            position.0 / face_size * face_size,
                            position.1 / face_size * face_size,
                        );
                        let spatial_rotation_at = cube[&corner_at];
                        let spatial_turn = spatial_rotation_at * rot_south;
                        let (&corner_there, &matching_spatial) = cube
                            .iter()
                            .find(|(_, m)| m[2] == spatial_turn[2])
                            .unwrap();
                        let (complement_row, _) = spatial_turn[2]
                            .iter()
                            .enumerate()
                            .find(|(_, x)| **x != 0)
                            .unwrap();
                        let local_turn = spatial_turn.complement(complement_row, 2);
                        let local_there = matching_spatial.complement(complement_row, 2);
                        let transition = local_there * local_turn.inverted();
                        let position_and_direction =
                            M2([V2([0, position.1 - corner_at.1]), V2([1, 0])]);
                        let M2([local_position_there, new_direction]) =
                            transition * position_and_direction;
                        let position_there = (
                            local_position_there[0] + corner_there.0,
                            local_position_there[1] + corner_there.1,
                        );
                        match map.get(&position_there) {
                            Some(Cell::Floor) => {
                                position = position_there;
                                direction = match new_direction.0 {
                                    [0, 1] => Direction::Right,
                                    [0, -1] => Direction::Left,
                                    [1, 0] => Direction::Down,
                                    [-1, 0] => Direction::Up,
                                    _ => unreachable!(),
                                };
                            }
                            Some(Cell::Wall) => {
                                break 'walk;
                            }
                            None => unreachable!(),
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
