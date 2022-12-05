use std::collections::BTreeMap;

pub fn star_one() -> String {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut stacks = BTreeMap::<usize, Vec<char>>::new();

    let mut lines = values.lines();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        for (lane, chunk) in line
            .chars()
            .collect::<Vec<char>>()
            .as_slice()
            .chunks(4)
            .enumerate()
        {
            if chunk[0] == '[' {
                stacks
                    .entry(lane + 1)
                    .or_insert_with(|| Vec::new())
                    .insert(0, chunk[1]);
            }
        }
    }

    while let Some(line) = lines.next() {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        let amount = tokens[1].parse::<usize>().unwrap();
        let lane_from = tokens[3].parse::<usize>().unwrap();
        let lane_to = tokens[5].parse::<usize>().unwrap();

        let mut shifted_crates = Vec::<char>::new();
        for _ in 0..amount {
            let lane_from = stacks.entry(lane_from).or_insert_with(|| vec![]);
            if let Some(item) = lane_from.pop() {
                shifted_crates.push(item);
            }
        }
        stacks
            .entry(lane_to)
            .or_insert_with(|| vec![])
            .extend(shifted_crates.into_iter());
    }

    let mut tops = String::new();
    for (_, mut lane) in stacks {
        tops.push(lane.pop().unwrap_or(' '))
    }

    tops
}

pub fn star_two() -> String {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut stacks = BTreeMap::<usize, Vec<char>>::new();

    let mut lines = values.lines();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        for (lane, chunk) in line
            .chars()
            .collect::<Vec<char>>()
            .as_slice()
            .chunks(4)
            .enumerate()
        {
            if chunk[0] == '[' {
                stacks
                    .entry(lane + 1)
                    .or_insert_with(|| Vec::new())
                    .insert(0, chunk[1]);
            }
        }
    }

    while let Some(line) = lines.next() {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        let amount = tokens[1].parse::<usize>().unwrap();
        let lane_from = tokens[3].parse::<usize>().unwrap();
        let lane_to = tokens[5].parse::<usize>().unwrap();

        let mut shifted_crates = Vec::<char>::new();
        for _ in 0..amount {
            let lane_from = stacks.entry(lane_from).or_insert_with(|| vec![]);
            if let Some(item) = lane_from.pop() {
                shifted_crates.push(item);
            }
        }
        stacks
            .entry(lane_to)
            .or_insert_with(|| vec![])
            .extend(shifted_crates.into_iter().rev());
    }

    let mut tops = String::new();
    for (_, mut lane) in stacks {
        tops.push(lane.pop().unwrap_or(' '))
    }

    tops
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), "CMZ".to_string());
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), "MCD".to_string());
    }
}
