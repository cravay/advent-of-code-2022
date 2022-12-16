use std::cmp::Ordering;

use itertools::{EitherOrBoth, Itertools};

#[derive(PartialEq, Eq)]
pub enum Packet {
    Integer(i32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => {
                for pair in a.iter().zip_longest(b) {
                    match pair {
                        EitherOrBoth::Both(a, b) => {
                            let cmp = a.cmp(b);

                            if cmp != Ordering::Equal {
                                return cmp;
                            }
                        }
                        EitherOrBoth::Left(_) => return Ordering::Greater,
                        EitherOrBoth::Right(_) => return Ordering::Less,
                    }
                }

                Ordering::Equal
            }
            (Packet::Integer(a), Packet::List(_)) => {
                Packet::List(vec![Packet::Integer(*a)]).cmp(other)
            }
            (Packet::List(_), Packet::Integer(b)) => {
                self.cmp(&Packet::List(vec![Packet::Integer(*b)]))
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_packet(input: &str) -> (usize, Packet) {
    let digits = input.chars().take_while(|c| c.is_ascii_digit()).count();

    if digits > 0 {
        return (digits, Packet::Integer(input[0..digits].parse().unwrap()));
    }

    assert!(input.starts_with('['));

    let mut packets = Vec::new();
    let mut offset = 1;

    while !input[offset..].starts_with(']') {
        let (size, packet) = parse_packet(&input[offset..]);
        offset += size;
        packets.push(packet);

        if input[offset..].starts_with(',') {
            offset += 1;
        }
    }

    (offset + 1, Packet::List(packets))
}

#[aoc_generator(day13, part1)]
fn input_generator_1(input: &str) -> Vec<(Packet, Packet)> {
    let mut lines = input.lines();
    let mut packets = Vec::new();

    while let Some(line) = lines.next() {
        let (_, a) = parse_packet(line);
        let (_, b) = parse_packet(lines.next().unwrap());
        packets.push((a, b));
        lines.next();
    }

    packets
}

#[aoc_generator(day13, part2)]
fn input_generator_2(input: &str) -> Vec<Packet> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (_, packet) = parse_packet(line);
            packet
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(packets: &[(Packet, Packet)]) -> usize {
    packets
        .iter()
        .enumerate()
        .filter(|(_, (a, b))| a < b)
        .map(|(i, _)| i + 1)
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(packets: &[Packet]) -> usize {
    let divider1 = Packet::Integer(2);
    let divider2 = Packet::Integer(6);

    let pos1 = packets.iter().filter(|&p| p < &divider1).count() + 1;
    let pos2 = packets.iter().filter(|&p| p < &divider2).count() + 2;

    pos1 * pos2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let test_input = include_str!("../input/2022/day13.sample.txt");

        assert_eq!(part1(&input_generator_1(test_input)), 13);
        assert_eq!(part2(&input_generator_2(test_input)), 140);
    }
}
