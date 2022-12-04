use std::collections::BTreeSet;

pub fn star_one() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut sum = 0;
    let mut set = BTreeSet::new();

    for line in values.lines() {
        set.clear();
        let len = line.len();
        let (dept_one, dept_two) = line.split_at(len / 2);
        for b in dept_one.as_bytes() {
            set.insert(if *b > 96 { *b - 96 } else { *b - 38 });
        }
        for b in dept_two.as_bytes() {
            let v = if *b > 96 { *b - 96 } else { *b - 38 };
            if set.contains(&v) {
                sum += v as u64;
                break;
            }
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
    let mut set_one = BTreeSet::new();
    let mut set_two = BTreeSet::new();

    let mut lines = values.lines();
    while let Some(line) = lines.next() {
        set_one.clear();
        set_two.clear();
        for b in line.as_bytes() {
            set_one.insert(if *b > 96 { *b - 96 } else { *b - 38 });
        }
        for b in lines.next().unwrap().as_bytes() {
            set_two.insert(if *b > 96 { *b - 96 } else { *b - 38 });
        }
        for b in lines.next().unwrap().as_bytes() {
            let v = if *b > 96 { *b - 96 } else { *b - 38 };
            if set_one.contains(&v) && set_two.contains(&v) {
                sum += v as u64;
                break;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 157);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 70);
    }
}
