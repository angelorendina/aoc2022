use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;

struct Valve {
    id: String,
    flow: usize,
    next: BTreeSet<String>,
}

pub fn star_one() -> usize {
    struct History<'a> {
        opened: BTreeSet<&'a str>,
        current: &'a Valve,
        total: usize,
    }

    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let map = values
        .lines()
        .map(|line| {
            let line = line.strip_prefix("Valve ").unwrap();
            let (id, line) = line.split_once(' ').unwrap();
            let line = line.strip_prefix("has flow rate=").unwrap();
            let (flow, line) = line.split_once(';').unwrap();
            let line = line.strip_prefix(" tunnel").unwrap();
            let line = line
                .strip_prefix("s ")
                .or_else(|| line.strip_prefix(' '))
                .unwrap();
            let line = line.strip_prefix("lead").unwrap();
            let line = line
                .strip_prefix("s ")
                .or_else(|| line.strip_prefix(' '))
                .unwrap();
            let line = line.strip_prefix("to valve").unwrap();
            let line = line
                .strip_prefix("s ")
                .or_else(|| line.strip_prefix(' '))
                .unwrap();
            let next = line.split(", ").map(|s| s.to_string()).collect();
            (
                id.to_string(),
                Valve {
                    id: id.to_string(),
                    flow: flow.parse().unwrap(),
                    next,
                },
            )
        })
        .collect::<BTreeMap<_, _>>();

    let mut past = vec![History {
        opened: BTreeSet::new(),
        current: map.get("AA").unwrap(),
        total: 0,
    }];

    for _ in 1..30 {
        let mut future = BTreeMap::<&str, HashMap<BTreeSet<&str>, usize>>::new();
        for old in past.drain(0..) {
            if old.current.flow > 0 {
                if !old.opened.contains(old.current.id.as_str()) {
                    let mut new_opened = old.opened.clone();
                    new_opened.insert(old.current.id.as_str());
                    let new_total = old.total
                        + new_opened
                            .iter()
                            .map(|&id| map.get(id).unwrap().flow)
                            .sum::<usize>();

                    let best_total = future
                        .entry(old.current.id.as_str())
                        .or_default()
                        .entry(new_opened)
                        .or_default();
                    if new_total > *best_total {
                        *best_total = new_total;
                    }
                }
            }

            let total = old.total
                + old
                    .opened
                    .iter()
                    .map(|&id| map.get(id).unwrap().flow)
                    .sum::<usize>();
            for next in &old.current.next {
                let best_total = future
                    .entry(next.as_str())
                    .or_default()
                    .entry(old.opened.clone())
                    .or_default();
                if total > *best_total {
                    *best_total = total;
                }
            }
        }

        for (current, by_open_valves) in future {
            for (opened, total) in by_open_valves {
                past.push(History {
                    opened,
                    current: map.get(current).unwrap(),
                    total,
                });
            }
        }
    }

    past.into_iter().map(|h| h.total).max().unwrap()
}

pub fn star_two() -> usize {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let map = values
        .lines()
        .map(|line| {
            let line = line.strip_prefix("Valve ").unwrap();
            let (id, line) = line.split_once(' ').unwrap();
            let line = line.strip_prefix("has flow rate=").unwrap();
            let (flow, line) = line.split_once(';').unwrap();
            let line = line.strip_prefix(" tunnel").unwrap();
            let line = line
                .strip_prefix("s ")
                .or_else(|| line.strip_prefix(' '))
                .unwrap();
            let line = line.strip_prefix("lead").unwrap();
            let line = line
                .strip_prefix("s ")
                .or_else(|| line.strip_prefix(' '))
                .unwrap();
            let line = line.strip_prefix("to valve").unwrap();
            let line = line
                .strip_prefix("s ")
                .or_else(|| line.strip_prefix(' '))
                .unwrap();
            let next = line.split(", ").map(|s| s.to_string()).collect();
            (
                id.to_string(),
                Valve {
                    id: id.to_string(),
                    flow: flow.parse().unwrap(),
                    next,
                },
            )
        })
        .collect::<BTreeMap<_, _>>();

    let mut past: HashMap<BTreeSet<&str>, BTreeMap<(&str, &str), usize>> = HashMap::new();
    let start = map.get("AA").unwrap().id.as_ref();
    past.entry(BTreeSet::new())
        .or_insert_with(|| BTreeMap::new())
        .insert((start, start), 0);

    for _ in 1..26 {
        let mut future: HashMap<BTreeSet<&str>, BTreeMap<(&str, &str), usize>> = HashMap::new();

        for (opened, total_by_position) in past {
            for ((me, you), total) in total_by_position {
                let (me, you) = if me < you { (me, you) } else { (you, me) };
                let mine = map.get(me).unwrap();
                let yours = map.get(you).unwrap();

                if mine.flow > 0 && !opened.contains(me) {
                    let mut new_opened = opened.clone();
                    new_opened.insert(me);

                    let new_total = total
                        + new_opened
                            .iter()
                            .map(|id| map.get(*id).unwrap().flow)
                            .sum::<usize>();
                    if yours.flow > 0 && !new_opened.contains(you) {
                        let mut new_opened = new_opened.clone();
                        new_opened.insert(you);

                        let new_total = new_total + yours.flow;
                        let best_total = future
                            .entry(new_opened)
                            .or_default()
                            .entry((me, you))
                            .or_default();
                        if new_total > *best_total {
                            *best_total = new_total;
                        }
                    }

                    for your_next in &yours.next {
                        let best_total = future
                            .entry(new_opened.clone())
                            .or_default()
                            .entry((me, your_next))
                            .or_default();
                        if new_total > *best_total {
                            *best_total = new_total;
                        }
                    }
                }

                for my_next in &mine.next {
                    let new_total = total
                        + opened
                            .iter()
                            .map(|id| map.get(*id).unwrap().flow)
                            .sum::<usize>();
                    if yours.flow > 0 && !opened.contains(you) {
                        let mut new_opened = opened.clone();
                        new_opened.insert(you);

                        let new_total = new_total + yours.flow;
                        let best_total = future
                            .entry(new_opened)
                            .or_default()
                            .entry((my_next, you))
                            .or_default();
                        if new_total > *best_total {
                            *best_total = new_total;
                        }
                    }

                    for your_next in &yours.next {
                        let best_total = future
                            .entry(opened.clone())
                            .or_default()
                            .entry((my_next, your_next))
                            .or_default();
                        if new_total > *best_total {
                            *best_total = new_total;
                        }
                    }
                }
            }
        }

        past = future;
    }

    *past
        .values()
        .map(|h| h.values().max().unwrap())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 1651);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 1707);
    }
}
