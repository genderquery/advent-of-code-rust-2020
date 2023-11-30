advent_of_code::solution!(10);

fn parse(input: &str) -> Vec<u8> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part_one(input: &str) -> Option<String> {
    let mut adapters = parse(input);
    adapters.sort();
    // charging outlet
    adapters.insert(0, 0);
    let last = adapters.last().unwrap();
    // built-in adapter
    adapters.push(last + 3);

    let mut one_jolt_diff = 0;
    let mut three_jolt_diff = 0;

    let mut windows = adapters.windows(2);
    while let Some(&[a, b]) = windows.next() {
        match b - a {
            1 => {
                one_jolt_diff += 1;
            }
            3 => {
                three_jolt_diff += 1;
            }
            _ => unreachable!(),
        }
    }

    (one_jolt_diff * three_jolt_diff).to_string().into()
}

pub fn part_two(input: &str) -> Option<String> {
    let mut adapters = parse(input);
    adapters.sort();
    // charging outlet
    adapters.insert(0, 0);
    let last = adapters.last().unwrap();
    // built-in adapter
    adapters.push(last + 3);

    let mut arrangements: Vec<usize> = adapters.iter().map(|_| 0).collect();
    arrangements[0] = 1;

    for i in 1..adapters.len() {
        for j in 0..i {
            if adapters[i] - adapters[j] <= 3 {
                arrangements[i] += arrangements[j];
            }
        }
    }

    arrangements.last().unwrap().to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        const INPUT: &str = indoc! {"
            28
            33
            18
            42
            31
            14
            46
            20
            48
            47
            24
            23
            49
            45
            19
            38
            39
            11
            1
            32
            25
            35
            8
            17
            7
            9
            4
            2
            34
            10
            3
        "};
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, "220");
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = indoc! {"
            28
            33
            18
            42
            31
            14
            46
            20
            48
            47
            24
            23
            49
            45
            19
            38
            39
            11
            1
            32
            25
            35
            8
            17
            7
            9
            4
            2
            34
            10
            3
        "};
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, "19208");
    }
}
