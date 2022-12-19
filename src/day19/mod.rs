use rayon::prelude::*;

#[derive(Clone, Copy)]
struct Amount {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl std::ops::Add<Amount> for Amount {
    type Output = Amount;

    fn add(self, rhs: Amount) -> Self::Output {
        Amount {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl std::ops::Sub<Amount> for Amount {
    type Output = Amount;

    fn sub(self, rhs: Amount) -> Self::Output {
        Amount {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

struct Blueprint {
    ore: Amount,
    clay: Amount,
    obsidian: Amount,
    geode: Amount,
}

struct State {
    time: usize,
    resources: Amount,
    robots: Amount,
}

fn parse_blueprint(line: &str) -> Blueprint {
    let mut tokens = line.split_ascii_whitespace();
    let ore_robot_cost = Amount {
        ore: tokens.nth(6).unwrap().parse().unwrap(),
        clay: 0,
        obsidian: 0,
        geode: 0,
    };
    let clay_robot_cost = Amount {
        ore: tokens.nth(5).unwrap().parse().unwrap(),
        clay: 0,
        obsidian: 0,
        geode: 0,
    };
    let obsidian_robot_cost = Amount {
        ore: tokens.nth(5).unwrap().parse().unwrap(),
        clay: tokens.nth(2).unwrap().parse().unwrap(),
        obsidian: 0,
        geode: 0,
    };
    let geode_robot_cost = Amount {
        ore: tokens.nth(5).unwrap().parse().unwrap(),
        clay: 0,
        obsidian: tokens.nth(2).unwrap().parse().unwrap(),
        geode: 0,
    };
    Blueprint {
        ore: ore_robot_cost,
        clay: clay_robot_cost,
        obsidian: obsidian_robot_cost,
        geode: geode_robot_cost,
    }
}

fn optimise(blueprint: &Blueprint, max_time: usize) -> usize {
    // We can produce at most a robot a minute
    // To do so, it is sufficient having a matching production of each of its requirements
    // Thus, for each resource kind, producing more than required by the most expensive machine is wasteful
    let max_ore_robots = [
        blueprint.ore.ore,
        blueprint.clay.ore,
        blueprint.obsidian.ore,
        blueprint.geode.ore,
    ]
    .into_iter()
    .max()
    .unwrap();
    let max_clay_robots = [
        blueprint.ore.clay,
        blueprint.clay.clay,
        blueprint.obsidian.clay,
        blueprint.geode.clay,
    ]
    .into_iter()
    .max()
    .unwrap();
    let max_obsidian_robots = [
        blueprint.ore.obsidian,
        blueprint.clay.obsidian,
        blueprint.obsidian.obsidian,
        blueprint.geode.obsidian,
    ]
    .into_iter()
    .max()
    .unwrap();
    let max_robots = Amount {
        ore: max_ore_robots,
        clay: max_clay_robots,
        obsidian: max_obsidian_robots,
        geode: usize::MAX,
    };

    let mut max_geodes = 0;
    simulate_recursively(
        &blueprint,
        &max_robots,
        &State {
            time: 0,
            resources: Amount {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            robots: Amount {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
        },
        max_time,
        &mut max_geodes,
    );
    max_geodes
}

fn simulate_recursively(
    blueprint: &Blueprint,
    max_robots: &Amount,
    state: &State,
    max_time: usize,
    max_geodes: &mut usize,
) {
    if state.time == max_time {
        return;
    }

    let mut state = State {
        time: state.time + 1,
        resources: state.resources + state.robots,
        robots: state.robots,
    };
    if state.resources.geode > *max_geodes {
        *max_geodes = state.resources.geode;
    }
    // if we could build geode robots every turn, would we catch up with the the cached max?
    if {
        let mut g = state.resources.geode;
        for t in 0.. {
            if state.time + t == max_time {
                break;
            }
            g = g + state.resources.geode + state.robots.geode + t;
        }
        g < *max_geodes
    } {
        return;
    }

    if state.robots.ore < max_robots.ore {
        if blueprint.ore.ore + state.robots.ore <= state.resources.ore {
            state.resources = state.resources - blueprint.ore;
            state.robots.ore += 1;
            simulate_recursively(blueprint, max_robots, &state, max_time, max_geodes);
            state.robots.ore -= 1;
            state.resources = state.resources + blueprint.ore;
        }
    }

    if state.robots.clay < max_robots.clay {
        if blueprint.clay.ore + state.robots.ore <= state.resources.ore {
            state.resources = state.resources - blueprint.clay;
            state.robots.clay += 1;
            simulate_recursively(blueprint, max_robots, &state, max_time, max_geodes);
            state.robots.clay -= 1;
            state.resources = state.resources + blueprint.clay;
        }
    }

    if state.robots.obsidian < max_robots.obsidian {
        if blueprint.obsidian.ore + state.robots.ore <= state.resources.ore
            && blueprint.obsidian.clay + state.robots.clay <= state.resources.clay
        {
            state.resources = state.resources - blueprint.obsidian;
            state.robots.obsidian += 1;
            simulate_recursively(blueprint, max_robots, &state, max_time, max_geodes);
            state.robots.obsidian -= 1;
            state.resources = state.resources + blueprint.obsidian;
        }
    }

    if blueprint.geode.ore + state.robots.ore <= state.resources.ore
        && blueprint.geode.obsidian + state.robots.obsidian <= state.resources.obsidian
    {
        state.resources = state.resources - blueprint.geode;
        state.robots.geode += 1;
        simulate_recursively(blueprint, max_robots, &state, max_time, max_geodes);
        state.robots.geode -= 1;
        state.resources = state.resources + blueprint.geode;
    }

    simulate_recursively(blueprint, max_robots, &state, max_time, max_geodes);
}

pub fn star_one() -> usize {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let blueprints = values.lines().map(parse_blueprint).collect::<Vec<_>>();

    blueprints
        .par_iter()
        .enumerate()
        .map(|(index, blueprint)| optimise(blueprint, 24) * (index + 1))
        .sum()
}

pub fn star_two() -> usize {
    #[cfg(test)]
    let values = include_str!("mock.txt");
    #[cfg(not(test))]
    let values = include_str!("input.txt");

    let blueprints = values.lines().map(parse_blueprint).collect::<Vec<_>>();

    blueprints
        .par_iter()
        .take(3)
        .map(|blueprint| optimise(blueprint, 32))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(), 33);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(), 3472);
    }
}
