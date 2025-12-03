advent_of_code::solution!(3);

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.as_bytes().iter().map(|b| b - b'0').collect())
        .collect()
}

fn calculate_joltage<const DIGIT_COUNT: usize>(bank: &[u8]) -> u64 {
    // For each digit, try to swap with a bigger digit to its left
    let mut digits = Vec::with_capacity(DIGIT_COUNT);
    let mut left = 0;
    let mut right = bank.len() - (DIGIT_COUNT - 1); // Include current digit

    while digits.len() < DIGIT_COUNT
        && let Some((i, largest)) = &bank[left..right]
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(&a.0)))
    {
        // Update bounds
        // Note that enumerate starts at 0 for slices
        left += *i + 1;
        right += 1;

        digits.push(**largest);
    }

    digits
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, d)| acc + (*d as u64) * 10u64.pow(i as u32))
}

// Special case for part one in 0.5x the time
fn calculate_joltage_part_one(bank: &[u8]) -> u8 {
    let mut a = 0;
    let mut b = 0;
    let len = bank.len();

    for (i, battery) in bank.iter().enumerate() {
        if i < len - 1 && *battery > a {
            a = *battery;
            b = 0;
        } else if *battery > b {
            b = *battery;
        }
    }

    a * 10 + b
}

pub fn part_one(input: &str) -> Option<u64> {
    let banks = parse_input(input);
    let result = banks
        .iter()
        .map(|bank| calculate_joltage_part_one(bank) as u64)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let banks = parse_input(input);
    let result = banks.iter().map(|bank| calculate_joltage::<12>(bank)).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
