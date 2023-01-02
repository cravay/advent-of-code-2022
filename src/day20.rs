use linked_list::{Cursor, LinkedList};

struct Node {
    value: i64,
    index: usize,
}

#[aoc_generator(day20)]
fn input_generator(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn seek_forward_skip_none<T>(cursor: &mut Cursor<T>, by: usize) {
    for _ in 0..by {
        if cursor.next().is_none() {
            cursor.next();
        }
    }
    if cursor.peek_next().is_none() {
        cursor.next();
    }
}

fn seek_up_to<T, F>(cursor: &mut Cursor<T>, f: F)
where
    F: Fn(&T) -> bool,
{
    loop {
        if let Some(node) = cursor.next() {
            if f(node) {
                cursor.prev();
                return;
            }
        }
    }
}

fn get_grove_coordinates_sum(values: &[i64], decryption_key: i64, mix_rounds: i32) -> i64 {
    let mut nodes: LinkedList<Node> = values
        .iter()
        .enumerate()
        .map(|(index, value)| Node {
            value: value * decryption_key,
            index,
        })
        .collect();
    let len = nodes.len();
    let mut cursor = nodes.cursor();

    for _ in 0..mix_rounds {
        for i in 0..len {
            seek_up_to(&mut cursor, |node| node.index == i);
            let node = cursor.remove().unwrap();
            seek_forward_skip_none(&mut cursor, node.value.rem_euclid(len as i64 - 1) as usize);
            cursor.insert(node);
        }
    }

    seek_up_to(&mut cursor, |node| node.value == 0);

    let mut sum = 0;

    for _ in 0..3 {
        seek_forward_skip_none(&mut cursor, 1000);
        sum += cursor.peek_next().unwrap().value;
    }

    sum
}

#[aoc(day20, part1)]
fn part1(input: &[i64]) -> i64 {
    get_grove_coordinates_sum(input, 1, 1)
}

#[aoc(day20, part2)]
fn part2(input: &[i64]) -> i64 {
    get_grove_coordinates_sum(input, 811589153, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let test_input = input_generator("1\n2\n-3\n3\n-2\n0\n4");

        assert_eq!(part1(&test_input), 3);
        assert_eq!(part2(&test_input), 1623178306);
    }
}
