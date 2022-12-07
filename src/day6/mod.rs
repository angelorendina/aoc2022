use std::collections::BTreeSet;

pub fn star_one() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let bytes = values.lines().next().unwrap().as_bytes();
    let offset = bytes
        .windows(4)
        .enumerate()
        .find(|(_, window)| {
            let symbols = window.iter().collect::<BTreeSet<_>>();
            symbols.len() == 4
        })
        .map(|(offset, _)| offset + 4)
        .unwrap();

    offset as u64
}

pub fn star_two() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let bytes = values.lines().next().unwrap().as_bytes();
    let offset = bytes
        .windows(14)
        .enumerate()
        .find(|(_, window)| {
            let symbols = window.iter().collect::<BTreeSet<_>>();
            symbols.len() == 14
        })
        .map(|(offset, _)| offset + 14)
        .unwrap();

    offset as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 7);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 19);
    }
}
