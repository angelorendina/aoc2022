use std::collections::BTreeMap;

struct GridMap<T> {
    data: BTreeMap<(isize, isize), T>,
    left: isize,
    right: isize,
    top: isize,
    bot: isize,
}

impl<T> GridMap<T> {
    fn new() -> Self {
        Self {
            data: BTreeMap::new(),
            left: 0,
            right: 0,
            top: 0,
            bot: 0,
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<&T> {
        self.data.get(&(x, y))
    }

    fn encloses(&self, x: isize, y: isize) -> bool {
        self.left <= x && self.right > x && self.top <= y && self.bot > y
    }

    fn set(&mut self, x: isize, y: isize, value: T) {
        self.data.insert((x, y), value);
        if self.data.len() == 1 {
            self.left = x;
            self.top = y;
            self.right = x + 1;
            self.bot = y + 1;
        } else {
            self.left = isize::min(self.left, x);
            self.top = isize::min(self.top, y);
            self.right = isize::max(self.right, x + 1);
            self.bot = isize::max(self.bot, y + 1);
        }
    }
}

pub fn star_one() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut map = GridMap::new();
    let source = (500, 0);
    map.set(source.0, source.1, ());

    for line in values.lines() {
        let mut rock = None;
        for coords in line.split(" -> ") {
            let mut coords = coords
                .split(",")
                .map(|coord| coord.parse::<isize>().unwrap());
            let x_to = coords.next().unwrap();
            let y_to = coords.next().unwrap();
            if let Some((x_from, y_from)) = rock.replace((x_to, y_to)) {
                let dx = (x_to - x_from).signum();
                let dy = (y_to - y_from).signum();
                for i in 0.. {
                    let x = x_from + dx * i;
                    let y = y_from + dy * i;
                    map.set(x, y, ());
                    if x == x_to && y == y_to {
                        break;
                    };
                }
            }
        }
    }

    let mut settled = 0;
    loop {
        let mut sand = source;
        loop {
            if !map.encloses(sand.0, sand.1) {
                return settled;
            }
            if map.get(sand.0, sand.1 + 1).is_none() {
                sand.1 += 1;
                continue;
            }
            if map.get(sand.0 - 1, sand.1 + 1).is_none() {
                sand.1 += 1;
                sand.0 -= 1;
                continue;
            }
            if map.get(sand.0 + 1, sand.1 + 1).is_none() {
                sand.1 += 1;
                sand.0 += 1;
                continue;
            }
            map.set(sand.0, sand.1, ());
            settled += 1;
            break;
        }
    }
}

pub fn star_two() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut map = GridMap::new();
    let source = (500, 0);
    map.set(source.0, source.1, ());

    for line in values.lines() {
        let mut rock = None;
        for coords in line.split(" -> ") {
            let mut coords = coords
                .split(",")
                .map(|coord| coord.parse::<isize>().unwrap());
            let x_to = coords.next().unwrap();
            let y_to = coords.next().unwrap();
            if let Some((x_from, y_from)) = rock.replace((x_to, y_to)) {
                let dx = (x_to - x_from).signum();
                let dy = (y_to - y_from).signum();
                for i in 0.. {
                    let x = x_from + dx * i;
                    let y = y_from + dy * i;
                    map.set(x, y, ());
                    if x == x_to && y == y_to {
                        break;
                    };
                }
            }
        }
    }

    let bot = map.bot;
    let mut settled = 0;
    loop {
        let mut sand = source;
        loop {
            if sand.1 < bot {
                if map.get(sand.0, sand.1 + 1).is_none() {
                    sand.1 += 1;
                    continue;
                }
                if map.get(sand.0 - 1, sand.1 + 1).is_none() {
                    sand.1 += 1;
                    sand.0 -= 1;
                    continue;
                }
                if map.get(sand.0 + 1, sand.1 + 1).is_none() {
                    sand.1 += 1;
                    sand.0 += 1;
                    continue;
                }
            }
            map.set(sand.0, sand.1, ());
            settled += 1;
            if sand == source {
                return settled;
            }
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 24);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 93);
    }
}
