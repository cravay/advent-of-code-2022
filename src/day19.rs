use std::iter::zip;

use itertools::izip;

type Resources = [usize; 4]; // ore, clay, obsidian, geode
type Blueprint = [Resources; 4]; // resource type: index, label: index + 1, cost: value

#[aoc_generator(day19)]
fn input_generator(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|line| {
            let i: Vec<_> = line.split(' ').filter_map(|v| v.parse().ok()).collect();
            assert_eq!(i.len(), 6);
            [
                [i[0], 0x00, 0x00, 0x00], // ore robot
                [i[1], 0x00, 0x00, 0x00], // clay robot
                [i[2], i[3], 0x00, 0x00], // obsidian robot
                [i[4], 0x00, i[5], 0x00], // geode robot
            ]
        })
        .collect()
}

fn get_max_cost(blueprint: &Blueprint) -> Resources {
    let mut max_cost = blueprint.iter().fold([0; 4], |a, b| {
        zip(a, b)
            .map(|(a, b)| a.max(*b))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    });
    max_cost[3] = usize::MAX;
    max_cost
}

fn get_max_geodes(
    blueprint: &Blueprint,
    robots: &mut Resources,
    max_cost: &Resources,
    resources: Resources,
    minutes: usize,
) -> usize {
    let geodes = resources[3];
    let geode_robots = robots[3];
    let mut max = geodes + minutes * geode_robots;

    for (i, cost) in blueprint.iter().enumerate().rev() {
        if resources[i] / minutes + robots[i] >= max_cost[i] {
            return max;
        }

        let wait_minutes = izip!(resources.iter(), robots.iter(), cost.iter())
            .map(|(&resource, &robots, &cost)| {
                if resource >= cost {
                    return 0;
                }

                if robots == 0 {
                    return usize::MAX;
                }

                (cost - resource + robots - 1) / robots
            })
            .max()
            .unwrap();

        if wait_minutes >= minutes - 1 {
            continue;
        }

        let wait_minutes = wait_minutes + 1;
        let minutes = minutes - wait_minutes;

        let mut resources = resources;

        for (resource, &robot, &cost) in izip!(resources.iter_mut(), robots.iter(), cost.iter()) {
            *resource = *resource + robot * wait_minutes - cost;
        }

        robots[i] += 1;

        max = max.max(get_max_geodes(
            blueprint, robots, max_cost, resources, minutes,
        ));

        robots[i] -= 1;
    }

    max
}

#[aoc(day19, part1)]
fn part1(input: &[Blueprint]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(i, blueprint)| {
            (i + 1)
                * get_max_geodes(
                    blueprint,
                    &mut [1, 0, 0, 0],
                    &get_max_cost(blueprint),
                    [0, 0, 0, 0],
                    24,
                )
        })
        .sum()
}

#[aoc(day19, part2)]
fn part2(input: &[Blueprint]) -> usize {
    input
        .iter()
        .take(3)
        .map(|blueprint| {
            get_max_geodes(
                blueprint,
                &mut [1, 0, 0, 0],
                &get_max_cost(blueprint),
                [0, 0, 0, 0],
                32,
            )
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let test_input = input_generator(
            "\
Blueprint 1:\
  Each ore robot costs 4 ore.\
  Each clay robot costs 2 ore.\
  Each obsidian robot costs 3 ore and 14 clay.\
  Each geode robot costs 2 ore and 7 obsidian.\
\n\
Blueprint 2:\
  Each ore robot costs 2 ore.\
  Each clay robot costs 3 ore.\
  Each obsidian robot costs 3 ore and 8 clay.\
  Each geode robot costs 3 ore and 12 obsidian.",
        );

        assert_eq!(part1(&test_input), 33);
        assert_eq!(part2(&test_input), 3472);
    }
}
