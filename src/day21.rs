use std::collections::HashMap;

#[derive(Clone)]
enum Monkey {
    Number(i64),
    Operation(Operation),
}

#[derive(Clone)]
struct Operation {
    operator: Operator,
    operands: (String, String),
}

#[derive(Clone)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

const ROOT_NAME: &str = "root";
const HUMAN_NAME: &str = "humn";

#[aoc_generator(day21)]
fn input_generator(input: &str) -> HashMap<String, Monkey> {
    let mut monkeys = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split(": ");
        let name = parts.next().unwrap().to_owned();
        let monkey = match parts.next().unwrap().split(' ').collect::<Vec<_>>()[..] {
            [number] => Monkey::Number(number.parse().unwrap()),
            [a, operator, b] => Monkey::Operation(Operation {
                operator: match operator {
                    "+" => Operator::Add,
                    "-" => Operator::Subtract,
                    "*" => Operator::Multiply,
                    "/" => Operator::Divide,
                    _ => panic!(),
                },
                operands: (a.to_owned(), b.to_owned()),
            }),
            _ => panic!(),
        };

        monkeys.insert(name, monkey);
    }

    monkeys
}

fn get_monkey_number(monkeys: &HashMap<String, Monkey>, name: &str) -> i64 {
    match monkeys.get(name).unwrap() {
        Monkey::Number(number) => *number,
        Monkey::Operation(Operation {
            operator,
            operands: (a, b),
        }) => {
            let value_a = get_monkey_number(monkeys, a);
            let value_b = get_monkey_number(monkeys, b);

            match operator {
                Operator::Add => value_a + value_b,
                Operator::Subtract => value_a - value_b,
                Operator::Multiply => value_a * value_b,
                Operator::Divide => value_a / value_b,
            }
        }
    }
}

fn get_human_number(
    monkeys: &HashMap<String, Monkey>,
    name: &str,
    expected_result: i64,
) -> Option<i64> {
    match monkeys.get(name).unwrap() {
        Monkey::Number(_) => {
            if name == HUMAN_NAME {
                Some(expected_result)
            } else {
                None
            }
        }
        Monkey::Operation(Operation {
            operator,
            operands: (a, b),
        }) => {
            let value_a = get_monkey_number(monkeys, a);
            let value_b = get_monkey_number(monkeys, b);

            let expected_a_result = match operator {
                // expected_a_result + value_b = expected_result
                Operator::Add => expected_result - value_b,

                // expected_a_result - value_b = expected_result
                Operator::Subtract => expected_result + value_b,

                // expected_a_result * value_b = expected_result
                Operator::Multiply => expected_result / value_b,

                // expected_a_result / value_b = expected_result
                Operator::Divide => expected_result * value_b,
            };

            if let Some(number) = get_human_number(monkeys, a, expected_a_result) {
                return Some(number);
            }

            let expected_b_result = match operator {
                // value_a + expected_b_result = expected_result
                Operator::Add => expected_result - value_a,

                // value_a - expected_b_result = expected_result
                Operator::Subtract => value_a - expected_result,

                // value_a * expected_b_result = expected_result
                Operator::Multiply => expected_result / value_a,

                // value_a / expected_b_result = expected_result
                Operator::Divide => value_a / expected_result,
            };

            if let Some(number) = get_human_number(monkeys, b, expected_b_result) {
                return Some(number);
            }

            None
        }
    }
}

#[aoc(day21, part1)]
fn part1(input: &HashMap<String, Monkey>) -> i64 {
    get_monkey_number(input, ROOT_NAME)
}

#[aoc(day21, part2)]
fn part2(input: &HashMap<String, Monkey>) -> i64 {
    if let Monkey::Operation(Operation {
        operands: (a, b), ..
    }) = input.get(ROOT_NAME).unwrap()
    {
        let value_a = get_monkey_number(input, a);
        let value_b = get_monkey_number(input, b);

        if let Some(number) = get_human_number(input, a, value_b) {
            return number;
        }

        if let Some(number) = get_human_number(input, b, value_a) {
            return number;
        }
    }

    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let test_input = input_generator(include_str!("../input/2022/day21.sample.txt"));

        assert_eq!(part1(&test_input), 152);
        assert_eq!(part2(&test_input), 301);
    }
}
