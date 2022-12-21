use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

#[derive(Clone)]
pub struct Valve {
    name: String,
    flow_rate: usize,
    connected_valves: Vec<usize>,
    distance_to_other_valves: Vec<usize>,
}

fn set_distance_to_other_valves(valves: &mut [Valve]) {
    let len = valves.len();

    for (i, valve) in valves.iter_mut().enumerate() {
        valve.distance_to_other_valves = vec![usize::MAX; len];
        valve.distance_to_other_valves[i] = 0;
    }

    let mut updated = true;

    while updated {
        updated = false;

        for valve_i in 0..len {
            for other_i in valves[valve_i].connected_valves.clone() {
                for distance_i in 0..len {
                    if distance_i == valve_i {
                        continue;
                    }

                    let distance = valves[valve_i].distance_to_other_valves[distance_i];
                    let other_distance = valves[other_i].distance_to_other_valves[distance_i];

                    if other_distance < distance - 1 {
                        valves[valve_i].distance_to_other_valves[distance_i] = other_distance + 1;
                        updated = true;
                    }
                }
            }
        }
    }
}

#[aoc_generator(day16)]
fn input_generator(input: &str) -> Vec<Valve> {
    let captures: Vec<_> =
        Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z ,]+)")
            .unwrap()
            .captures_iter(input)
            .collect();

    let mut name_to_index = HashMap::new();

    for (i, cap) in captures.iter().enumerate() {
        name_to_index.insert(cap[1].to_owned(), i);
    }

    let mut valves: Vec<Valve> = captures
        .iter()
        .map(|cap| Valve {
            name: cap[1].to_owned(),
            flow_rate: cap[2].parse().unwrap(),
            connected_valves: cap[3]
                .split(", ")
                .map(|name| name_to_index.get(name).copied().unwrap())
                .collect(),
            distance_to_other_valves: Vec::new(),
        })
        .collect();

    set_distance_to_other_valves(&mut valves);

    valves
}

#[derive(Clone, Copy)]
struct Agent {
    position: usize,
    sleep: usize,
}

fn get_max_released_pressure<const L: usize>(
    valves: &mut [Valve],
    mut agents: [Agent; L],
    minutes: usize,
) -> usize {
    agents.sort_by(|a, b| a.sleep.cmp(&b.sleep));

    let Agent {
        position, sleep, ..
    } = agents[0];

    if sleep >= minutes {
        return 0;
    }

    for mut agent in agents.iter_mut() {
        agent.sleep -= sleep;
    }

    let minutes = minutes - sleep;

    let mut max_released_pressure = 0;

    for next_position in 0..valves.len() {
        let flow_rate = valves[next_position].flow_rate;

        if flow_rate == 0 {
            continue;
        }

        let distance = valves[position].distance_to_other_valves[next_position];

        if distance == usize::MAX {
            continue;
        }

        let cost = 1 + distance;

        if cost >= minutes {
            continue;
        }

        agents[0] = Agent {
            position: next_position,
            sleep: cost,
        };

        valves[next_position].flow_rate = 0;

        let released_pressure =
            (minutes - cost) * flow_rate + get_max_released_pressure(valves, agents, minutes);

        valves[next_position].flow_rate = flow_rate;

        max_released_pressure = max_released_pressure.max(released_pressure);
    }

    agents[0].sleep = usize::MAX;
    let released_pressure = get_max_released_pressure(valves, agents, minutes);
    max_released_pressure = max_released_pressure.max(released_pressure);

    max_released_pressure
}

fn get_start_position(valves: &[Valve]) -> usize {
    valves
        .iter()
        .find_position(|valve| valve.name == "AA")
        .unwrap()
        .0
}

#[aoc(day16, part1)]
fn part1(input: &[Valve]) -> usize {
    let start_position = get_start_position(input);
    get_max_released_pressure(
        &mut input.to_vec(),
        [Agent {
            position: start_position,
            sleep: 0,
        }],
        30,
    )
}

#[aoc(day16, part2)]
fn part2(input: &[Valve]) -> usize {
    let start_position = get_start_position(input);
    get_max_released_pressure(
        &mut input.to_vec(),
        [
            Agent {
                position: start_position,
                sleep: 0,
            },
            Agent {
                position: start_position,
                sleep: 0,
            },
        ],
        26,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let test_input = input_generator(include_str!("../input/2022/day16.sample.txt"));

        assert_eq!(part1(&test_input), 1651);
        assert_eq!(part2(&test_input), 1707);
    }
}
