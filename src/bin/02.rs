use std::ops::RangeInclusive;

advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .trim()
        .split(',')
        .map(|s| {
            let (start, end) = s.split_once('-').unwrap();
            let start = start.parse().unwrap();
            let end = end.parse().unwrap();

            RangeInclusive::new(start, end)
        })
        .collect()
}

fn is_invalid_part_one(n: u64) -> bool {
    let s = n.to_string();
    let b = s.as_bytes();

    if b.len() % 2 != 0 {
        return false;
    }

    let second_half = b.len() / 2;

    for i in 0..second_half {
        if b[i] != b[second_half + i] {
            return false;
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_input(input);

    let invalid_sum = ranges
        .into_iter()
        .map(|range| range.filter(|&n| is_invalid_part_one(n)).sum::<u64>())
        .sum();

    Some(invalid_sum)
}

fn is_invalid_part_two(n: u64) -> bool {
    let s = n.to_string();
    let b = s.as_bytes();
    let pattern_size_max = b.len() / 2;

    'size: for pattern_size in 1..=pattern_size_max {
        // Only continue checking if ID is divisible by pattern size
        if b.len() % pattern_size != 0 {
            continue;
        }

        let pattern_count = b.len() / pattern_size;

        for i in 0..pattern_size {
            for j in 0..pattern_count {
                if b[j * pattern_size + i] != b[i] {
                    // Pattern is disrupted, check next one
                    continue 'size;
                }
            }
        }

        return true;
    }

    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_input(input);

    let invalid_sum = ranges
        .into_iter()
        .map(|range| range.filter(|&n| is_invalid_part_two(n)).sum::<u64>())
        .sum();

    Some(invalid_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
