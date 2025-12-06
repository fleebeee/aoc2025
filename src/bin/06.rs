advent_of_code::solution!(6);

fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let (numbers, operators) = input.trim().rsplit_once('\n').unwrap();
    let numbers = numbers
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let operators = operators
        .split_ascii_whitespace()
        .map(|s| s.bytes().next().unwrap() as char)
        .collect();

    (numbers, operators)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (numbers, operators) = parse_input(input);
    let mut total = 0;

    for column in 0..operators.len() {
        let operator = operators[column];
        let initial = if operator == '+' { 0 } else { 1 };
        let result = numbers
            .iter()
            .map(|row| row[column])
            .fold(initial, |acc, n| match operator {
                '+' => acc + n,
                '*' => acc * n,
                _ => unreachable!(),
            });
        total += result;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (numbers, operators) = input.trim().rsplit_once('\n').unwrap();
    let mut operators: Vec<char> = operators
        .split_ascii_whitespace()
        .map(|s| s.bytes().next().unwrap() as char)
        .collect();

    let numbers: Vec<Vec<u8>> = numbers
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let w = numbers[0].len();
    let h = numbers.len();

    let mut total = 0;
    let mut operands = vec![];

    for x in (0..w).rev() {
        let mut number = 0;
        for y in 0..h {
            let digit = numbers[y][x];
            if let b'0'..=b'9' = digit {
                number = number * 10 + (digit - b'0') as u64
            }
        }

        if number != 0 {
            operands.push(number);
        }

        // if number is 0, this is a gap column
        if number == 0 || x == 0 {
            let operator = operators.pop().unwrap();
            total += match operator {
                '+' => operands.iter().sum::<u64>(),
                '*' => operands.iter().product(),
                _ => unreachable!(),
            };
            operands.clear();
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
