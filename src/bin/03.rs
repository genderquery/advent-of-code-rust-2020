advent_of_code::solution!(3);

struct Forest {
    trees: Vec<bool>,
    width: usize,
    height: usize,
}

impl Forest {
    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + (x % self.width)
    }

    pub fn has_tree(&self, x: usize, y: usize) -> bool {
        self.trees[self.index(x, y)]
    }

    /// Traverse the forest using the pattern `right` and `down`,
    /// returning the number of trees encountered.
    pub fn traverse(&self, right: usize, down: usize) -> usize {
        let mut x = 0;
        let mut y = 0;
        let mut count = 0;

        while y < self.height {
            if self.has_tree(x, y) {
                count += 1;
            }
            x += right;
            y += down;
        }

        count
    }
}

fn parse(input: &str) -> Forest {
    let mut lines = input.lines();
    let first = lines.next().unwrap();
    let width = first.len();
    let is_tree = |c| c == '#';
    let mut trees: Vec<bool> = first.chars().map(is_tree).collect();
    for row in lines {
        trees.extend(row.chars().map(is_tree));
    }
    let height = trees.len() / width;
    Forest {
        trees,
        width,
        height,
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let forest = parse(input);
    let count = forest.traverse(3, 1);
    count.to_string().into()
}

pub fn part_two(input: &str) -> Option<String> {
    let forest = parse(input);
    let count = forest.traverse(1, 1)
        * forest.traverse(3, 1)
        * forest.traverse(5, 1)
        * forest.traverse(7, 1)
        * forest.traverse(1, 2);
    count.to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        const INPUT: &str = indoc! {"
            ..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#
        "};
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, "7");
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = indoc! {"
            ..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#
        "};
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, "336");
    }
}
