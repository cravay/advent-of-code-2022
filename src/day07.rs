use core::panic;
use std::collections::HashMap;

#[derive(Debug)]
enum Node {
    File(i32),
    Directory(HashMap<String, Node>),
}

impl Node {
    fn new() -> Node {
        Node::Directory(HashMap::new())
    }

    fn add_file(&mut self, file_name: &str, file_size: i32) {
        match self {
            Node::Directory(ref mut children) => {
                children.insert(file_name.to_string(), Node::File(file_size));
            }
            _ => panic!(),
        }
    }

    fn add_directory(&mut self, directory_name: &str) {
        match self {
            Node::Directory(ref mut children) => {
                children.insert(directory_name.to_string(), Node::Directory(HashMap::new()));
            }
            _ => panic!(),
        }
    }

    fn get_size(&self) -> i32 {
        match self {
            Node::File(size) => *size,
            Node::Directory(children) => children.values().map(|node| (*node).get_size()).sum(),
        }
    }

    fn get_total_size_of_small_folders(&self) -> i32 {
        let mut sum = 0;

        if let Node::Directory(children) = self {
            let size = self.get_size();
            if size <= 100000 {
                sum += size;
            }

            sum += children
                .values()
                .map(Node::get_total_size_of_small_folders)
                .sum::<i32>()
        }

        sum
    }

    fn get_smallest_folder_size_larger_than(&self, min_size: i32) -> i32 {
        let mut smallest_folder_size = i32::MAX;

        if let Node::Directory(children) = self {
            let size = self.get_size();
            if size >= min_size {
                smallest_folder_size = size;
            }

            let min_child_size = children
                .values()
                .map(|node| node.get_smallest_folder_size_larger_than(min_size))
                .min()
                .unwrap_or(i32::MAX);

            if min_child_size < smallest_folder_size {
                smallest_folder_size = min_child_size;
            }
        }

        smallest_folder_size
    }
}

fn get_root_node_for_command_log(input: &str) -> Node {
    let mut root = Node::new();
    let mut current_path: Vec<*mut Node> = vec![&mut root];

    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();

        match parts[..] {
            ["$", "cd", directory_name] => {
                if directory_name == "/" {
                    current_path.truncate(1);
                } else if directory_name == ".." {
                    current_path.pop().unwrap();
                } else {
                    let current_node = *current_path.last().unwrap();

                    unsafe {
                        let child_node = match &mut *current_node {
                            Node::Directory(children) => children.get_mut(directory_name).unwrap(),
                            _ => panic!(),
                        };

                        current_path.push(&mut *child_node);
                    }
                }
            }
            ["$", "ls", ..] => (),
            ["dir", directory_name] => unsafe {
                (**current_path.last().unwrap()).add_directory(directory_name);
            },
            [file_size, file_name] => unsafe {
                (**current_path.last().unwrap()).add_file(file_name, file_size.parse().unwrap());
            },
            _ => panic!(),
        }
    }

    root
}

#[aoc(day7, part1)]
fn part1(input: &str) -> i32 {
    get_root_node_for_command_log(input).get_total_size_of_small_folders()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> i32 {
    let root = get_root_node_for_command_log(input);
    root.get_smallest_folder_size_larger_than(30000000 - (70000000 - root.get_size()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input_string = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!(part1(input_string), 95437);
        assert_eq!(part2(input_string), 24933642);
    }
}
