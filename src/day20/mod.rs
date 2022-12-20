use std::collections::VecDeque;

fn mix(data: &[isize], times: usize) -> VecDeque<usize> {
    let mut positions = data
        .iter()
        .enumerate()
        .map(|(i, _)| i)
        .collect::<VecDeque<_>>();
    let wrap = data.len() - 1;

    for _ in 0..times {
        for (i, &value) in data.iter().enumerate() {
            if value == 0 {
                continue;
            }

            let j = positions.iter().position(|&p| p == i).unwrap();
            let replaced = positions.remove(j).unwrap();

            if value < 0 {
                positions.rotate_right((-value) as usize % wrap);
            } else {
                positions.rotate_left(value as usize % wrap);
            }
            positions.insert(j, replaced);
        }
    }

    positions
}

pub fn star_one() -> isize {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let values = values
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let positions = mix(&values, 1);

    let original_zero_index = values.iter().position(|&v| v == 0).unwrap();
    let zero_index = positions
        .iter()
        .position(|&i| i == original_zero_index)
        .unwrap();

    (1..=3)
        .map(|i| {
            let j = (zero_index + i * 1000) % positions.len();
            values[positions[j]]
        })
        .sum()
}

pub fn star_two() -> isize {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let values = values
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .map(|v| v * 811589153)
        .collect::<Vec<_>>();

    let positions = mix(&values, 10);

    let original_zero_index = values.iter().position(|&v| v == 0).unwrap();
    let zero_index = positions
        .iter()
        .position(|&i| i == original_zero_index)
        .unwrap();

    (1..=3)
        .map(|i| {
            let j = (zero_index + i * 1000) % positions.len();
            values[positions[j]]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 3);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 1623178306);
    }
}
