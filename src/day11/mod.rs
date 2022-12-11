use std::collections::VecDeque;

type Worry = u64;

struct Monkey {
    items: VecDeque<Worry>,
    op: Box<dyn Fn(Worry) -> Worry>,
    test: Worry,
    targets: [usize; 2],
}

impl Monkey {
    fn new(
        line_items: &str,
        line_op: &str,
        line_test: &str,
        line_true: &str,
        line_false: &str,
    ) -> Self {
        let items = line_items
            .split_whitespace()
            .skip(2)
            .map(|item| {
                item.strip_suffix(",")
                    .unwrap_or(item)
                    .parse::<Worry>()
                    .unwrap()
            })
            .collect::<VecDeque<_>>();
        let op = {
            let mut ops = line_op.split_whitespace().skip(4);
            let sign = match ops.next().unwrap() {
                "+" => "+",
                "*" => "*",
                _ => unreachable!(),
            };
            let op = ops.next().unwrap().parse::<Worry>().ok();
            Box::new(move |old: Worry| match sign {
                "+" => old + op.unwrap_or(old),
                "*" => old * op.unwrap_or(old),
                _ => unreachable!(),
            })
        };
        let test = line_test
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<Worry>()
            .unwrap();
        let if_true = line_true
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let if_false = line_false
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        Self {
            items,
            op,
            test,
            targets: [if_true, if_false],
        }
    }
}

pub fn star_one() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut monkeys = vec![];

    let mut lines = values.lines();
    while let Some(line) = lines.next() {
        if line.starts_with("Monkey") {
            monkeys.push(Monkey::new(
                lines.next().unwrap(),
                lines.next().unwrap(),
                lines.next().unwrap(),
                lines.next().unwrap(),
                lines.next().unwrap(),
            ));
        }
    }

    let mut inspections = monkeys.iter().map(|_| 0u64).collect::<Vec<_>>();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let mut passes = VecDeque::<(usize, Worry)>::new();
            let monkey = monkeys.get_mut(i).unwrap();
            while let Some(item) = monkey.items.pop_front() {
                let worry = (monkey.op)(item) / 3;
                let target = if worry % monkey.test == 0 {
                    monkey.targets[0]
                } else {
                    monkey.targets[1]
                };
                passes.push_back((target, worry));
                *inspections.get_mut(i).unwrap() += 1;
            }
            while let Some((target, worry)) = passes.pop_front() {
                monkeys.get_mut(target).unwrap().items.push_back(worry);
            }
        }
    }

    inspections.sort();
    inspections
        .into_iter()
        .rev()
        .take(2)
        .reduce(|biz, x| biz * x)
        .unwrap()
}

pub fn star_two() -> u64 {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let mut monkeys = vec![];

    let mut lines = values.lines();
    while let Some(line) = lines.next() {
        if line.starts_with("Monkey") {
            monkeys.push(Monkey::new(
                lines.next().unwrap(),
                lines.next().unwrap(),
                lines.next().unwrap(),
                lines.next().unwrap(),
                lines.next().unwrap(),
            ));
        }
    }

    let common_multiple = monkeys.iter().fold(1u64, |cm, monkey| cm * monkey.test);

    let mut inspections = monkeys.iter().map(|_| 0u64).collect::<Vec<_>>();

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            let mut passes = VecDeque::<(usize, Worry)>::new();
            let monkey = monkeys.get_mut(i).unwrap();
            while let Some(item) = monkey.items.pop_front() {
                let worry = (monkey.op)(item) % common_multiple;
                let target = if worry % monkey.test == 0 {
                    monkey.targets[0]
                } else {
                    monkey.targets[1]
                };
                passes.push_back((target, worry));
                *inspections.get_mut(i).unwrap() += 1;
            }
            while let Some((target, worry)) = passes.pop_front() {
                monkeys.get_mut(target).unwrap().items.push_back(worry);
            }
        }
    }

    inspections.sort();
    inspections
        .into_iter()
        .rev()
        .take(2)
        .reduce(|biz, x| biz * x)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 10605);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 2713310158);
    }
}
