#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().fold(vec![0], |mut acc, line| {
        match line.parse::<i32>() {
            Ok(n) => {
                *acc.last_mut().unwrap() += n;
            }
            Err(_) => {
                acc.push(0);
            }
        };
        acc
    })
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<i32>) -> i32 {
    input.iter().max().unwrap().to_owned()
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<i32>) -> i32 {
    let mut mutable_input = input.to_owned();
    mutable_input.sort();
    mutable_input.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    #[test]
    fn sample1() {
        let test_input =
            &input_generator("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");

        assert_eq!(part1(test_input), 24000);
        assert_eq!(part2(test_input), 45000);
    }
}
