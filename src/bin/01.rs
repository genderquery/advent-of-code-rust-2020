advent_of_code::solution!(1);

use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
enum ParseError {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

fn parse(input: &str) -> Result<Vec<u32>, ParseError> {
    input
        .lines()
        .map(|line| line.trim().parse().map_err(ParseError::ParseIntError))
        .collect()
}

/// Find two elements that summed together equal `sum`.
fn find_sum_pair(s: &[u32], sum: u32) -> Option<(usize, usize)> {
    if s.len() < 2 {
        return None;
    }
    for i in 0..(s.len() - 1) {
        for j in (i + 1)..s.len() {
            if s[i] + s[j] == sum {
                return Some((i, j));
            }
        }
    }
    None
}

/// Find three elements that summed together equal `sum`.
fn find_sum_triple(s: &[u32], sum: u32) -> Option<(usize, usize, usize)> {
    if s.len() < 3 {
        return None;
    }
    for i in 0..(s.len() - 2) {
        for j in (i + 1)..(s.len() - 1) {
            for k in (j + 1)..s.len() {
                if s[i] + s[j] + s[k] == sum {
                    return Some((i, j, k));
                }
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<String> {
    let report = parse(input).unwrap();
    let (i, j) = find_sum_pair(&report, 2020).unwrap();
    let product = report[i] * report[j];
    product.to_string().into()
}

pub fn part_two(input: &str) -> Option<String> {
    let report = parse(input).unwrap();
    let (i, j, k) = find_sum_triple(&report, 2020).unwrap();
    let product = report[i] * report[j] * report[k];
    product.to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_find_sum_pair() {
        assert_eq!(find_sum_pair(&[], 0), None);
        assert_eq!(find_sum_pair(&[0], 0), None);
        assert_eq!(find_sum_pair(&[1, 2], 3), Some((0, 1)));
        assert_eq!(
            find_sum_pair(&[1721, 979, 366, 299, 675, 1456], 2020),
            Some((0, 3))
        );
    }

    #[test]
    fn test_find_sum_triple() {
        assert_eq!(find_sum_triple(&[], 0), None);
        assert_eq!(find_sum_triple(&[0], 0), None);
        assert_eq!(find_sum_triple(&[1, 2, 3], 6), Some((0, 1, 2)));
        assert_eq!(
            find_sum_triple(&[1721, 979, 366, 299, 675, 1456], 2020),
            Some((1, 2, 4))
        );
    }

    #[test]
    fn test_part_one() {
        const INPUT: &str = indoc! {"
            1721
            979
            366
            299
            675
            1456
        "};
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, "514579");
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = indoc! {"
            1721
            979
            366
            299
            675
            1456
        "};
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, "241861950");
    }
}
