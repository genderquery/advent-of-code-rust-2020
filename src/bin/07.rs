use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

type Rule = (String, HashMap<String, usize>);
type Rules = HashMap<String, HashMap<String, usize>>;

fn parse_rule(input: &str) -> Rule {
    let input = input.strip_suffix('.').unwrap();
    let (color, contents) = input.split_once(" bags contain ").unwrap();
    let mut contents_map = HashMap::new();
    // 1 bright white bag, 2 muted yellow bags
    for bag in contents.split(", ") {
        if bag == "no other bags" {
            continue;
        }
        // 1 bright white bag
        let (quantity, color) = bag.split_once(' ').unwrap();
        let quantity = quantity.parse().unwrap();
        // bright white bag
        let (color, _) = color.rsplit_once(' ').unwrap();
        contents_map.insert(color.to_string(), quantity);
    }
    (color.to_string(), contents_map)
}

/// Find all bags containing `target`, directly or indirectly, given `rules`.
fn find_bags_containing(rules: &Rules, target: &str) -> HashSet<String> {
    let mut bags_containing_target = HashSet::new();

    for (bag, contents) in rules.iter() {
        if contents.contains_key(target) {
            bags_containing_target.insert(bag.clone());
            // find all bags containing this bag
            bags_containing_target.extend(find_bags_containing(rules, bag));
        }
    }

    bags_containing_target
}

/// Return the total amount of bags inside `start`.
fn bag_depth(rules: &Rules, start: &str) -> usize {
    rules
        .get(start)
        .unwrap()
        .iter()
        .map(|(bag, quantity)| quantity * (1 + bag_depth(rules, bag)))
        .sum()
}

fn parse(input: &str) -> Rules {
    input.lines().map(parse_rule).collect()
}

pub fn part_one(input: &str) -> Option<String> {
    let rules = parse(input);
    let bags = find_bags_containing(&rules, "shiny gold");
    bags.len().to_string().into()
}

pub fn part_two(input: &str) -> Option<String> {
    let rules = parse(input);
    let depth = bag_depth(&rules, "shiny gold");
    depth.to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        const INPUT: &str = indoc! {"
            light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.
        "};
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, "4");
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = indoc! {"
            light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.
        "};
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, "32");
    }
}
