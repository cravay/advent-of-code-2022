use core::panic;
use std::{fmt::Write, iter::Peekable, slice::Iter};

pub enum Instruction {
    Noop,
    AddX(i32),
}

struct Processor<'a> {
    instructions: Peekable<Iter<'a, Instruction>>,
    x: i32,
    cycle: u32,
    next: u32,
}

impl Processor<'_> {
    fn new(instructions: &[Instruction]) -> Processor {
        Processor {
            x: 1,
            cycle: 0,
            next: 1,
            instructions: instructions.iter().peekable(),
        }
    }

    fn step(&mut self) {
        self.cycle += 1;
        self.next -= 1;

        if self.next > 0 {
            return;
        }

        if let Some(Instruction::AddX(x)) = self.instructions.next() {
            self.x += x;
        }

        self.next = match self.instructions.peek() {
            Some(Instruction::Noop) => 1,
            Some(Instruction::AddX(_)) => 2,
            None => 1,
        }
    }
}

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            match parts.next().unwrap() {
                "noop" => Instruction::Noop,
                "addx" => Instruction::AddX(parts.next().unwrap().parse().unwrap()),
                _ => panic!(),
            }
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &[Instruction]) -> i32 {
    let mut processor = Processor::new(input);
    let mut sum = 0;

    for cycle in [20, 60, 100, 140, 180, 220] {
        while processor.cycle < cycle - 2 {
            processor.step();
        }

        sum += processor.x * cycle as i32
    }

    sum
}

#[aoc(day10, part2)]
pub fn part2(input: &[Instruction]) -> String {
    let mut processor = Processor::new(input);
    let mut output = String::new();
    let mut sprite_pos = processor.x;

    for _ in 0..6 {
        output.write_char('\n').unwrap();

        for i in 0..40 {
            let distance = (sprite_pos - i).abs();
            let char = if distance <= 1 { '#' } else { '.' };
            output.write_char(char).unwrap();
            sprite_pos = processor.x;
            processor.step();
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let test_input = input_generator(include_str!("../input/2022/day10.sample.txt"));

        assert_eq!(part1(&test_input), 13140);
        assert_eq!(
            part2(&test_input),
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
