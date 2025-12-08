use advent_of_code::utils::{parse::parse_unsigned, point3d::Point3D};
use hashbrown::HashSet;

advent_of_code::solution!(8);

#[derive(Debug)]
struct Pair {
    a: usize,
    b: usize,
    distance: f64,
}

fn parse_input(input: &str) -> Vec<Point3D> {
    parse_unsigned(input)
        .chunks_exact(3)
        .map(|c| Point3D::new(c[0], c[1], c[2]))
        .collect()
}

fn calculate_pairs(junctions: &[Point3D]) -> Vec<Pair> {
    let mut pairs = vec![];

    for i in 0..junctions.len() - 1 {
        for j in i + 1..junctions.len() {
            let distance = junctions[i].distance(junctions[j]);
            pairs.push(Pair {
                a: i,
                b: j,
                distance,
            });
        }
    }

    pairs.sort_unstable_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    pairs
}

pub fn part_one(input: &str) -> Option<u64> {
    let junctions = parse_input(input);
    let pairs = calculate_pairs(&junctions);

    let mut unions = vec![];
    for i in 0..junctions.len() {
        let mut set = HashSet::new();
        set.insert(i);
        unions.push(set);
    }

    for pair in pairs.iter().take(1000) {
        let union_a_idx = unions
            .iter()
            .position(|u: &HashSet<usize>| u.contains(&pair.a));
        let union_b_idx = unions
            .iter()
            .position(|u: &HashSet<usize>| u.contains(&pair.b));

        match (union_a_idx, union_b_idx) {
            (Some(union_a_idx), Some(union_b_idx)) => {
                if union_a_idx != union_b_idx {
                    unions[union_a_idx] = unions[union_a_idx]
                        .union(&unions[union_b_idx])
                        .copied()
                        .collect();
                    unions.remove(union_b_idx);
                }
            }
            _ => unreachable!(),
        }
    }

    unions.sort_unstable_by_key(|u| u.len());
    let result = unions
        .iter()
        .rev()
        .take(3)
        .map(|u| u.len())
        .product::<usize>();

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let junctions = parse_input(input);
    let pairs = calculate_pairs(&junctions);

    let mut unions = vec![];
    for i in 0..junctions.len() {
        let mut set = HashSet::new();
        set.insert(i);
        unions.push(set);
    }

    for pair in pairs {
        let union_a_idx = unions
            .iter()
            .position(|u: &HashSet<usize>| u.contains(&pair.a));
        let union_b_idx = unions
            .iter()
            .position(|u: &HashSet<usize>| u.contains(&pair.b));

        match (union_a_idx, union_b_idx) {
            (Some(union_a_idx), Some(union_b_idx)) => {
                if union_a_idx != union_b_idx {
                    unions[union_a_idx] = unions[union_a_idx]
                        .union(&unions[union_b_idx])
                        .copied()
                        .collect();
                    unions.remove(union_b_idx);

                    if unions.len() == 1 {
                        let result = junctions[pair.a].x * junctions[pair.b].x;
                        return Some(result as u64);
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
