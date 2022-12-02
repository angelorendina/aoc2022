#[derive(Clone, Copy)]
enum Sign {
    Rock,
    Paper,
    Scissor,
}

impl std::str::FromStr for Sign {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Sign::Rock),
            "B" | "Y" => Ok(Sign::Paper),
            "C" | "Z" => Ok(Sign::Scissor),
            _ => Err("Not recognised"),
        }
    }
}

impl Sign {
    fn score(elf: Sign, response: Sign) -> u64 {
        match response {
            Sign::Rock => {
                1 + match elf {
                    Sign::Rock => 3,
                    Sign::Paper => 0,
                    Sign::Scissor => 6,
                }
            }
            Sign::Paper => {
                2 + match elf {
                    Sign::Rock => 6,
                    Sign::Paper => 3,
                    Sign::Scissor => 0,
                }
            }
            Sign::Scissor => {
                3 + match elf {
                    Sign::Rock => 0,
                    Sign::Paper => 6,
                    Sign::Scissor => 3,
                }
            }
        }
    }

    /// response encodes:
    /// - rock => actual response should lose
    /// - paper => actual response should draw
    /// - scissor => actual response should win
    fn evaluate_strategy(elf: Sign, encoded_response: Sign) -> Sign {
        match elf {
            Sign::Rock => match encoded_response {
                Sign::Rock => Sign::Scissor,
                Sign::Paper => Sign::Rock,
                Sign::Scissor => Sign::Paper,
            },
            Sign::Paper => match encoded_response {
                Sign::Rock => Sign::Rock,
                Sign::Paper => Sign::Paper,
                Sign::Scissor => Sign::Scissor,
            },
            Sign::Scissor => match encoded_response {
                Sign::Rock => Sign::Paper,
                Sign::Paper => Sign::Scissor,
                Sign::Scissor => Sign::Rock,
            },
        }
    }
}

pub fn star_one() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut score = 0;
    for line in values.lines() {
        let mut tokens = line.split_whitespace();
        let elf = tokens
            .next()
            .expect("Missing elf")
            .parse::<Sign>()
            .expect("Invalid elf");
        let response = tokens
            .next()
            .expect("Missing response")
            .parse::<Sign>()
            .expect("Invalid response");
        score += Sign::score(elf, response);
    }

    score
}

pub fn star_two() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut score = 0;
    for line in values.lines() {
        let mut tokens = line.split_whitespace();
        let elf = tokens
            .next()
            .expect("Missing elf")
            .parse::<Sign>()
            .expect("Invalid elf");
        let encoded_response = tokens
            .next()
            .expect("Missing response")
            .parse::<Sign>()
            .expect("Invalid response");
        let actual_response = Sign::evaluate_strategy(elf, encoded_response);
        score += Sign::score(elf, actual_response);
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 15);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 12);
    }
}
