use std::collections::HashMap;

type Rock = [u8; 4];

const ROCKS: [Rock; 5] = [
    [
        0b0000000, // .......
        0b0000000, // .......
        0b0000000, // .......
        0b0011110, // ..####.
    ],
    [
        0b0000000, // .......
        0b0001000, // ...#...
        0b0011100, // ..###..
        0b0001000, // ...#...
    ],
    [
        0b0000000, // .......
        0b0000100, // ....#..
        0b0000100, // ....#..
        0b0011100, // ..###..
    ],
    [
        0b0010000, // ..#....
        0b0010000, // ..#....
        0b0010000, // ..#....
        0b0010000, // ..#....
    ],
    [
        0b0000000, // .......
        0b0000000, // .......
        0b0011000, // ..##...
        0b0011000, // ..##...
    ],
];

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

const SNAPSHOT_SIZE: usize = 32;

struct Snapshot {
    height: usize,
    rock_count: usize,
    top_rows: [u8; SNAPSHOT_SIZE],
}

#[aoc_generator(day17)]
fn input_generator(input: &str) -> Vec<Direction> {
    input
        .chars()
        .filter(|&c| c == '<' || c == '>')
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => unreachable!(),
        })
        .collect()
}

fn rock_is_colliding(rock: &mut Rock, position: usize, chamber: &[u8]) -> bool {
    for i in 0..rock.len() {
        if chamber[position - i] & rock[i] != 0 {
            return true;
        }
    }

    false
}

fn add_rock_to_chamber(rock: &mut Rock, position: usize, chamber: &mut [u8]) {
    for i in 0..rock.len() {
        chamber[position - i] |= rock[i];
    }
}

fn move_rock_left(rock: &mut Rock) {
    rock.iter_mut().for_each(|row| *row <<= 1);
}

fn move_rock_right(rock: &mut Rock) {
    rock.iter_mut().for_each(|row| *row >>= 1);
}

fn try_move_rock_left(rock: &mut Rock, position: usize, chamber: &[u8]) {
    if rock.iter().any(|&row| row & 0b1000000 != 0) {
        return;
    }

    move_rock_left(rock);

    if rock_is_colliding(rock, position, chamber) {
        move_rock_right(rock);
    }
}

fn try_move_rock_right(rock: &mut Rock, position: usize, chamber: &[u8]) {
    if rock.iter().any(|&row| row & 0b0000001 != 0) {
        return;
    }

    move_rock_right(rock);

    if rock_is_colliding(rock, position, chamber) {
        move_rock_left(rock);
    }
}

fn drop_rock<I>(rock: &mut Rock, mut position: usize, chamber: &mut Vec<u8>, directions: &mut I)
where
    I: Iterator<Item = (usize, Direction)>,
{
    while chamber.len() <= position {
        chamber.push(0);
    }

    loop {
        match directions.next().unwrap().1 {
            Direction::Left => try_move_rock_left(rock, position, chamber),
            Direction::Right => try_move_rock_right(rock, position, chamber),
        }

        position -= 1;

        if position < 3 || rock_is_colliding(rock, position, chamber) {
            position += 1;
            add_rock_to_chamber(rock, position, chamber);
            return;
        }
    }
}

fn get_tower_height(directions: &[Direction], mut rock_count: usize) -> usize {
    let mut height: usize = 0;
    let mut extra_height: usize = 0;
    let mut chamber = Vec::new();
    let mut snapshots: HashMap<(usize, usize), Snapshot> = HashMap::new();
    let mut rocks = ROCKS.iter().copied().enumerate().cycle();
    let mut directions = directions.iter().copied().enumerate().cycle().peekable();

    while rock_count > 0 {
        let (rock_i, mut rock) = rocks.next().unwrap();
        let position = height + 2 + rock.len();

        drop_rock(&mut rock, position, &mut chamber, &mut directions);
        rock_count -= 1;

        while chamber[height] != 0 {
            height += 1;
        }

        if height < SNAPSHOT_SIZE {
            continue;
        }

        let direction_i = directions.peek().unwrap().0;
        let top_rows = chamber[height - SNAPSHOT_SIZE..height].try_into().unwrap();

        if let Some(snapshot) = snapshots.get(&(rock_i, direction_i)) {
            if top_rows == snapshot.top_rows {
                let height_diff = height - snapshot.height;
                let rock_count_diff = snapshot.rock_count - rock_count;
                let cycles = rock_count / rock_count_diff;
                rock_count -= cycles * rock_count_diff;
                extra_height += cycles * height_diff;
            }
        }

        snapshots.insert(
            (rock_i, direction_i),
            Snapshot {
                height,
                top_rows,
                rock_count,
            },
        );
    }

    height + extra_height
}

#[aoc(day17, part1)]
fn part1(input: &[Direction]) -> usize {
    get_tower_height(input, 2022)
}

#[aoc(day17, part2)]
fn part2(input: &[Direction]) -> usize {
    get_tower_height(input, 1000000000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let test_input = input_generator(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");

        assert_eq!(part1(&test_input), 3068);
        assert_eq!(part2(&test_input), 1514285714288);
    }
}
