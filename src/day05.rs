#[derive(Clone, Copy)]
pub struct Instruction {
    times: usize,
    from: usize,
    to: usize,
}

type Input = (Vec<Vec<char>>, Vec<Instruction>);

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();

    let crates: Vec<Vec<char>> = lines
        .by_ref()
        .take_while(|line| line.trim() != "")
        .map(|line| line.chars().skip(1).step_by(4).collect())
        .collect();

    let mut stacks = vec![vec![]; crates.last().unwrap().len()];

    crates[0..crates.len() - 1].iter().rev().for_each(|stack| {
        stack.iter().enumerate().for_each(|(i, value)| {
            if *value != ' ' {
                stacks[i].push(*value)
            }
        })
    });

    let instructions = lines
        .map(|line| {
            let mut parts = line.split(' ');
            Instruction {
                times: parts.nth(1).unwrap().parse().unwrap(),
                from: parts.nth(1).unwrap().parse::<usize>().unwrap() - 1,
                to: parts.nth(1).unwrap().parse::<usize>().unwrap() - 1,
            }
        })
        .collect();

    (stacks, instructions)
}

#[aoc(day5, part1)]
pub fn part1(input: &Input) -> String {
    let (mut stacks, instructions) = input.clone();

    for Instruction { times, from, to } in instructions {
        for _ in 0..times {
            let value = stacks.get_mut(from).unwrap().pop().unwrap();
            stacks.get_mut(to).unwrap().push(value);
        }
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

#[aoc(day5, part2)]
pub fn part2(input: &Input) -> String {
    let (mut stacks, instructions) = input.clone();

    for Instruction { times, from, to } in instructions {
        for i in 0..times {
            let from_stack = stacks.get_mut(from).unwrap();
            let value = from_stack.remove(from_stack.len() + i - times);
            stacks.get_mut(to).unwrap().push(value);
        }
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let test_input = input_generator(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );

        assert_eq!(part1(&test_input), "CMZ");
        assert_eq!(part2(&test_input), "MCD");
    }
}
