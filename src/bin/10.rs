use std::collections::VecDeque;

use hashbrown::HashSet;
use rayon::prelude::*;

advent_of_code::solution!(10);

type Lights = Vec<bool>;
type Buttons = Vec<Vec<usize>>;
type Joltages = Vec<u64>;

#[derive(Debug, Clone)]
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

#[derive(Clone)]
struct Matrix {
    raw: Vec<f64>,
    width: usize,
    height: usize,
}

impl Matrix {
    fn get(&self, i: usize, j: usize) -> f64 {
        self.raw[self.width * i + j]
    }

    fn set(&mut self, i: usize, j: usize, value: f64) {
        self.raw[self.width * i + j] = value;
    }

    fn swap(&mut self, i_1: usize, i_2: usize) {
        for j in 0..self.width {
            (
                self.raw[i_1 * self.width + j],
                self.raw[i_2 * self.width + j],
            ) = (
                self.raw[i_2 * self.width + j],
                self.raw[i_1 * self.width + j],
            );
        }
    }
}

fn get_matrix(machine: &Machine) -> Matrix {
    let mut raw = vec![];
    let target = &machine.joltages_target;

    let m = target.len();
    let n = machine.buttons.len();

    for i in 0..m {
        for j in 0..n {
            raw.push(if machine.buttons[j].contains(&i) {
                1f64
            } else {
                0f64
            });
        }

        raw.push(target[i] as f64);
    }

    Matrix {
        raw,
        width: n + 1,
        height: m,
    }
}

fn find_min_steps(machine: &Machine) -> Option<u64> {
    let mut matrix = get_matrix(machine);
    let m = matrix.height;
    let n = matrix.width;

    // Use Gauss elimination for the matrix

    // Example:
    // Original
    // 1    1    1    0   10
    // 1    1    0    1   11
    // 1    1    0    1   11
    // 1    0    1    0    5
    // 1    1    1    0   10
    // 0    1    0    0    5

    // Gaussed
    // 1    1    1    0   10
    // 0    1    0    0    5
    // 0    0    1   -1   -1
    // 0    0    0    0    0
    // 0    0    0    0    0
    // 0    0    0    0    0

    let mut h = 0;
    let mut k = 0;

    while h < m && k < n - 1 {
        let i_max = (h..m)
            .max_by(|a, b| {
                matrix
                    .get(*a, k)
                    .abs()
                    .partial_cmp(&matrix.get(*b, k).abs())
                    .unwrap()
            })
            .unwrap();

        if matrix.get(i_max, k).abs() < 1e-10 {
            k += 1;
        } else {
            matrix.swap(h, i_max);

            // Normalize pivot row
            let pivot = matrix.get(h, k);
            for j in k..n {
                matrix.set(h, j, matrix.get(h, j) / pivot);
            }
            // Force pivot to exactly 1.0
            matrix.set(h, k, 1.0);

            for i in h + 1..m {
                let f = matrix.get(i, k);
                matrix.set(i, k, 0.0);
                for j in k + 1..n {
                    matrix.set(i, j, matrix.get(i, j) - matrix.get(h, j) * f);
                }
            }

            h += 1;
            k += 1;
        }
    }

    // Back substitution
    // Keep in mind constraints:
    // 1. We can't have negative button presses (n >= 0)
    // 2. n must be an integer

    let mut pivots = vec![];
    for i in 0..m {
        for j in 0..n - 1 {
            let val = matrix.get(i, j);
            if val.abs() < 1e-10 {
                continue;
            } else {
                pivots.push(j);
                break;
            }
        }
    }

    let free_variables: Vec<usize> = (0..n - 1).filter(|j| !pivots.contains(j)).collect();

    let mut solution = vec![0.0; n - 1];
    let mut least = None;

    back_substitute(
        machine,
        &matrix,
        &mut solution,
        0,
        &pivots,
        &free_variables,
        &mut least,
    );

    least.map(|v| v as u64)
}

fn back_substitute(
    machine: &Machine,
    matrix: &Matrix,
    solution: &mut Vec<f64>,
    nth_free: usize,
    pivots: &[usize],
    free_variables: &[usize],
    least: &mut Option<usize>,
) {
    let n = matrix.width;

    // Guesses for free variables are in, try to work
    // out the variables
    if nth_free == free_variables.len() {
        // Fill in free variables first
        let mut presses = 0;
        for j in free_variables {
            presses += solution[*j] as usize;
        }

        // Start working through pivots from bottom to top
        for r in (0..pivots.len()).rev() {
            let j_p = pivots[r];
            let mut target = matrix.get(r, n - 1);
            for j in j_p + 1..n - 1 {
                target -= matrix.get(r, j) * solution[j];
            }

            let rounded = target.round();

            // Reject non-integers and negative numbers
            if (target - rounded).abs() > 1e-4 || rounded < -1e-10 {
                return;
            }

            solution[j_p] = rounded;
            presses += rounded as usize;
        }

        if least.is_none() || presses < least.unwrap() {
            *least = Some(presses);
        }
    } else {
        let j = free_variables[nth_free];

        // A button can be pressed at most as many times
        // as the smallest joltage target it has an effect towards
        let max = machine.buttons[j]
            .iter()
            .map(|&i| machine.joltages_target[i])
            .min()
            .unwrap();

        for guess in 0..=max {
            solution[j] = guess as f64;
            back_substitute(
                machine,
                matrix,
                solution,
                nth_free + 1,
                pivots,
                free_variables,
                least,
            );
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse_input(input);

    let result = machines.par_iter().filter_map(find_min_steps).sum();

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
