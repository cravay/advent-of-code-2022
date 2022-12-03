#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HandShape {
    Rock,
    Paper,
    Scissors,
}
use HandShape::*;

#[derive(Clone, Copy)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}
use Outcome::*;

pub struct HandShapes {
    opponent: HandShape,
    own: HandShape,
}
pub struct Strategy {
    opponent_hand_shape: HandShape,
    outcome: Outcome,
}

fn parse_opponent_hand_shape(input: &str) -> HandShape {
    match input {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => panic!(),
    }
}

fn parse_own_hand_shape(input: &str) -> HandShape {
    match input {
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        _ => panic!(),
    }
}
fn parse_outcome(input: &str) -> Outcome {
    match input {
        "X" => Lose,
        "Y" => Draw,
        "Z" => Win,
        _ => panic!(),
    }
}

#[aoc_generator(day2, part1)]
pub fn input_generator_part1(input: &str) -> Vec<HandShapes> {
    input
        .lines()
        .map(|line| HandShapes {
            opponent: parse_opponent_hand_shape(line.get(0..1).unwrap()),
            own: parse_own_hand_shape(line.get(2..3).unwrap()),
        })
        .collect()
}

#[aoc_generator(day2, part2)]
pub fn input_generator_part2(input: &str) -> Vec<Strategy> {
    input
        .lines()
        .map(|line| Strategy {
            opponent_hand_shape: parse_opponent_hand_shape(line.get(0..1).unwrap()),
            outcome: parse_outcome(line.get(2..3).unwrap()),
        })
        .collect()
}

fn get_score(hand_shapes: &HandShapes) -> i32 {
    let mut score = match hand_shapes.own {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };

    score += match (hand_shapes.opponent, hand_shapes.own) {
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 0,
    };

    score
}
#[aoc(day2, part1)]
pub fn part1(input: &[HandShapes]) -> i32 {
    input.iter().map(get_score).sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Strategy]) -> i32 {
    input
        .iter()
        .map(|strategy| {
            let own_hand_shape = match (strategy.opponent_hand_shape, strategy.outcome) {
                (Rock, Draw) | (Paper, Lose) | (Scissors, Win) => Rock,
                (Rock, Win) | (Paper, Draw) | (Scissors, Lose) => Paper,
                (Rock, Lose) | (Paper, Win) | (Scissors, Draw) => Scissors,
            };
            get_score(&HandShapes {
                opponent: strategy.opponent_hand_shape,
                own: own_hand_shape,
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator_part1, input_generator_part2, part1, part2};

    #[test]
    fn sample1() {
        let input_string = "A Y\nB X\nC Z";
        assert_eq!(part1(&input_generator_part1(input_string)), 15);
        assert_eq!(part2(&input_generator_part2(input_string)), 12);
    }
}
