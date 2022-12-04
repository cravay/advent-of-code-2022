use std::ops::Range;

fn parse_range(input: &str) -> Range<i32> {
    let mut parts = input.split('-').map(|s| s.parse::<i32>().unwrap());
    parts.next().unwrap()..parts.next().unwrap() + 1
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<(Range<i32>, Range<i32>)> {
    input
        .lines()
        .map(|line| {
            let mut ranges = line.split(',').map(parse_range);
            (ranges.next().unwrap(), ranges.next().unwrap())
        })
        .collect()
}

fn is_sub_range(range: &Range<i32>, other: &Range<i32>) -> bool {
    range.start <= other.start && other.end <= range.end
}

#[aoc(day4, part1)]
pub fn part1(input: &[(Range<i32>, Range<i32>)]) -> usize {
    input
        .iter()
        .filter(|(range1, range2)| is_sub_range(range1, range2) || is_sub_range(range2, range1))
        .count()
}

fn ranges_overlap(range1: &Range<i32>, range2: &Range<i32>) -> bool {
    range1.contains(&range2.start) || range2.contains(&range1.start)
}

#[aoc(day4, part2)]
pub fn part2(input: &[(Range<i32>, Range<i32>)]) -> usize {
    input
        .iter()
        .filter(|(range1, range2)| ranges_overlap(range1, range2))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let test_input = &input_generator(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
        );

        assert_eq!(part1(test_input), 2);
        assert_eq!(part2(test_input), 4);
    }
}
