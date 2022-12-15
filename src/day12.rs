// Partially based on https://doc.rust-lang.org/std/collections/binary_heap/index.html#examples

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Clone)]
pub struct HeightMap {
    nodes: Vec<u8>,
    size: (usize, usize),
    start: (usize, usize),
    end: (usize, usize),
}

impl HeightMap {
    fn get(&self, (x, y): (usize, usize)) -> u8 {
        *self.nodes.get(y * self.size.0 + x).unwrap()
    }

    fn get_neighbor_positions(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbor_positions = vec![];
        let height = self.get((x, y));

        let mut potential_neighbors = vec![];

        if x > 0 {
            potential_neighbors.push((x - 1, y));
        }

        if x < self.size.0 - 1 {
            potential_neighbors.push((x + 1, y));
        }

        if y > 0 {
            potential_neighbors.push((x, y - 1));
        }

        if y < self.size.1 - 1 {
            potential_neighbors.push((x, y + 1));
        }

        for neighbor in potential_neighbors {
            let neighbor_height = self.get(neighbor);
            if height <= neighbor_height || height - neighbor_height == 1 {
                neighbor_positions.push(neighbor);
            }
        }

        neighbor_positions
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day12)]
fn input_generator(input: &str) -> HeightMap {
    let mut start = 0;
    let mut end = 0;

    let nodes: Vec<u8> = input
        .chars()
        .filter(|c| c.is_alphabetic())
        .enumerate()
        .map(|(i, c)| match c {
            'S' => {
                start = i;
                0
            }
            'E' => {
                end = i;
                25
            }
            'a'..='z' => c as u8 - b'a',
            _ => panic!(),
        })
        .collect();
    let width = input.lines().next().unwrap().len();
    let height = nodes.len() / width;

    HeightMap {
        nodes,
        size: (width, height),
        start: (start % width, start / width),
        end: (end % width, end / width),
    }
}

fn get_distance<F>(height_map: &HeightMap, predicate: F) -> Option<usize>
where
    F: Fn(&(usize, usize), u8) -> bool,
{
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(height_map.end, 0);
    heap.push(State {
        cost: 0,
        position: height_map.end,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if predicate(&position, height_map.get(position)) {
            return Some(cost);
        }

        if cost > dist.get(&position).copied().unwrap_or(usize::MAX) {
            continue;
        }

        for neighbor_position in height_map.get_neighbor_positions(position) {
            let next = State {
                cost: cost + 1,
                position: neighbor_position,
            };

            if next.cost < dist.get(&neighbor_position).copied().unwrap_or(usize::MAX) {
                heap.push(next);
                dist.insert(next.position, next.cost);
            }
        }
    }

    None
}

#[aoc(day12, part1)]
pub fn part1(height_map: &HeightMap) -> usize {
    get_distance(height_map, |position, _| *position == height_map.start).unwrap()
}

#[aoc(day12, part2)]
pub fn part2(height_map: &HeightMap) -> usize {
    get_distance(height_map, |_, height| height == 0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let test_input = input_generator(
            "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
        );

        assert_eq!(part1(&test_input), 31);
        assert_eq!(part2(&test_input), 29);
    }
}
