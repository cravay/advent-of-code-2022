use std::fmt::{self, Display};

type Line = Vec<(usize, usize)>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Rock,
    Air,
    Sand,
}

struct Map {
    offset: (usize, usize),
    size: (usize, usize),
    source: (usize, usize),
    tiles: Vec<Tile>,
}

impl Map {
    fn from_lines(lines: &[Line], source: (usize, usize), draw_floor_line: bool) -> Map {
        let mut min = source;
        let mut max = source;

        for line in lines {
            for (x, y) in line {
                min.0 = min.0.min(*x);
                min.1 = min.1.min(*y);
                max.0 = max.0.max(*x);
                max.1 = max.1.max(*y);
            }
        }

        let floor_line = if draw_floor_line {
            let y = max.1 + 2;
            let height = y - source.1;
            let from_x = source.0 - height;
            let to_x = source.0 + height;

            max.1 += 2;
            min.0 = min.0.min(from_x);
            max.0 = max.0.max(to_x);

            Some(vec![(from_x, y), (to_x, y)])
        } else {
            None
        };

        let offset = min;
        let size = ((max.0 - offset.0 + 1), (max.1 - offset.1 + 1));
        let tiles = vec![Tile::Air; size.0 * size.1];

        let mut map = Map {
            offset,
            size,
            source,
            tiles,
        };

        for line in lines {
            map.draw_line(line);
        }

        if let Some(line) = floor_line {
            map.draw_line(&line);
        }

        map
    }

    fn draw_line(&mut self, line: &Line) {
        let mut points = line.iter();
        let mut from = points.next().unwrap();

        for to in points {
            self.draw_line_segment(from, to);
            from = to;
        }
    }

    fn draw_line_segment(&mut self, from: &(usize, usize), to: &(usize, usize)) {
        let step = (
            (to.0 as isize - from.0 as isize).clamp(-1, 1),
            (to.1 as isize - from.1 as isize).clamp(-1, 1),
        );

        let mut position = *from;

        loop {
            self.set_tile(&position, Tile::Rock);

            if position == *to {
                break;
            }

            position = (
                (position.0 as isize + step.0) as usize,
                (position.1 as isize + step.1) as usize,
            );
        }
    }

    fn get_tile_index(&self, position: &(usize, usize)) -> usize {
        (position.1 - self.offset.1) * self.size.0 + (position.0 - self.offset.0)
    }

    fn contains_position(&self, position: &(usize, usize)) -> bool {
        (self.offset.0..self.offset.0 + self.size.0).contains(&position.0)
            && (self.offset.1..self.offset.1 + self.size.1).contains(&position.1)
    }

    fn set_tile(&mut self, position: &(usize, usize), tile: Tile) {
        let index = self.get_tile_index(position);
        self.tiles[index] = tile;
    }

    fn get_tile(&self, position: &(usize, usize)) -> Tile {
        if !self.contains_position(position) {
            return Tile::Air;
        }

        self.tiles[self.get_tile_index(position)]
    }

    fn drop_sand(&mut self) -> bool {
        let mut position = self.source;

        if self.get_tile(&position) != Tile::Air {
            return false;
        }

        while self.contains_position(&position) {
            if self.get_tile(&(position.0, position.1 + 1)) == Tile::Air {
                position = (position.0, position.1 + 1);
            } else if self.get_tile(&(position.0 - 1, position.1 + 1)) == Tile::Air {
                position = (position.0 - 1, position.1 + 1);
            } else if self.get_tile(&(position.0 + 1, position.1 + 1)) == Tile::Air {
                position = (position.0 + 1, position.1 + 1);
            } else {
                self.set_tile(&position, Tile::Sand);
                return true;
            }
        }

        false
    }

    fn drop_loads_of_sand(&mut self) -> i32 {
        let mut i = 0;

        while self.drop_sand() {
            i += 1;
        }

        i
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let source_index = self.get_tile_index(&self.source);

        for (i, tile) in self.tiles.iter().enumerate() {
            if i % self.size.0 == 0 {
                writeln!(f)?;
            }

            let tile_char = match tile {
                Tile::Rock => '#',
                Tile::Air if i == source_index => '+',
                Tile::Air => '.',
                Tile::Sand => 'o',
            };

            write!(f, "{}", tile_char)?;
        }
        writeln!(f)
    }
}

#[aoc_generator(day14)]
fn input_generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coordinates| {
                    let mut parts = coordinates.split(',');
                    (
                        parts.next().unwrap().parse().unwrap(),
                        parts.next().unwrap().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn part1(input: &[Line]) -> i32 {
    Map::from_lines(input, (500, 0), false).drop_loads_of_sand()
}

#[aoc(day14, part2)]
pub fn part2(input: &[Line]) -> i32 {
    Map::from_lines(input, (500, 0), true).drop_loads_of_sand()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let test_input = input_generator(
            "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
        );

        assert_eq!(part1(&test_input), 24);
        assert_eq!(part2(&test_input), 93);
    }
}
