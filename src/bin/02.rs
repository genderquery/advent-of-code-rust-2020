use parser::password_list;

advent_of_code::solution!(2);

mod parser {

    use nom::{
        bytes::complete::tag,
        character::complete::{anychar, char, digit1, newline, not_line_ending, space1},
        combinator::{map, map_res},
        multi::separated_list0,
        sequence::{separated_pair, tuple},
        IResult,
    };

    use crate::{PasswordEntry, PasswordList, PasswordPolicy};

    fn integer(input: &str) -> IResult<&str, u8> {
        map_res(digit1, str::parse)(input)
    }

    fn range(input: &str) -> IResult<&str, (u8, u8)> {
        separated_pair(integer, char('-'), integer)(input)
    }

    pub fn password_policy(input: &str) -> IResult<&str, PasswordPolicy> {
        map(
            tuple((range, space1, anychar)),
            |((min, max), _, letter)| PasswordPolicy { letter, min, max },
        )(input)
    }

    pub fn password_entry(input: &str) -> IResult<&str, PasswordEntry> {
        map(
            tuple((password_policy, tag(": "), not_line_ending)),
            |(policy, _, password)| PasswordEntry {
                policy,
                password: password.to_string(),
            },
        )(input)
    }

    pub fn password_list(input: &str) -> IResult<&str, PasswordList> {
        separated_list0(newline, password_entry)(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use indoc::indoc;

        #[test]
        fn example() {
            const INPUT: &str = indoc! {"
                1-3 a: abcde
                1-3 b: cdefg
                2-9 c: ccccccccc
            "};
            let (_, result) = password_list(INPUT).unwrap();
            assert_eq!(
                result,
                vec![
                    PasswordEntry {
                        policy: PasswordPolicy {
                            letter: 'a',
                            min: 1,
                            max: 3
                        },
                        password: "abcde".to_string()
                    },
                    PasswordEntry {
                        policy: PasswordPolicy {
                            letter: 'b',
                            min: 1,
                            max: 3
                        },
                        password: "cdefg".to_string()
                    },
                    PasswordEntry {
                        policy: PasswordPolicy {
                            letter: 'c',
                            min: 2,
                            max: 9
                        },
                        password: "ccccccccc".to_string()
                    },
                ]
            );
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct PasswordPolicy {
    letter: char,
    min: u8,
    max: u8,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct PasswordEntry {
    password: String,
    policy: PasswordPolicy,
}

type PasswordList = Vec<PasswordEntry>;

impl PasswordEntry {
    pub fn is_valid_1(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|&c| c == self.policy.letter)
            .count() as u8;
        count >= self.policy.min && count <= self.policy.max
    }

    pub fn is_valid_2(&self) -> bool {
        let a = self.policy.min as usize - 1;
        let b = self.policy.max as usize - 1;

        let a = self.password.chars().nth(a);
        if a.is_none() {
            return false;
        }

        let b = self.password.chars().nth(b);
        if b.is_none() {
            return false;
        }

        let a = a.unwrap();
        let b = b.unwrap();

        (a != b) && (a == self.policy.letter || b == self.policy.letter)
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (_, list) = password_list(input).unwrap();
    list.iter()
        .filter(|e| e.is_valid_1())
        .count()
        .to_string()
        .into()
}

pub fn part_two(input: &str) -> Option<String> {
    let (_, list) = password_list(input).unwrap();
    list.iter()
        .filter(|e| e.is_valid_2())
        .count()
        .to_string()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_is_valid_1() {
        assert_eq!(
            PasswordEntry {
                policy: PasswordPolicy {
                    letter: 'a',
                    min: 1,
                    max: 3
                },
                password: "abcde".to_string()
            }
            .is_valid_1(),
            true
        );
        assert_eq!(
            PasswordEntry {
                policy: PasswordPolicy {
                    letter: 'b',
                    min: 1,
                    max: 3
                },
                password: "cdefg".to_string()
            }
            .is_valid_1(),
            false
        );
        assert_eq!(
            PasswordEntry {
                policy: PasswordPolicy {
                    letter: 'c',
                    min: 2,
                    max: 9
                },
                password: "ccccccccc".to_string()
            }
            .is_valid_1(),
            true
        );
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_is_valid_2() {
        assert_eq!(
            PasswordEntry {
                policy: PasswordPolicy {
                    letter: 'a',
                    min: 1,
                    max: 3
                },
                password: "abcde".to_string()
            }
            .is_valid_2(),
            true
        );
        assert_eq!(
            PasswordEntry {
                policy: PasswordPolicy {
                    letter: 'b',
                    min: 1,
                    max: 3
                },
                password: "cdefg".to_string()
            }
            .is_valid_2(),
            false
        );
        assert_eq!(
            PasswordEntry {
                policy: PasswordPolicy {
                    letter: 'c',
                    min: 2,
                    max: 9
                },
                password: "ccccccccc".to_string()
            }
            .is_valid_2(),
            false
        );
    }

    #[test]
    fn test_part_one() {
        const INPUT: &str = indoc! {"
            1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc
        "};
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = indoc! {"
            1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc
        "};
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, "1");
    }
}
