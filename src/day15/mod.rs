use rayon::prelude::*;
use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Segment(isize, isize);

impl Segment {
    fn new(from_included: isize, to_included: isize) -> Option<Self> {
        if to_included >= from_included {
            Some(Self(from_included, to_included + 1))
        } else {
            None
        }
    }

    fn subtraction(self, rhs: Self) -> Vec<Self> {
        let (Segment(a, b), Segment(c, d)) = (self, rhs);
        match a.cmp(&c) {
            std::cmp::Ordering::Less => match b.cmp(&c) {
                std::cmp::Ordering::Less => vec![Segment(a, b)],
                std::cmp::Ordering::Equal => vec![Segment(a, b)],
                std::cmp::Ordering::Greater => match b.cmp(&d) {
                    std::cmp::Ordering::Less => vec![Segment(a, c)],
                    std::cmp::Ordering::Equal => vec![Segment(a, c)],
                    std::cmp::Ordering::Greater => vec![Segment(a, c), Segment(b, d)],
                },
            },
            std::cmp::Ordering::Equal => match b.cmp(&d) {
                std::cmp::Ordering::Less => vec![],
                std::cmp::Ordering::Equal => vec![],
                std::cmp::Ordering::Greater => vec![Segment(d, b)],
            },
            std::cmp::Ordering::Greater => match d.cmp(&a) {
                std::cmp::Ordering::Less => vec![Segment(a, b)],
                std::cmp::Ordering::Equal => vec![Segment(a, b)],
                std::cmp::Ordering::Greater => match d.cmp(&b) {
                    std::cmp::Ordering::Less => vec![Segment(d, b)],
                    std::cmp::Ordering::Equal => vec![],
                    std::cmp::Ordering::Greater => vec![],
                },
            },
        }
    }

    fn intersection(self, rhs: Self) -> Option<Self> {
        let (Segment(_, b), Segment(c, d)) = if self.0 < rhs.0 {
            (self, rhs)
        } else {
            (rhs, self)
        };
        match b.cmp(&c) {
            std::cmp::Ordering::Less => None,
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => match b.cmp(&d) {
                std::cmp::Ordering::Less => Some(Self(c, b)),
                std::cmp::Ordering::Equal => Some(Self(c, b)),
                std::cmp::Ordering::Greater => Some(Self(c, d)),
            },
        }
    }

    fn contains(&self, x: isize) -> bool {
        x >= self.0 && x < self.1
    }
}

pub fn star_one() -> isize {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut map = BTreeMap::<(isize, isize), (isize, isize)>::new();

    for line in values.lines() {
        let line = line.strip_prefix("Sensor at ").unwrap();
        let (coords, line) = line.split_once(':').unwrap();

        let coords = coords.split_once(", ").unwrap();
        let sensor = [coords.0, coords.1].map(|coord| {
            let (_, coord) = coord.split_once('=').unwrap();
            coord.parse::<isize>().unwrap()
        });

        let coords = line.strip_prefix(" closest beacon is at ").unwrap();
        let coords = coords.split_once(", ").unwrap();
        let beacon = [coords.0, coords.1].map(|coord| {
            let (_, coord) = coord.split_once('=').unwrap();
            coord.parse::<isize>().unwrap()
        });

        map.insert((sensor[0], sensor[1]), (beacon[0], beacon[1]));
    }

    #[cfg(test)]
    let h = 10;
    #[cfg(not(test))]
    let h = 2_000_000;

    let mut segmentation = Vec::<Segment>::new();
    for (sensor, beacon) in map {
        let distance = isize::abs(sensor.0 - beacon.0) + isize::abs(sensor.1 - beacon.1);
        let dh = isize::abs(sensor.1 - h);
        if distance >= dh {
            let u = distance - dh;
            let mut segmented_new = vec![Segment::new(sensor.0 - u, sensor.0 + u).unwrap()];
            for &old in &segmentation {
                segmented_new = segmented_new
                    .into_iter()
                    .flat_map(|chunk| chunk.subtraction(old))
                    .collect();
            }
            segmentation.extend(segmented_new);
        }
    }

    segmentation.into_iter().map(|s| s.1 - s.0).sum::<isize>() - 1
}

pub fn star_two() -> isize {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut map = BTreeMap::<(isize, isize), (isize, isize)>::new();

    for line in values.lines() {
        let line = line.strip_prefix("Sensor at ").unwrap();
        let (coords, line) = line.split_once(':').unwrap();

        let coords = coords.split_once(", ").unwrap();
        let sensor = [coords.0, coords.1].map(|coord| {
            let (_, coord) = coord.split_once('=').unwrap();
            coord.parse::<isize>().unwrap()
        });

        let coords = line.strip_prefix(" closest beacon is at ").unwrap();
        let coords = coords.split_once(", ").unwrap();
        let beacon = [coords.0, coords.1].map(|coord| {
            let (_, coord) = coord.split_once('=').unwrap();
            coord.parse::<isize>().unwrap()
        });

        map.insert((sensor[0], sensor[1]), (beacon[0], beacon[1]));
    }

    #[cfg(test)]
    const H: isize = 20;
    #[cfg(not(test))]
    const H: isize = 4_000_000;

    (0..=H)
        .into_par_iter()
        .find_map_any(|h| {
            let mut segmentation = Vec::<Segment>::new();
            for (&sensor, &beacon) in &map {
                let distance = isize::abs(sensor.0 - beacon.0) + isize::abs(sensor.1 - beacon.1);
                let dh = isize::abs(sensor.1 - h);
                if distance >= dh {
                    let u = distance - dh;
                    let mut segmented_new = vec![Segment::new(sensor.0 - u, sensor.0 + u)
                        .unwrap()
                        .intersection(Segment::new(0, H).unwrap())
                        .unwrap()];
                    for &old in &segmentation {
                        segmented_new = segmented_new
                            .into_iter()
                            .flat_map(|chunk| chunk.subtraction(old))
                            .collect();
                    }
                    segmentation.extend(segmented_new);
                }
            }

            if segmentation.iter().map(|&s| s.1 - s.0).sum::<isize>() == H {
                for x in 0..=H {
                    if segmentation.iter().any(|s| s.contains(x)) {
                        continue;
                    }
                    return Some(x * 4000000 + h);
                }
            }
            None
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 26);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 56000011);
    }
}
