advent_of_code::solution!(1);

const DIAL_SIZE: i64 = 100;
const DIAL_INITIAL: i64 = 50;

fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_at(1);
            let distance: i64 = distance.parse().unwrap();
            let sign = if direction.starts_with('L') { -1 } else { 1 };

            distance * sign
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = DIAL_INITIAL;
    let instructions = parse_input(input);
    let count = instructions
        .into_iter()
        .filter(|instruction| {
            dial = (dial + (instruction % DIAL_SIZE) + DIAL_SIZE) % DIAL_SIZE;
            dial == 0
        })
        .count();

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial = DIAL_INITIAL;
    let instructions = parse_input(input);
    let count = instructions
        .into_iter()
        .map(|instruction| {
            let mut zeros = 0;
            let full_rotations = instruction / DIAL_SIZE;

            zeros += full_rotations.abs();

            let remainder = instruction - (full_rotations * DIAL_SIZE);
            let next = dial + remainder;

            if dial == 0 {
                // If dial started on a zero, full rotations are all
                // we care about, so just adjust dial
                dial = (next + DIAL_SIZE) % DIAL_SIZE;
            } else {
                // We go past zero
                if !(0..DIAL_SIZE + 1).contains(&next) {
                    zeros += 1;
                }

                dial = (next + DIAL_SIZE) % DIAL_SIZE;

                // Or we end up on zero
                if dial == 0 {
                    zeros += 1;
                }
            }

            zeros
        })
        .sum::<i64>();

    Some(count as u64)
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
        assert_eq!(result, Some(6));
    }
}
