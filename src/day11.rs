use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum Operation {
    Add(i64),
    Subtract(i64),
    Multiply(i64),
    Square,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    divisor: i64,
    next_monkey_if_divisible: usize,
    next_monkey_if_indivisible: usize,
    inspections: i64,
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Vec<Monkey> {
    let mut lines = input.lines();
    let mut monkeys = Vec::new();

    while let Some(_) = lines.next() {
        let items = lines
            .next()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();

        let operation = match lines
            .next()
            .unwrap()
            .split(" = ")
            .last()
            .unwrap()
            .split(' ')
            .collect::<Vec<&str>>()[..]
        {
            ["old", "*", "old"] => Operation::Square,
            ["old", "+", addend] => Operation::Add(addend.parse().unwrap()),
            ["old", "-", subtrahend] => Operation::Subtract(subtrahend.parse().unwrap()),
            ["old", "*", multiplier] => Operation::Multiply(multiplier.parse().unwrap()),
            _ => panic!(),
        };

        fn parse_i64(line: &str) -> i64 {
            line.split(' ').last().unwrap().parse().unwrap()
        }

        monkeys.push(Monkey {
            items,
            operation,
            divisor: parse_i64(lines.next().unwrap()),
            next_monkey_if_divisible: parse_i64(lines.next().unwrap()) as usize,
            next_monkey_if_indivisible: parse_i64(lines.next().unwrap()) as usize,
            inspections: 0,
        });

        lines.next();
    }

    monkeys
}

fn get_monkey_business_level(monkeys: &[Monkey], rounds: i32, divide_worry_level: bool) -> i64 {
    let mut monkeys: Vec<Monkey> = monkeys.into();
    let len = monkeys.len();
    let divisor_product = monkeys.iter().map(|monkey| monkey.divisor).product();

    for _ in 0..rounds {
        for i in 0..len {
            let Monkey {
                items,
                operation,
                divisor,
                next_monkey_if_divisible,
                next_monkey_if_indivisible,
                mut inspections,
            } = monkeys[i].clone();
            monkeys[i].items.clear();

            for mut level in items.into_iter() {
                inspections += 1;

                level = match operation {
                    Operation::Add(addend) => level + addend,
                    Operation::Subtract(subtrahend) => level - subtrahend,
                    Operation::Multiply(multiplier) => level * multiplier,
                    Operation::Square => level * level,
                };

                level = level.rem_euclid(divisor_product);

                if divide_worry_level {
                    level /= 3;
                }

                let next_monkey = if level % divisor == 0 {
                    next_monkey_if_divisible
                } else {
                    next_monkey_if_indivisible
                };

                monkeys[next_monkey].items.push_back(level);
            }

            monkeys[i].inspections = inspections;
        }
    }

    let mut inspections = monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect::<Vec<i64>>();

    inspections.sort();
    inspections[len - 1] * inspections[len - 2]
}

#[aoc(day11, part1)]
fn part1(input: &[Monkey]) -> i64 {
    get_monkey_business_level(input, 20, true)
}

#[aoc(day11, part2)]
fn part2(input: &[Monkey]) -> i64 {
    get_monkey_business_level(input, 10000, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let test_input = input_generator(include_str!("../input/2022/day11.sample.txt"));

        assert_eq!(part1(&test_input), 10605);
        assert_eq!(part2(&test_input), 2713310158);
    }
}
