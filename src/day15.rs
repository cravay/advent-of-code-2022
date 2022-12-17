use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Sensor {
    position: (i32, i32),
    closest_beacon: (i32, i32),
    closest_beacon_distance: u32,
}

fn manhattan_distance(a: &(i32, i32), b: &(i32, i32)) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

impl Sensor {
    fn new(position: (i32, i32), closest_beacon: (i32, i32)) -> Sensor {
        Sensor {
            position,
            closest_beacon,
            closest_beacon_distance: manhattan_distance(&position, &closest_beacon),
        }
    }
}

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Vec<Sensor> {
    Regex::new(r"Sensor at x=(\d+), y=(\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
        .unwrap()
        .captures_iter(input)
        .map(|cap| {
            Sensor::new(
                (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                (cap[3].parse().unwrap(), cap[4].parse().unwrap()),
            )
        })
        .collect()
}

fn normalize_ranges(ranges: &mut [(i32, i32)]) -> Vec<(i32, i32)> {
    ranges.sort();

    let mut ranges = ranges.iter().peekable();
    let mut normalized_ranges = Vec::with_capacity(ranges.len());

    while let Some(&(from, mut to)) = ranges.next() {
        while let Some(next) = ranges.peek() {
            if next.0 <= to + 1 {
                to = to.max(next.1);
                ranges.next();
            } else {
                break;
            }
        }

        normalized_ranges.push((from, to));
    }

    normalized_ranges
}

fn get_beacon_ranges(sensors: &[Sensor], y: i32) -> Vec<(i32, i32)> {
    let mut ranges = Vec::with_capacity(sensors.len());

    for sensor in sensors {
        let y_diff = sensor.position.1.abs_diff(y);
        let side_len = sensor.closest_beacon_distance as i32 - y_diff as i32;

        if side_len < 0 {
            continue;
        }

        let min_x = sensor.position.0 - side_len;
        let max_x = sensor.position.0 + side_len;

        ranges.push((min_x, max_x));
    }

    normalize_ranges(&mut ranges)
}

fn count_range_elements(ranges: &[(i32, i32)]) -> i32 {
    ranges.iter().map(|(from, to)| to - from + 1).sum()
}

fn count_positions_without_a_beacon(sensors: &[Sensor], y: i32) -> i32 {
    let ranges = get_beacon_ranges(sensors, y);
    let mut count = count_range_elements(&ranges);

    sensors
        .iter()
        .filter(|sensor| sensor.closest_beacon.1 == y)
        .map(|sensor| sensor.closest_beacon.0)
        .unique()
        .for_each(|_| count -= 1);

    count
}

#[aoc(day15, part1)]
fn part1(input: &[Sensor]) -> i32 {
    count_positions_without_a_beacon(input, 2000000)
}

fn get_tuning_frequency(sensors: &[Sensor], max: i32) -> Option<i64> {
    for y in 0..=max {
        for (prev, curr) in get_beacon_ranges(sensors, y).iter().tuple_windows() {
            let x = prev.1 + 1;

            if x == curr.0 - 1 && 0 <= x && x <= max {
                return Some(x as i64 * 4000000 + y as i64);
            }
        }
    }

    None
}

#[aoc(day15, part2)]
fn part2(input: &[Sensor]) -> i64 {
    get_tuning_frequency(input, 4000000).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let test_input = input_generator(include_str!("../input/2022/day15.sample.txt"));

        assert_eq!(count_positions_without_a_beacon(&test_input, 10), 26);
        assert_eq!(get_tuning_frequency(&test_input, 20), Some(56000011));
    }
}
