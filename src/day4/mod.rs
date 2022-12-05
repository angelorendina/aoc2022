struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
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

    let bytes = values.as_bytes();
    let len = bytes.len();

    let mut i = 0;
    let mut numbers = [0u64; 4];
    while i < len {
        unsafe {
            let mut j = 0;
            while j < 4 {
                let b = *bytes.get_unchecked(i);
                i += 1;
                if b == 44 || b == 45 || b == 10 {
                    j += 1;
                } else {
                    *numbers.get_unchecked_mut(j) *= 10;
                    *numbers.get_unchecked_mut(j) += b as u64 - 48;
                }
            }
            let a = numbers.get_unchecked(0);
            let b = numbers.get_unchecked(1);
            let c = numbers.get_unchecked(2);
            let d = numbers.get_unchecked(3);
            if (c >= a) && (d <= b) {
                sum += 1;
            } else {
                if (a >= c) && (b <= d) {
                    sum += 1;
                }
            }
            numbers = [0; 4];
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
