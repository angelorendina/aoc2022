use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

struct Valve {
    flow: usize,
    next: BTreeSet<String>,
}

#[derive(Clone, Copy)]
enum Action<'a> {
    Walk(&'a str),
    Open(&'a str),
}

pub fn star_one() -> usize {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let valves = values
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
                    flow: flow.parse().unwrap(),
                    next,
                },
            )
        })
        .collect::<BTreeMap<_, _>>();

    let good_valves = valves
        .iter()
        .filter_map(|(id, valve)| {
            if valve.flow > 0 {
                Some((id.as_str(), valve))
            } else {
                None
            }
        })
        .collect::<BTreeMap<&str, &Valve>>();
    let best_flows = good_valves
        .iter()
        .map(|(id, valve)| (valve.flow, *id))
        .collect::<BTreeMap<_, _>>();

    let mut map = BTreeMap::<(&str, &str), Vec<&str>>::new();
    for start in valves.keys() {
        let start = start.as_str();
        map.insert((start, start), vec![start]);
        let mut exploration = VecDeque::from([start]);
        while let Some(id_to) = exploration.pop_front() {
            let path_so_far = map.get(&(start, id_to)).unwrap().clone();
            let valve_to = valves.get(id_to).unwrap();
            for id_next in &valve_to.next {
                let id_next = id_next.as_str();
                if !map.contains_key(&(start, id_next)) {
                    let mut new_path = path_so_far.clone();
                    new_path.push(id_next);
                    map.insert((start, id_next), new_path);
                    exploration.push_back(id_next);
                }
            }
        }
    }

    let mut max = 0;
    let mut exploration = VecDeque::<Vec<Action>>::from([vec![]]);
    while let Some(pathing) = exploration.pop_front() {
        let id_at = match pathing.last() {
            Some(Action::Open(id_at)) => *id_at,
            None => "AA",
            _ => unreachable!(),
        };

        let mut good_ids_visited = BTreeSet::new();
        let pathing_score = (0..29).fold(0, |score, i| {
            if let Some(Action::Open(id)) = pathing.get(i) {
                good_ids_visited.insert(*id);
            }
            score
                + good_ids_visited
                    .iter()
                    .map(|&id| valves.get(id).unwrap().flow)
                    .sum::<usize>()
        });
        if pathing_score > max {
            max = pathing_score;
        }

        // suboptimality
        if {
            let mut pathing = pathing.clone();
            let good_ids_visited = pathing
                .iter()
                .filter_map(|action| match action {
                    Action::Walk(_) => None,
                    Action::Open(id) => Some(*id),
                })
                .collect::<BTreeSet<_>>();
            for &good_valve in best_flows
                .values()
                .rev()
                .filter(|v| !good_ids_visited.contains(*v))
            {
                pathing.push(Action::Walk(good_valve));
                pathing.push(Action::Open(good_valve));
            }
            let mut good_ids_visited = BTreeSet::new();
            let score_if_hopping = (0..29).fold(0, |score, i| {
                if let Some(Action::Open(id)) = pathing.get(i) {
                    good_ids_visited.insert(*id);
                }
                score
                    + good_ids_visited
                        .iter()
                        .map(|&id| valves.get(id).unwrap().flow)
                        .sum::<usize>()
            });
            score_if_hopping < max
        } {
            continue;
        }

        if pathing.len() < 29 {
            for &good_id_to in good_valves.keys() {
                if !good_ids_visited.contains(good_id_to) {
                    let path_extension = map
                        .get(&(id_at, good_id_to))
                        .unwrap()
                        .into_iter()
                        .skip(1)
                        .map(|id| Action::Walk(*id));
                    let mut new_pathing = pathing.clone();
                    new_pathing.extend(path_extension);
                    new_pathing.push(Action::Open(good_id_to));
                    exploration.push_back(new_pathing);
                }
            }
        }
    }
    max
}

