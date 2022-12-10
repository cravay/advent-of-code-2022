use std::collections::HashSet;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Motion {
    direction: Direction,
    steps: i32,
}

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<Motion> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');

            Motion {
                direction: match parts.next().unwrap() {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!(),
                },
                steps: parts.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

fn get_new_knot_pos((x, y): (i32, i32), (nx, ny): (i32, i32)) -> (i32, i32) {
    match (nx - x, ny - y) {
        (2, 0) => (nx - 1, ny),
        (-2, 0) => (nx + 1, ny),
        (0, 2) => (nx, ny - 1),
        (0, -2) => (nx, ny + 1),
        (dx, dy) if dx.abs() <= 1 && dy.abs() <= 1 => (x, y),
        (dx, dy) => (x + dx.clamp(-1, 1), y + dy.clamp(-1, 1)),
    }
}

fn count_visited_tiles<const L: usize>(motions: &[Motion]) -> usize {
    let mut knots = [(0, 0); L];
    let mut visited_tiles = HashSet::<(i32, i32)>::new();

    for Motion { direction, steps } in motions {
        for _ in 0..*steps {
            let head = &mut knots[0];

            match direction {
                Direction::Up => head.1 -= 1,
                Direction::Down => head.1 += 1,
                Direction::Left => head.0 -= 1,
                Direction::Right => head.0 += 1,
            }

            for i in 1..L {
                knots[i] = get_new_knot_pos(knots[i], knots[i - 1]);
            }

            visited_tiles.insert(knots[L - 1]);
        }
    }

    visited_tiles.len()
}

#[aoc(day9, part1)]
pub fn part1(input: &[Motion]) -> usize {
    count_visited_tiles::<2>(input)
}

#[aoc(day9, part2)]
pub fn part2(input: &[Motion]) -> usize {
    count_visited_tiles::<10>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let test_input = input_generator(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        );

        assert_eq!(part1(&test_input), 13);
        assert_eq!(part2(&test_input), 1);
    }

    #[test]
    fn sample2() {
        let test_input = input_generator(
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
        );

        assert_eq!(part2(&test_input), 36);
    }
}
