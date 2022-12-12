/// Structure responsible for storing the total calories of N elves.
/// These are kept in descending order.
///
/// Everything is stack-allocated.
struct ChunkiestElves<const N: usize> {
    elves: [u64; N],
}

impl<const N: usize> ChunkiestElves<N> {
    /// Reads an empty-string separated list of calories,
    /// aggregating by elf and remembering the top N.
    ///
    /// See `mock.txt` or `input.txt` for examples on the formatting.
    fn parse(values: &str) -> Self {
        let mut this = Self { elves: [0; N] };

        // current aggregate
        let mut elf = 0;

        for line in values.lines() {
            if line.is_empty() {
                // on empty lines, store the aggregated value, and start a new one
                this.push(elf);
                elf = 0;
            } else {
                // increase the aggregate by the value in this line
                let calories = line.parse::<u64>().expect("Cannot parse value");
                elf += calories;
            }
        }

        // remember to store the last elf!
        this.push(elf);

        this
    }

    /// Consider the given Elf, and store the value at the correct position if it is amongst the top N.
    fn push(&mut self, mut elf: u64) {
        // the algorithm keeps higher values in place, and replaces the first lower value with the target;
        // this lower value is the new target, and we keep iterating down the ordered list
        // repeating the process as needed.
        // Essentially, this is inserting the value at the right place and shifting lower values further
        for candidate in self.elves.iter_mut() {
            if candidate < &mut elf {
                std::mem::swap(candidate, &mut elf);
            }
        }
    }
}

pub fn star_one() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");
    ChunkiestElves::<1>::parse(values).elves[0]
}

pub fn star_two() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");
    ChunkiestElves::<3>::parse(values).elves.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 24000);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 45000);
    }
}
