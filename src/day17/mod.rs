use std::collections::BTreeMap;
use std::collections::BTreeSet;

enum Block {
    Minus,
    Plus,
    Bend,
    Pipe,
    Cube,
}

impl From<usize> for Block {
    fn from(value: usize) -> Self {
        match value % 5 {
            0 => Self::Minus,
            1 => Self::Plus,
            2 => Self::Bend,
            3 => Self::Pipe,
            4 => Self::Cube,
            _ => unreachable!(),
        }
    }
}

impl Block {
    fn project(&self, left: usize, bottom: usize) -> BTreeSet<(usize, usize)> {
        match self {
            Block::Minus => BTreeSet::from([
                (bottom, left),
                (bottom, left + 1),
                (bottom, left + 2),
                (bottom, left + 3),
            ]),
            Block::Plus => BTreeSet::from([
                (bottom, left + 1),
                (bottom + 1, left),
                (bottom + 1, left + 1),
                (bottom + 1, left + 2),
                (bottom + 2, left + 1),
            ]),
            Block::Bend => BTreeSet::from([
                (bottom, left),
                (bottom, left + 1),
                (bottom, left + 2),
                (bottom + 1, left + 2),
                (bottom + 2, left + 2),
            ]),
            Block::Pipe => BTreeSet::from([
                (bottom, left),
                (bottom + 1, left),
                (bottom + 2, left),
                (bottom + 3, left),
            ]),
            Block::Cube => BTreeSet::from([
                (bottom, left),
                (bottom, left + 1),
                (bottom + 1, left),
                (bottom + 1, left + 1),
            ]),
        }
    }
}

pub fn star_one() -> usize {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut tableau: BTreeSet<(usize, usize)> = BTreeSet::new();

    let mut tick = 0usize;
    for n_rock in 0usize..2022 {
        let mut left = 2usize;
        let (mut bottom, _) = *tableau.iter().last().unwrap_or_else(|| &(0, 0));
        bottom += 4;
        loop {
            let shift = values.as_bytes()[tick % (values.len() - 1)];
            let block = Block::from(n_rock);
            let projection = block.project(left, bottom);
            match shift {
                60 => {
                    let against_left_wall = projection.iter().any(|&(_, l)| l == 0);
                    if !against_left_wall {
                        let would_sweep_into = projection
                            .iter()
                            .any(|&(b, l)| tableau.contains(&(b, l - 1)));
                        if !would_sweep_into {
                            left -= 1;
                        }
                    }
                }
                62 => {
                    let against_right_wall = projection.iter().any(|&(_, l)| l == 6);
                    if !against_right_wall {
                        let would_sweep_into = projection
                            .iter()
                            .any(|&(b, l)| tableau.contains(&(b, l + 1)));
                        if !would_sweep_into {
                            left += 1;
                        }
                    }
                }
                _ => unreachable!(),
            }
            tick += 1;
            let projection = block.project(left, bottom);
            let will_lay = bottom == 1
                || projection
                    .iter()
                    .any(|&(b, l)| tableau.contains(&(b - 1, l)));
            if will_lay {
                tableau.extend(projection);
                break;
            } else {
                bottom -= 1;
            }
        }
    }

    let (bottom, _) = *tableau.iter().last().unwrap();
    bottom
}

pub fn star_two() -> usize {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    const N: usize = 1_000_000_000_000 - 1;

    let mut tableau: BTreeSet<(usize, usize)> = BTreeSet::new();

    let mut prefix = None;
    let mut cached_height_by_rock = BTreeMap::<usize, usize>::new();

    let mut tick = 0usize;
    for n_rock in 0usize.. {
        let mut left = 2usize;
        let (mut bottom, _) = *tableau.iter().last().unwrap_or_else(|| &(0, 0));
        bottom += 4;
        loop {
            let shift = values.as_bytes()[tick % (values.len() - 1)];
            let block = Block::from(n_rock);
            let projection = block.project(left, bottom);
            match shift {
                60 => {
                    let against_left_wall = projection.iter().any(|&(_, l)| l == 0);
                    if !against_left_wall {
                        let would_sweep_into = projection
                            .iter()
                            .any(|&(b, l)| tableau.contains(&(b, l - 1)));
                        if !would_sweep_into {
                            left -= 1;
                        }
                    }
                }
                62 => {
                    let against_right_wall = projection.iter().any(|&(_, l)| l == 6);
                    if !against_right_wall {
                        let would_sweep_into = projection
                            .iter()
                            .any(|&(b, l)| tableau.contains(&(b, l + 1)));
                        if !would_sweep_into {
                            left += 1;
                        }
                    }
                }
                _ => unreachable!(),
            }
            tick += 1;
            let projection = block.project(left, bottom);
            let will_lay = bottom == 1
                || projection
                    .iter()
                    .any(|&(b, l)| tableau.contains(&(b - 1, l)));
            if will_lay {
                tableau.extend(projection);

                cached_height_by_rock.insert(n_rock, tableau.last().unwrap().0);

                if tick % (values.len() - 1) == 1 {
                    if let Some((pre_n, pre_h)) = prefix {
                        let post_h = tableau.last().unwrap().0;
                        let post_n = n_rock;
                        let cycle_n = post_n - pre_n;
                        let cycle_h = post_h - pre_h;

                        let early_n = cycle_n - pre_n;
                        let offset_n = (N + early_n) % cycle_n;
                        let offset_h = *cached_height_by_rock.get(&(pre_n + offset_n)).unwrap();

                        return (N - pre_n) / cycle_n * cycle_h + offset_h;
                    } else {
                        let h = tableau.last().unwrap().0;
                        prefix = Some((n_rock, h));
                    }
                }

                break;
            } else {
                bottom -= 1;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 3068);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 1514285714288);
    }
}
