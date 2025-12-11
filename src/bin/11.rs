use hashbrown::HashMap;

advent_of_code::solution!(11);

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut outputs_map = HashMap::new();

    input.lines().for_each(|line| {
        let (name, outputs) = line.split_once(": ").unwrap();
        let outputs: Vec<_> = outputs.split(" ").map(|s| s.to_string()).collect();

        outputs_map.insert(name.to_string(), outputs);
    });

    outputs_map
}

fn count(outputs_map: &HashMap<String, Vec<String>>, current: String) -> u64 {
    let mut sum = 0;

    if current == "out" {
        return 1;
    }

    if let Some(outputs) = outputs_map.get(&current) {
        for output in outputs {
            sum += count(outputs_map, output.to_string());
        }

        sum
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let output_map = parse_input(input);

    let result = count(&output_map, "you".to_string());

    Some(result)
}

fn count2(
    outputs_map: &HashMap<String, Vec<String>>,
    visited_map: &mut HashMap<(String, u8), u64>,
    current: String,
    mut visited: u8,
) -> u64 {
    if let Some(sum) = visited_map.get(&(current.clone(), visited)) {
        return *sum;
    }

    match current.as_str() {
        "out" => {
            return if visited == 0b11 { 1 } else { 0 };
        }
        "dac" => visited |= 0b01,
        "fft" => visited |= 0b10,
        _ => (),
    };

    let mut sum = 0;

    if let Some(outputs) = outputs_map.get(&current) {
        for output in outputs {
            sum += count2(outputs_map, visited_map, output.to_string(), visited);
        }

        visited_map.insert((current, visited), sum);

        sum
    } else {
        0
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let output_map = parse_input(input);

    let mut visited_map = HashMap::new();
    let result = count2(&output_map, &mut visited_map, "svr".to_string(), 0);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
