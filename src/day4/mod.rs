struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    fn contains(&self, other: &Self) -> bool {
        (other.start >= self.start) && (other.end <= self.end)
    }

    fn overlaps(this: &Self, other: &Self) -> bool {
        !(other.start > this.end) && !(other.end < this.start)
    }
}

pub fn star_one() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut sum = 0;

    for line in values.lines() {
        let mut tokeniser = line.split(",").map(|each| {
            let mut each = each.split("-").map(|n| n.parse::<u64>().unwrap());
            Range::new(each.next().unwrap(), each.next().unwrap())
        });
        let range_one = tokeniser.next().unwrap();
        let range_two = tokeniser.next().unwrap();
        if range_one.contains(&range_two) || range_two.contains(&range_one) {
            sum += 1;
        }
    }

    sum
}

pub fn star_two() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut sum = 0;

    for line in values.lines() {
        let mut tokeniser = line.split(",").map(|each| {
            let mut each = each.split("-").map(|n| n.parse::<u64>().unwrap());
            Range::new(each.next().unwrap(), each.next().unwrap())
        });
        let range_one = tokeniser.next().unwrap();
        let range_two = tokeniser.next().unwrap();
        if Range::overlaps(&range_one, &range_two) {
            sum += 1;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 2);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 4);
    }
}