pub fn star_two() -> usize {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let valves = values
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
                    flow: flow.parse().unwrap(),
                    next,
                },
            )
        })
        .collect::<BTreeMap<_, _>>();

    let good_valves = valves
        .iter()
        .filter_map(|(id, valve)| {
            if valve.flow > 0 {
                Some((id.as_str(), valve))
            } else {
                None
            }
        })
        .collect::<BTreeMap<&str, &Valve>>();
    let best_flows = good_valves
        .iter()
        .map(|(id, valve)| (valve.flow, *id))
        .collect::<BTreeMap<_, _>>();

    let mut map = BTreeMap::<(&str, &str), Vec<&str>>::new();
    for start in valves.keys() {
        let start = start.as_str();
        map.insert((start, start), vec![start]);
        let mut exploration = VecDeque::from([start]);
        while let Some(id_to) = exploration.pop_front() {
            let path_so_far = map.get(&(start, id_to)).unwrap().clone();
            let valve_to = valves.get(id_to).unwrap();
            for id_next in &valve_to.next {
                let id_next = id_next.as_str();
                if !map.contains_key(&(start, id_next)) {
                    let mut new_path = path_so_far.clone();
                    new_path.push(id_next);
                    map.insert((start, id_next), new_path);
                    exploration.push_back(id_next);
                }
            }
        }
    }

    let mut max = 0;
    let mut exploration = VecDeque::<(Vec<Action>, Vec<Action>)>::from([(vec![], vec![])]);
    while let Some((my_pathing, his_pathing)) = exploration.pop_front() {
        let mut good_ids_visited = BTreeSet::new();
        let pathing_score = (0..25).fold(0, |score, i| {
            if let Some(Action::Open(id)) = my_pathing.get(i) {
                good_ids_visited.insert(*id);
            }
            if let Some(Action::Open(id)) = his_pathing.get(i) {
                good_ids_visited.insert(*id);
            }
            score
                + good_ids_visited
                    .iter()
                    .map(|&id| valves.get(id).unwrap().flow)
                    .sum::<usize>()
        });
        if pathing_score > max {
            max = pathing_score;
        }

        // suboptimality
        if {
            let mut my_pathing = my_pathing.clone();
            let mut his_pathing = his_pathing.clone();
            let mut good_ids_visited = BTreeSet::new();
            for action in &my_pathing {
                if let Action::Open(id) = action {
                    good_ids_visited.insert(*id);
                }
            }
            for action in &his_pathing {
                if let Action::Open(id) = action {
                    good_ids_visited.insert(*id);
                }
            }
            for &next_target in best_flows
                .values()
                .rev()
                .filter(|v| !good_ids_visited.contains(*v))
            {
                if my_pathing.len() <= his_pathing.len() {
                    my_pathing.push(Action::Walk(next_target));
                    my_pathing.push(Action::Open(next_target));
                } else {
                    his_pathing.push(Action::Walk(next_target));
                    his_pathing.push(Action::Open(next_target));
                }
            }

            let mut good_ids_visited = BTreeSet::new();
            let score_if_hopping = (0..25).fold(0, |score, i| {
                if let Some(Action::Open(id)) = my_pathing.get(i) {
                    good_ids_visited.insert(*id);
                }
                if let Some(Action::Open(id)) = his_pathing.get(i) {
                    good_ids_visited.insert(*id);
                }
                score
                    + good_ids_visited
                        .iter()
                        .map(|&id| valves.get(id).unwrap().flow)
                        .sum::<usize>()
            });
            score_if_hopping < max
        } {
            continue;
        }

        if my_pathing.len() < 25 || his_pathing.len() < 25 {
            if my_pathing.len() <= his_pathing.len() {
                let my_id_at = match my_pathing.last() {
                    Some(Action::Open(id_at)) => *id_at,
                    None => "AA",
                    _ => unreachable!(),
                };
                for &good_id_to in good_valves.keys() {
                    if !good_ids_visited.contains(good_id_to) {
                        let path_extension = map
                            .get(&(my_id_at, good_id_to))
                            .unwrap()
                            .into_iter()
                            .skip(1)
                            .map(|id| Action::Walk(*id));
                        let mut my_pathing = my_pathing.clone();
                        my_pathing.extend(path_extension);
                        my_pathing.push(Action::Open(good_id_to));
                        exploration.push_front((my_pathing, his_pathing.clone()));
                    }
                }
            } else {
                let his_id_at = match his_pathing.last() {
                    Some(Action::Open(id_at)) => *id_at,
                    None => "AA",
                    _ => unreachable!(),
                };
                for &good_id_to in good_valves.keys() {
                    if !good_ids_visited.contains(good_id_to) {
                        let path_extension = map
                            .get(&(his_id_at, good_id_to))
                            .unwrap()
                            .into_iter()
                            .skip(1)
                            .map(|id| Action::Walk(*id));
                        let mut his_pathing = his_pathing.clone();
                        his_pathing.extend(path_extension);
                        his_pathing.push(Action::Open(good_id_to));
                        exploration.push_front((my_pathing.clone(), his_pathing));
                    }
                }
            }
        }
    }
    max
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
