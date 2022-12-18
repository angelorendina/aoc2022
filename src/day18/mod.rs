use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

pub fn star_one() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut cubes = BTreeSet::<(isize, isize, isize)>::new();
    for line in values.lines() {
        let mut coords = line.split(',').map(|c| c.parse::<isize>().unwrap());
        cubes.insert((
            coords.next().unwrap(),
            coords.next().unwrap(),
            coords.next().unwrap(),
        ));
    }

    let mut faces = 0;

    for &(x, y, z) in &cubes {
        let neighbours = cubes
            .range((x - 1, y - 1, z - 1)..=(x + 1, y + 1, z + 1))
            .collect::<BTreeSet<_>>();
        faces += 6;
        for &(xx, yy, zz) in neighbours {
            let dx = isize::abs_diff(x, xx);
            let dy = isize::abs_diff(y, yy);
            let dz = isize::abs_diff(z, zz);
            if dx + dy + dz == 1 {
                faces -= 1;
            }
        }
    }

    faces
}

pub fn star_two() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    #[derive(Debug)]
    enum Cell {
        Exterior,
        Solid,
    }

    let mut map = BTreeMap::<(isize, isize, isize), Cell>::new();

    let mut bound_start = (isize::MAX, isize::MAX, isize::MAX);
    let mut bound_end = (isize::MIN, isize::MIN, isize::MIN);
    let mut cubes = BTreeSet::<(isize, isize, isize)>::new();
    for line in values.lines() {
        let mut coords = line.split(',').map(|c| c.parse::<isize>().unwrap());
        let x = coords.next().unwrap();
        let y = coords.next().unwrap();
        let z = coords.next().unwrap();
        if x < bound_start.0 {
            bound_start.0 = x
        };
        if y < bound_start.1 {
            bound_start.1 = y
        };
        if z < bound_start.2 {
            bound_start.2 = z
        };
        if x > bound_end.0 {
            bound_end.0 = x
        };
        if y > bound_end.1 {
            bound_end.1 = y
        };
        if z > bound_end.2 {
            bound_end.2 = z
        };
        cubes.insert((x, y, z));
        map.insert((x, y, z), Cell::Solid);
    }
    bound_start.0 -= 1;
    bound_start.1 -= 1;
    bound_start.2 -= 1;
    bound_end.0 += 1;
    bound_end.1 += 1;
    bound_end.2 += 1;
    map.insert(bound_start, Cell::Exterior);
    map.insert(bound_end, Cell::Exterior);

    let mut queue = VecDeque::from([(bound_start)]);
    while let Some((x, y, z)) = queue.pop_front() {
        let neighbours = vec![
            (x + 1, y, z),
            (x - 1, y, z),
            (x, y + 1, z),
            (x, y - 1, z),
            (x, y, z + 1),
            (x, y, z - 1),
        ]
        .into_iter()
        .filter(|q| {
            q.0 >= bound_start.0
                && q.1 >= bound_start.1
                && q.2 >= bound_start.2
                && q.0 <= bound_end.0
                && q.1 <= bound_end.1
                && q.2 <= bound_end.2
        });
        for q in neighbours {
            if !map.contains_key(&q) {
                map.insert(q, Cell::Exterior);
                queue.push_back(q);
            }
        }
    }

    let mut faces = 0;

    for &(x, y, z) in &cubes {
        for xx in (x - 1)..=(x + 1) {
            let dx = isize::abs_diff(x, xx);
            for yy in (y - 1)..=(y + 1) {
                let dy = isize::abs_diff(y, yy);
                for zz in (z - 1)..=(z + 1) {
                    let dz = isize::abs_diff(z, zz);
                    if dx + dy + dz == 1 {
                        if let Some(Cell::Exterior) = map.get(&(xx, yy, zz)) {
                            faces += 1;
                        }
                    }
                }
            }
        }
    }

    faces
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 64);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 58);
    }
}
