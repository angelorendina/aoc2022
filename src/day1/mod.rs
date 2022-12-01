use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn star_one(filename: &str) -> u64 {
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
        .into_iter()
        .map(|elf| elf.into_iter().sum())
        .max()
        .expect("Vector is empty")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("src/day1/mock.txt"), 24000);
    }
}
