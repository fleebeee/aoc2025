use hashbrown::HashMap;

advent_of_code::solution!(11);

struct ParsedInput {
    outputs: Vec<Vec<usize>>,
    you: Option<usize>,
    svr: Option<usize>,
    dac: Option<usize>,
    fft: Option<usize>,
    out: Option<usize>,
}

fn parse_input(input: &str) -> ParsedInput {
    // Instead of using a hashmap with strings, we can use
    // an array where the indices correspond to device names
    // in the order they were parsed in

    // First make a regular String -> Vec<String> hashmap,
    // then translate it to an array interface that works
    // as usize -> Vec<usize>

    let mut outputs: HashMap<String, Vec<String>> = HashMap::new();
    let mut translations = HashMap::new();

    input.lines().enumerate().for_each(|(i, line)| {
        let (name, outs) = line.split_once(": ").unwrap();
        let outs: Vec<_> = outs.split(" ").map(|s| s.to_string()).collect();

        translations.insert(name, i);
        outputs.insert(name.to_string(), outs);
    });

    translations.insert("out", translations.len());

    let mut translated: Vec<Vec<usize>> = vec![vec![]; translations.len()];

    for (name, outs) in outputs {
        let idx = translations[name.as_str()];
        translated[idx] = outs.iter().map(|v| translations[v.as_str()]).collect();
    }

    // Supply indices for PoIs
    ParsedInput {
        outputs: translated,
        you: translations.get("you").copied(),
        svr: translations.get("svr").copied(),
        dac: translations.get("dac").copied(),
        fft: translations.get("fft").copied(),
        out: translations.get("out").copied(),
    }
}

fn count(outputs: &[Vec<usize>], current: usize, idx_out: usize) -> u64 {
    let mut sum = 0;

    if current == idx_out {
        return 1;
    }

    if let Some(outs) = outputs.get(current) {
        for out in outs {
            sum += count(outputs, *out, idx_out);
        }

        sum
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let parsed = parse_input(input);

    let result = count(&parsed.outputs, parsed.you?, parsed.out?);

    Some(result)
}

fn count2(
    outputs: &Vec<Vec<usize>>,
    memo: &mut HashMap<(usize, u8), u64>,
    current: usize,
    idx_dac: usize,
    idx_fft: usize,
    idx_out: usize,
    mut visited: u8,
) -> u64 {
    if let Some(sum) = memo.get(&(current, visited)) {
        return *sum;
    }

    match current {
        x if x == idx_out => {
            return if visited == 0b11 { 1 } else { 0 };
        }
        x if x == idx_dac => visited |= 0b01,
        x if x == idx_fft => visited |= 0b10,
        _ => (),
    };

    let mut sum = 0;

    if let Some(outs) = outputs.get(current) {
        for output in outs {
            sum += count2(outputs, memo, *output, idx_dac, idx_fft, idx_out, visited);
        }

        memo.insert((current, visited), sum);

        sum
    } else {
        0
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed = parse_input(input);

    let mut memo = HashMap::new();
    let result = count2(
        &parsed.outputs,
        &mut memo,
        parsed.svr?,
        parsed.dac?,
        parsed.fft?,
        parsed.out?,
        0,
    );

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
