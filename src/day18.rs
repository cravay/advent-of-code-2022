use itertools::Itertools;

const SCAN_SIZE: usize = 22;

#[derive(Clone)]
struct Scan {
    cubes: [[[bool; SCAN_SIZE]; SCAN_SIZE]; SCAN_SIZE],
}

impl Scan {
    fn new() -> Scan {
        Scan {
            cubes: [[[false; SCAN_SIZE]; SCAN_SIZE]; SCAN_SIZE],
        }
    }

    fn get(&self, (x, y, z): (usize, usize, usize)) -> bool {
        self.cubes[x][y][z]
    }

    fn set(&mut self, (x, y, z): (usize, usize, usize), value: bool) {
        self.cubes[x][y][z] = value;
    }

    fn for_each_position<F>(mut f: F)
    where
        F: FnMut((usize, usize, usize)),
    {
        for x in 0..SCAN_SIZE {
            for y in 0..SCAN_SIZE {
                for z in 0..SCAN_SIZE {
                    f((x, y, z));
                }
            }
        }
    }

    fn for_each_neighbor_position<F>((x, y, z): (usize, usize, usize), mut f: F)
    where
        F: FnMut(Option<(usize, usize, usize)>),
    {
        let min = 0;
        let max = SCAN_SIZE - 1;

        f(if x == min { None } else { Some((x - 1, y, z)) });
        f(if x == max { None } else { Some((x + 1, y, z)) });

        f(if y == min { None } else { Some((x, y - 1, z)) });
        f(if y == max { None } else { Some((x, y + 1, z)) });

        f(if z == min { None } else { Some((x, y, z - 1)) });
        f(if z == max { None } else { Some((x, y, z + 1)) });
    }

    fn get_surface_area(&self) -> usize {
        let mut surface_area = 0;

        Scan::for_each_position(|position| {
            if !self.get(position) {
                return;
            }

            Scan::for_each_neighbor_position(position, |option| match option {
                Some(neighbor_position) => {
                    if !self.get(neighbor_position) {
                        surface_area += 1;
                    }
                }
                None => surface_area += 1,
            });
        });

        surface_area
    }

    fn get_filled_clone(&self) -> Scan {
        let mut filled_scan = Scan {
            cubes: [[[true; SCAN_SIZE]; SCAN_SIZE]; SCAN_SIZE],
        };

        let mut stack = Vec::new();

        for i in 0..SCAN_SIZE {
            for ii in 0..SCAN_SIZE {
                stack.push((0, i, ii));
                stack.push((SCAN_SIZE - 1, i, ii));

                stack.push((i, 0, ii));
                stack.push((i, SCAN_SIZE - 1, ii));

                stack.push((i, ii, 0));
                stack.push((i, ii, SCAN_SIZE - 1));
            }
        }

        while let Some(position) = stack.pop() {
            if self.get(position) || !filled_scan.get(position) {
                continue;
            }

            filled_scan.set(position, false);

            Scan::for_each_neighbor_position(position, |option| {
                if let Some(neighbor_position) = option {
                    stack.push(neighbor_position);
                }
            });
        }

        filled_scan
    }
}

#[aoc_generator(day18)]
fn input_generator(input: &str) -> Scan {
    let mut scan = Scan::new();

    for line in input.lines() {
        let position = line
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect_tuple()
            .unwrap();

        scan.set(position, true);
    }

    scan
}

#[aoc(day18, part1)]
fn part1(input: &Scan) -> usize {
    input.get_surface_area()
}

#[aoc(day18, part2)]
fn part2(input: &Scan) -> usize {
    input.get_filled_clone().get_surface_area()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let test_input = input_generator(include_str!("../input/2022/day18.sample.txt"));

        assert_eq!(part1(&test_input), 64);
        assert_eq!(part2(&test_input), 58);
    }
}
