use std::ops::RangeInclusive;

use advent_of_code::utils::range::{Range, union_vec};
use range_set::RangeSet;

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (fresh_ranges, inventory) = input.split_once("\n\n").unwrap();

    let fresh_ranges = fresh_ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    let inventory = inventory
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (fresh_ranges, inventory)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (fresh_ranges, inventory) = parse_input(input);

    let range_set = RangeSet::<[RangeInclusive<u64>; 4]>::from_ranges(fresh_ranges);

    let total = inventory
        .into_iter()
        .filter(|ingredient| range_set.contains(*ingredient))
        .count();

    Some(total as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (fresh_ranges, _) = parse_input(input);

    // Transform the ranges into exclusive ranges
    // that this repo has utils for
    let custom_ranges: Vec<Range<u64>> = fresh_ranges
        .iter()
        .map(|r| Range::new(*r.start(), *r.end() + 1))
        .collect();

    // union_vec takes in an arbitrary vector of ranges and returns
    // a sorted and merged vector of ranges with no overlaps
    let union = union_vec(&custom_ranges);

    let result = union.iter().map(|r| r.end - r.start).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
