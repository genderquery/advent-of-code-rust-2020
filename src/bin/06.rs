use std::collections::{HashMap, HashSet};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<String> {
    let mut sum = 0;
    for group in input.split("\n\n") {
        let mut answers = HashSet::new();
        for line in group.lines() {
            for c in line.chars() {
                answers.insert(c);
            }
        }
        sum += answers.len();
    }
    sum.to_string().into()
}

pub fn part_two(input: &str) -> Option<String> {
    const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";
    let mut sum = 0;
    for group in input.split("\n\n") {
        let mut count = 0;
        for c in ALPHA.chars() {
            if group.lines().all(|line| line.contains(c)) {
                count += 1;
            }
        }
        sum += count;
    }
    sum.to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        const INPUT: &str = indoc! {""};
        let result = part_one(INPUT);
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = indoc! {""};
        let result = part_two(INPUT);
        assert_eq!(result, None);
    }
}
