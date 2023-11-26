use std::collections::HashMap;

advent_of_code::solution!(4);

#[derive(Debug, PartialEq, Eq)]
struct Passport<'a>(HashMap<&'a str, &'a str>);
type Batch<'a> = Vec<Passport<'a>>;

impl<'a> Passport<'a> {
    pub fn is_valid_1(&self) -> bool {
        self.0.contains_key("byr")
            && self.0.contains_key("iyr")
            && self.0.contains_key("eyr")
            && self.0.contains_key("hgt")
            && self.0.contains_key("hcl")
            && self.0.contains_key("ecl")
            && self.0.contains_key("pid")
    }

    pub fn is_valid_2(&self) -> bool {
        self.is_valid_1()
            && is_year_valid(self.0["byr"], 1920, 2002)
            && is_year_valid(self.0["iyr"], 2010, 2020)
            && is_year_valid(self.0["eyr"], 2020, 2030)
            && is_height_valid(self.0["hgt"])
            && is_hair_color_valid(self.0["hcl"])
            && is_eye_color_valid(self.0["ecl"])
            && is_pid_valid(self.0["pid"])
    }
}

fn is_year_valid(input: &str, min: u16, max: u16) -> bool {
    if let Ok(year) = input.parse::<u16>() {
        (min..=max).contains(&year)
    } else {
        false
    }
}

fn is_height_valid(input: &str) -> bool {
    if input.ends_with("cm") {
        if let Ok(height) = input.strip_suffix("cm").unwrap().parse::<u16>() {
            (150..=193).contains(&height)
        } else {
            false
        }
    } else if input.ends_with("in") {
        if let Ok(height) = input.strip_suffix("in").unwrap().parse::<u16>() {
            (59..=76).contains(&height)
        } else {
            false
        }
    } else {
        false
    }
}

fn is_hair_color_valid(input: &str) -> bool {
    input.starts_with('#')
        && input
            .strip_prefix('#')
            .unwrap()
            .chars()
            .all(|c| c.is_ascii_hexdigit())
}

fn is_eye_color_valid(input: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&input)
}

fn is_pid_valid(input: &str) -> bool {
    input.len() == 9 && input.chars().all(|c| c.is_ascii_digit())
}

fn parse(input: &str) -> Batch {
    input
        .split("\n\n")
        .map(|passport| {
            Passport(
                passport
                    .split_whitespace()
                    .map(|field| field.split_once(':').unwrap())
                    .collect(),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<String> {
    parse(input)
        .iter()
        .filter(|p| p.is_valid_1())
        .count()
        .to_string()
        .into()
}

pub fn part_two(input: &str) -> Option<String> {
    parse(input)
        .iter()
        .filter(|p| p.is_valid_2())
        .count()
        .to_string()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        const INPUT: &str = indoc! {"
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm

            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929

            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm

            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in
        "};
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = indoc! {"
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm

            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929

            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm

            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in
        "};
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, "2");
    }
}
