#[derive(Clone)]
struct Tree {
    height: i8,
    is_visible: bool,
}

#[derive(Clone)]
struct Forest {
    height: usize,
    width: usize,
    trees: Vec<Tree>,
}

impl From<&str> for Forest {
    fn from(string: &str) -> Self {
        let width = string.lines().next().unwrap().chars().count();

        let trees: Vec<Tree> = string
            .chars()
            .filter(|ch| ('0'..='9').contains(ch))
            .map(|ch| Tree {
                height: (ch as u8 - b'0') as i8,
                is_visible: false,
            })
            .collect();

        let height = trees.len() / width;

        Forest {
            height,
            width,
            trees,
        }
    }
}

impl Forest {
    fn get_tree_at(&mut self, x: usize, y: usize) -> Option<&mut Tree> {
        self.trees.get_mut(y * self.width + x)
    }
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Forest {
    input.into()
}

fn mark_invisible_trees(
    forest: &mut Forest,
    start: (usize, usize),
    step: (isize, isize),
    steps: usize,
) {
    let mut highest_tree = i8::MIN;
    let (mut x, mut y) = start;

    for _ in 0..steps {
        if let Some(tree) = forest.get_tree_at(x, y) {
            if tree.height > highest_tree {
                tree.is_visible = true;
                highest_tree = tree.height;
            }
        }

        x = (x as isize + step.0) as usize;
        y = (y as isize + step.1) as usize;
    }
}

#[aoc(day8, part1)]
fn part1(forest: &Forest) -> usize {
    let mut forest = forest.clone();
    let Forest { width, height, .. } = forest;

    for y in 0..height {
        mark_invisible_trees(&mut forest, (0, y), (1, 0), width);
        mark_invisible_trees(&mut forest, (width - 1, y), (-1, 0), width);
    }

    for x in 0..width {
        mark_invisible_trees(&mut forest, (x, 0), (0, 1), height);
        mark_invisible_trees(&mut forest, (x, height - 1), (0, -1), height);
    }

    forest.trees.iter().filter(|tree| tree.is_visible).count()
}

fn get_viewing_distance(
    forest: &mut Forest,
    position: (usize, usize),
    step: (isize, isize),
    distance_to_border: usize,
) -> i32 {
    let mut viewing_distance = 1;
    let (mut x, mut y) = position;
    let tree_height = forest.get_tree_at(x, y).unwrap().height;

    for _ in 0..distance_to_border - 1 {
        x = (x as isize + step.0) as usize;
        y = (y as isize + step.1) as usize;

        let other_tree_height = forest.get_tree_at(x, y).unwrap().height;

        if other_tree_height >= tree_height {
            return viewing_distance;
        }

        viewing_distance += 1;
    }

    viewing_distance
}

fn get_scenic_score(forest: &mut Forest, position: (usize, usize)) -> i32 {
    let (x, y) = position;
    let width = forest.width;
    let height = forest.height;

    get_viewing_distance(forest, position, (1, 0), width - x - 1)
        * get_viewing_distance(forest, position, (-1, 0), x)
        * get_viewing_distance(forest, position, (0, 1), height - y - 1)
        * get_viewing_distance(forest, position, (0, -1), y)
}

#[aoc(day8, part2)]
fn part2(forest: &Forest) -> i32 {
    let mut forest = forest.clone();
    let Forest { width, height, .. } = forest;
    let mut max_scenic_score = i32::MIN;

    for x in 1..width - 1 {
        for y in 1..height - 1 {
            let scenic_score = get_scenic_score(&mut forest, (x, y));
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    max_scenic_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let test_input = input_generator(
            "30373
25512
65332
33549
35390",
        );

        assert_eq!(part1(&test_input), 21);
        assert_eq!(part2(&test_input), 8);
    }
}
