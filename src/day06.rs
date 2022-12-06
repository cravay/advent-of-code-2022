use std::collections::VecDeque;

fn has_duplicates(deque: &VecDeque<char>) -> bool {
    for (i1, ch1) in deque.iter().enumerate() {
        for (i2, ch2) in deque.iter().enumerate() {
            if i1 != i2 && ch1 == ch2 {
                return true;
            }
        }
    }

    false
}

fn get_message_start_index(input: &str, marker_size: usize) -> usize {
    let mut deque: VecDeque<char> = input.chars().take(marker_size - 1).collect();

    for (i, ch) in input.chars().enumerate().skip(marker_size - 1) {
        deque.push_back(ch);

        if !has_duplicates(&deque) {
            return i + 1;
        }

        deque.pop_front();
    }

    unreachable!()
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    get_message_start_index(input, 4)
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    get_message_start_index(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);

        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
