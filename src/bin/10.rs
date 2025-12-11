use pathfinding::prelude::astar;
use std::collections::VecDeque;

use hashbrown::HashSet;
use rayon::prelude::*;

advent_of_code::solution!(10);

type Lights = Vec<bool>;
type Buttons = Vec<Vec<usize>>;
type Joltages = Vec<u64>;

#[derive(Debug)]
struct Machine {
    lights_target: Lights,
    buttons: Buttons,
    joltages_target: Joltages,
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let (lights, rest) = line.split_once(' ').unwrap();
            let (buttons, joltages) = rest.rsplit_once(' ').unwrap();

            let mut machine = Machine {
                lights_target: lights.as_bytes()[1..lights.len() - 1]
                    .iter()
                    .map(|b| match b {
                        b'#' => true,
                        b'.' => false,
                        _ => unreachable!(),
                    })
                    .collect(),
                buttons: buttons
                    .split(' ')
                    .map(|sequence| {
                        sequence[1..sequence.len() - 1]
                            .split(',')
                            .map(|button| button.parse::<usize>().unwrap())
                            .collect()
                    })
                    .collect(),
                joltages_target: joltages[1..joltages.len() - 1]
                    .split(',')
                    .map(|joltage| joltage.parse::<u64>().unwrap())
                    .collect(),
            };

            machine
                .buttons
                .sort_by_key(|button| usize::MAX - button.len());

            machine
        })
        .collect()
}

fn find_shortest_light_sequence(machine: &Machine) -> Option<u64> {
    let target: u128 = machine.lights_target.iter().rev().fold(0, |acc, joltage| {
        let joltage = if *joltage { 1 } else { 0 };
        (acc << 1) | joltage
    });

    let mut frontier = VecDeque::new();
    frontier.push_back((0, 0));
    let mut visited = HashSet::new();

    while let Some((presses, lights)) = frontier.pop_front() {
        if lights == target {
            return Some(presses);
        }

        visited.insert(lights);

        for button in &machine.buttons {
            let mut next = lights;
            for i in button {
                next ^= 1 << i;
            }

            if visited.contains(&next) {
                continue;
            }

            frontier.push_back((presses + 1, next));
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_input(input);

    let result = machines
        .par_iter()
        .map(|machine| find_shortest_light_sequence(machine).unwrap())
        .sum::<u64>();

    Some(result)
}

// fn add(container: u128, index: u128, value: u128) -> u128 {
//     container + (value << (index * 8))
// }

fn unpack(container: u128, index: u128) -> u128 {
    (container >> (index * 8)) & 0xFF
}

fn find_shortest_joltage_sequence(machine: &Machine) -> Option<u64> {
    let target: u128 = machine
        .joltages_target
        .iter()
        .rev()
        .fold(0, |acc, joltage| (acc << 8) | *joltage as u128);

    let mut frontier = Vec::new();
    frontier.push((0, 0));

    let mut visited = HashSet::new();
    visited.insert(0);

    let mut largest = 0;

    while let Some((presses, joltages)) = frontier.pop() {
        if joltages == target {
            return Some(presses);
        }

        if presses > largest {
            // println!("Depth: {}", presses);
            // let unpacked: Vec<u8> = (0..machine.joltages_target.len())
            //     .map(|i| ((joltages >> (i * 8)) & 0xFF) as u8)
            //     .collect();
            // println!("Joltages: {:?}", unpacked);
            largest = presses;
        }

        // For every joltage, check that it can be satisfied
        // theoretically without overflowing something else
        // for i in 0..machine.joltages_target.len() {
        //     let diff = unpack(joltages, i as u128);
        //     let matching_buttons: Vec<&Vec<usize>> = machine
        //         .buttons
        //         .iter()
        //         .filter(|button| button.contains(&i))
        //         .collect();

        //     let overflows: Vec<Vec<usize>> = matching_buttons
        //         .iter()
        //         .map(|button| {
        //             let mut theoretical_joltages = joltages;
        //             for i in *button {
        //                 theoretical_joltages = add(theoretical_joltages, *i as u128, diff);
        //             }
        //             let overflows: Vec<_> = machine
        //                 .joltages_target
        //                 .iter()
        //                 .enumerate()
        //                 .filter(|(i, joltage)| unpack(joltages, *i as u128) > **joltage as u128)
        //                 .map(|(i, _)| i)
        //                 .collect();

        //             overflows
        //         })
        //         .collect();

        //     for i in 0..machine.joltages_target.len() {
        //         if overflows.iter().any(|overflow| overflow.contains(&i)) {
        //             continue 'outer2;
        //         }
        //     }
        // }

        // dbg!(&machine.buttons);

        'outer: for button in machine.buttons.iter().rev() {
            let mut next = joltages;
            for i in button {
                next += 1 << (i * 8);

                if unpack(next, *i as u128) > unpack(target, *i as u128) {
                    continue 'outer;
                }
            }

            if visited.insert(next) {
                frontier.push((presses + 1, next));
            }
        }
    }

    None
}

#[allow(dead_code)]
fn find_shortest_joltage_sequence2(machine: &Machine) -> u64 {
    let mut frontier = Vec::new();
    frontier.push((0, 0, vec![0; machine.joltages_target.len()]));

    let mut lowest = u64::MAX;

    while let Some((presses, index, joltage)) = frontier.pop() {
        if joltage == machine.joltages_target && presses < lowest {
            println!("Lowest: {presses}");
            lowest = presses;
        }

        if index == machine.buttons.len() {
            // println!("Hit the end, presses: {presses}");
            continue;
        }

        let n_max = machine.buttons[index]
            .iter()
            .map(|i| machine.joltages_target[*i] - joltage[*i])
            .min()
            .unwrap();

        // println!(
        //     "Can apply button {:?} at most {n_max} times",
        //     machine.buttons[index],
        // );

        for n in 0..=n_max {
            let mut next = joltage.clone();
            machine.buttons[index].iter().for_each(|i| next[*i] += n);

            // println!("Adding {:?}", &next);
            frontier.push((presses + n, index + 1, next));
        }
    }

    lowest
}

#[allow(dead_code)]
fn find_shortest_joltage_sequence3(machine: &Machine) -> u64 {
    let target = &machine.joltages_target;
    let result = astar(
        &vec![0u64; target.len()],
        |joltage: &Vec<u64>| {
            machine
                .buttons
                .iter()
                .filter_map(|button| {
                    let mut next = joltage.clone();
                    for i in button {
                        next[*i] += 1;
                        if next[*i] > target[*i] {
                            return None;
                        }
                    }

                    Some((next, 1))
                })
                .collect::<Vec<_>>()
        },
        |joltage: &Vec<u64>| {
            (0..target.len())
                .map(|i| target[i] - joltage[i])
                .max()
                .unwrap()
        },
        |joltage| joltage == target,
    )
    .unwrap();

    result.1
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse_input(input);

    let result = machines
        .iter()
        .enumerate()
        .map(|(i, machine)| {
            println!("Machine {}", i + 1);
            find_shortest_joltage_sequence(machine).unwrap()
        })
        .sum::<u64>();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
