advent_of_code::solution!(9);

struct RingBuffer {
    capacity: usize,
    position: usize,
    inner: Vec<u64>,
}

impl RingBuffer {
    pub fn push(&mut self, value: u64) {
        self.inner[self.position] = value;
        self.position = (self.position + 1) % self.capacity;
    }
}

impl From<&[u64]> for RingBuffer {
    fn from(value: &[u64]) -> Self {
        RingBuffer {
            capacity: value.len(),
            position: 0,
            inner: Vec::from(value),
        }
    }
}

fn parse(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

/// Determine if a pair of integers in `numbers` exists that sum to `target_sum`.
pub fn has_sum_pair(numbers: &[u64], target_sum: u64) -> bool {
    for i in 0..(numbers.len() - 1) {
        for j in 1..numbers.len() {
            if numbers[i] + numbers[j] == target_sum {
                return true;
            }
        }
    }
    false
}

/// Find a contiguous range of integers in `numbers` that sum to `target_sum`.
fn find_contiguous_sum(numbers: &[u64], target_sum: u64) -> Option<(usize, usize)> {
    let mut start = 0;
    let mut end = 0;
    let mut current_sum = 0;

    while end < numbers.len() {
        if current_sum == target_sum {
            return Some((start, end));
        }

        if current_sum < target_sum {
            current_sum += numbers[end];
            end += 1;
        } else {
            current_sum -= numbers[start];
            start += 1;
        }
    }

    None
}

/// Find the first integer in `numbers` after `preamble_len` integers that is
/// not the sum of two of the `preamble_len` integers before it.
fn find_weakness_1(numbers: &[u64], preamble_len: usize) -> Option<u64> {
    let mut buffer = RingBuffer::from(&numbers[..preamble_len]);
    for &number in &numbers[preamble_len..] {
        if !has_sum_pair(&buffer.inner, number) {
            return Some(number);
        }
        buffer.push(number);
    }
    None
}

/// Find the sum of the lowest and highest integer  in a contiguous range that
/// sums to `target_sum`.
fn find_weakness_2(numbers: &[u64], target_sum: u64) -> Option<u64> {
    if let Some((i, j)) = find_contiguous_sum(numbers, target_sum) {
        let low = numbers[i..j].iter().min().unwrap();
        let high = numbers[i..j].iter().max().unwrap();
        return Some(low + high);
    }
    None
}

pub fn part_one(input: &str) -> Option<String> {
    let numbers = parse(input);
    find_weakness_1(&numbers, 25).unwrap().to_string().into()
}

pub fn part_two(input: &str) -> Option<String> {
    let numbers = parse(input);
    let target_sum = find_weakness_1(&numbers, 25).unwrap();
    find_weakness_2(&numbers, target_sum)
        .unwrap()
        .to_string()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_buffer() {
        let numbers: Vec<u64> = (1..=5).collect();
        let mut buffer = RingBuffer::from(&numbers[..]);
        assert_eq!(buffer.inner, vec![1, 2, 3, 4, 5]);
        buffer.push(6);
        buffer.push(7);
        buffer.push(8);
        assert_eq!(buffer.inner, vec![6, 7, 8, 4, 5]);
    }

    #[test]
    fn test_has_pair() {
        let numbers: Vec<u64> = (1..=25).collect();
        let buffer = RingBuffer::from(&numbers[..]);
        assert!(has_sum_pair(&buffer.inner, 26));
        assert!(has_sum_pair(&buffer.inner, 49));
        assert!(!has_sum_pair(&buffer.inner, 100));
        assert!(!has_sum_pair(&buffer.inner, 50));
    }

    #[test]
    fn test_find_contiguous() {
        let numbers = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
        ];
        assert_eq!(find_contiguous_sum(&numbers, 127).unwrap(), (2, 6))
    }

    #[test]
    fn test_find_weakness_1() {
        const INPUT: &str = indoc! {"
            35
            20
            15
            25
            47
            40
            62
            55
            65
            95
            102
            117
            150
            182
            127
            219
            299
            277
            309
            576
        "};
        let numbers = parse(INPUT);
        let result = find_weakness_1(&numbers, 5).unwrap();
        assert_eq!(result, 127);
    }

    #[test]
    fn test_find_weakness_2() {
        const INPUT: &str = indoc! {"
            35
            20
            15
            25
            47
            40
            62
            55
            65
            95
            102
            117
            150
            182
            127
            219
            299
            277
            309
            576
        "};
        let numbers = parse(INPUT);
        let result = find_weakness_2(&numbers, 127).unwrap();
        assert_eq!(result, 62);
    }
}
