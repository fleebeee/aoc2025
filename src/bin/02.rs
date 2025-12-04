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

fn count_digits(n: u64) -> u64 {
    if n == 0 { 1 } else { (n.ilog10() + 1) as u64 }
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_input(input);

    let mut total = 0;

    for range in ranges {
        let start = *range.start();
        let end = *range.end();
        let min_digits = count_digits(start);
        let max_digits = count_digits(end);

        // Some ranges span multiple digit counts
        for len in min_digits..=max_digits {
            if len % 2 != 0 {
                continue;
            }

            let pattern_len = len / 2;

            // We can calculate all the invalid IDs
            // as follows:
            // seed * multiplier = id
            // 1 * 11 = 11
            // 12 * 101 = 1212
            // 123 * 1001 = 123123

            let multiplier = 10u64.pow(pattern_len as u32) + 1;

            // Figure out min and max ranges for seeds
            // by division and ensuring digit count is correct
            // seed * multiplier <= max
            // seed * multiplier >= min

            let seed_min = (start.div_ceil(multiplier)).max(10u64.pow((pattern_len - 1) as u32));
            let seed_max = (end / multiplier).min(10u64.pow(pattern_len as u32) - 1);

            if seed_min > seed_max {
                continue;
            }

            // We know all the seeds, just do arithmetic sum
            let n = seed_max - seed_min + 1;
            let sum = n * (seed_min + seed_max) / 2;
            total += sum * multiplier;
        }
    }

    Some(total)
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
