advent_of_code::solution!(5);

fn row(input: &str) -> usize {
    let mut low = 0;
    let mut high = 127;
    for c in input[0..=6].chars() {
        if c == 'F' {
            high = (low + high) / 2;
        } else {
            low = ((low + high) / 2) + 1;
        }
    }
    if input.chars().nth(7).unwrap() == 'F' {
        low
    } else {
        high
    }
}

fn col(input: &str) -> usize {
    let mut low = 0;
    let mut high = 7;
    for c in input[7..=8].chars() {
        if c == 'L' {
            high = (low + high) / 2;
        } else {
            low = ((low + high) / 2) + 1;
        }
    }
    if input.chars().nth(9).unwrap() == 'L' {
        low
    } else {
        high
    }
}

fn seat_id(input: &str) -> usize {
    row(input) * 8 + col(input)
}

pub fn part_one(input: &str) -> Option<String> {
    input.lines().map(seat_id).max().unwrap().to_string().into()
}

pub fn part_two(input: &str) -> Option<String> {
    let mut seat_ids: Vec<usize> = input.lines().map(seat_id).collect();
    seat_ids.sort();
    let min = seat_ids.iter().min().unwrap();
    let max = seat_ids.iter().max().unwrap();
    let (_, seat_id) = seat_ids
        .iter()
        .zip(*min..=*max)
        .find(|(a, b)| **a != *b)
        .unwrap();
    seat_id.to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_id() {
        assert_eq!(row("FBFBBFFRLR"), 44);
        assert_eq!(col("FBFBBFFRLR"), 5);
        assert_eq!(seat_id("FBFBBFFRLR"), 357);
        assert_eq!(row("BFFFBBFRRR"), 70);
        assert_eq!(col("BFFFBBFRRR"), 7);
        assert_eq!(seat_id("BFFFBBFRRR"), 567);
        assert_eq!(row("FFFBBBFRRR"), 14);
        assert_eq!(col("FFFBBBFRRR"), 7);
        assert_eq!(seat_id("FFFBBBFRRR"), 119);
        assert_eq!(row("BBFFBBFRLL"), 102);
        assert_eq!(col("BBFFBBFRLL"), 4);
        assert_eq!(seat_id("BBFFBBFRLL"), 820);
    }
}
