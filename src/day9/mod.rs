use std::collections::BTreeSet;

pub fn star_one() -> usize {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut head = (0isize, 0isize);
    let mut tail = (0isize, 0isize);
    let mut visited = BTreeSet::new();
    visited.insert((tail.0, tail.1));

    for line in values.lines() {
        let mut tokens = line.split_ascii_whitespace();
        let directon = tokens.next().expect("there should be a direction");
        let steps = tokens
            .next()
            .expect("there should be a number of steps")
            .parse::<usize>()
            .expect("steps should be a number");
        for _ in 0..steps {
            match directon {
                "R" => {
                    head.1 += 1;
                }
                "L" => {
                    head.1 -= 1;
                }
                "U" => {
                    head.0 += 1;
                }
                "D" => {
                    head.0 -= 1;
                }
                _ => unreachable!("unrecognised direction"),
            }
            if tail.1 + 1 < head.1 {
                tail.1 += 1;
                if tail.0 < head.0 {
                    tail.0 += 1;
                }
                if tail.0 > head.0 {
                    tail.0 -= 1;
                }
            }
            if head.1 + 1 < tail.1 {
                tail.1 -= 1;
                if tail.0 < head.0 {
                    tail.0 += 1;
                }
                if tail.0 > head.0 {
                    tail.0 -= 1;
                }
            }
            if tail.0 + 1 < head.0 {
                tail.0 += 1;
                if tail.1 < head.1 {
                    tail.1 += 1;
                }
                if tail.1 > head.1 {
                    tail.1 -= 1;
                }
            }
            if head.0 + 1 < tail.0 {
                tail.0 -= 1;
                if tail.1 < head.1 {
                    tail.1 += 1;
                }
                if tail.1 > head.1 {
                    tail.1 -= 1;
                }
            }
            visited.insert((tail.0, tail.1));
        }
    }

    visited.len()
}

pub fn star_two() -> usize {
    #[cfg(test)]
    let values = include_str!("mock2.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut knots = [(0isize, 0isize); 10];
    let mut visited = BTreeSet::new();
    visited.insert((knots[0].0, knots[0].1));

    for line in values.lines() {
        let mut tokens = line.split_ascii_whitespace();
        let directon = tokens.next().expect("there should be a direction");
        let steps = tokens
            .next()
            .expect("there should be a number of steps")
            .parse::<usize>()
            .expect("steps should be a number");
        for _ in 0..steps {
            for i in (0usize..9).rev() {
                let mut head = knots[i + 1];
                let mut tail = knots[i];
                if i == 8 {
                    match directon {
                        "R" => {
                            head.1 += 1;
                        }
                        "L" => {
                            head.1 -= 1;
                        }
                        "U" => {
                            head.0 += 1;
                        }
                        "D" => {
                            head.0 -= 1;
                        }
                        _ => unreachable!("unrecognised direction"),
                    }
                }
                if tail.1 + 1 < head.1 {
                    tail.1 += 1;
                    if tail.0 < head.0 {
                        tail.0 += 1;
                    }
                    if tail.0 > head.0 {
                        tail.0 -= 1;
                    }
                }
                if head.1 + 1 < tail.1 {
                    tail.1 -= 1;
                    if tail.0 < head.0 {
                        tail.0 += 1;
                    }
                    if tail.0 > head.0 {
                        tail.0 -= 1;
                    }
                }
                if tail.0 + 1 < head.0 {
                    tail.0 += 1;
                    if tail.1 < head.1 {
                        tail.1 += 1;
                    }
                    if tail.1 > head.1 {
                        tail.1 -= 1;
                    }
                }
                if head.0 + 1 < tail.0 {
                    tail.0 -= 1;
                    if tail.1 < head.1 {
                        tail.1 += 1;
                    }
                    if tail.1 > head.1 {
                        tail.1 -= 1;
                    }
                }
                if i == 0 {
                    visited.insert((tail.0, tail.1));
                }
                knots[i + 1] = head;
                knots[i] = tail;
            }
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 13);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 36);
    }
}
