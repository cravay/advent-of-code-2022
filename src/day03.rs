use std::collections::HashSet;

pub struct Rucksack {
    all_items: HashSet<char>,
    first_compartment: HashSet<char>,
    second_compartment: HashSet<char>,
}

impl From<&str> for Rucksack {
    fn from(string: &str) -> Self {
        let compartments = string.split_at(string.len() / 2);
        Rucksack {
            all_items: string.chars().collect(),
            first_compartment: compartments.0.chars().collect(),
            second_compartment: compartments.1.chars().collect(),
        }
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Rucksack> {
    input.lines().map(Rucksack::from).collect()
}

fn get_priority(item: char) -> i32 {
    match item {
        'a'..='z' => item as i32 - 'a' as i32 + 1,
        'A'..='Z' => item as i32 - 'A' as i32 + 26 + 1,
        _ => panic!(),
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &[Rucksack]) -> i32 {
    input
        .iter()
        .map(|rucksack| {
            let wrongly_packed_item = rucksack
                .first_compartment
                .intersection(&rucksack.second_compartment)
                .next()
                .unwrap();

            get_priority(*wrongly_packed_item)
        })
        .sum()
}
#[aoc(day3, part2)]
pub fn part2(input: &[Rucksack]) -> i32 {
    input
        .chunks(3)
        .map(|rucksack_group| {
            let badge = rucksack_group[0]
                .all_items
                .iter()
                .find(|item| {
                    rucksack_group[1].all_items.contains(item)
                        && rucksack_group[2].all_items.contains(item)
                })
                .unwrap();
            get_priority(*badge)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let test_input = &input_generator(
            r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#,
        );

        assert_eq!(part1(test_input), 157);
        assert_eq!(part2(test_input), 70);
    }
}
