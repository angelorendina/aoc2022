use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn star_one(filename: &str) -> u64 {
    read_calories(filename)
        .into_iter()
        .map(|elf| elf.into_iter().sum())
        .max()
        .expect("Vector is empty")
}

pub fn star_two(filename: &str) -> u64 {
    #[derive(PartialEq, Eq)]
    struct Elf {
        calories: u64,
        index: usize,
    }

    impl PartialOrd for Elf {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Elf {
        fn cmp(&self, other: &Self) -> Ordering {
            self.calories.cmp(&other.calories)
        }
    }

    read_calories(filename)
        .into_iter()
        .enumerate()
        .map(|(index, calories)| Elf {
            calories: calories.into_iter().sum::<u64>(),
            index,
        })
        .collect::<BTreeSet<Elf>>()
        .into_iter()
        .rev()
        .take(3)
        .map(|elf| elf.calories)
        .sum::<u64>()
}

fn read_calories(filename: &str) -> Vec<Vec<u64>> {
    let mut calories_by_elf: Vec<Vec<u64>> = vec![];
    let mut current_elf: Option<&mut Vec<u64>> = None;

    let file = File::open(filename).expect("Cannot open file");
    for line in io::BufReader::new(file).lines() {
        let line = line.expect("Cannot read line");
        if line.is_empty() {
            current_elf = None;
        } else {
            let elf = match current_elf.take() {
                Some(elf) => elf,
                None => {
                    calories_by_elf.push(vec![]);
                    calories_by_elf.last_mut().expect("Vector is empty")
                }
            };
            let food = line.parse().expect("Cannot parse value");
            elf.push(food);
            current_elf = Some(elf);
        }
    }

    calories_by_elf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("src/day1/mock.txt"), 24000);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two("src/day1/mock.txt"), 45000);
    }
}
