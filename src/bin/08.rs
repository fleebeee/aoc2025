use advent_of_code::utils::{parse::parse_unsigned, point3d::Point3D};

advent_of_code::solution!(8);

#[derive(Debug)]
struct Pair {
    a: usize,
    b: usize,
    distance: i64,
}

fn parse_input(input: &str) -> Vec<Point3D> {
    parse_unsigned(input)
        .chunks_exact(3)
        .map(|c| Point3D::new(c[0], c[1], c[2]))
        .collect()
}

fn calculate_pairs(junctions: &[Point3D]) -> Vec<Pair> {
    let mut pairs = Vec::with_capacity(junctions.len().pow(2));

    for i in 0..junctions.len() - 1 {
        for j in i + 1..junctions.len() {
            // Save some time by not taking the square root
            let distance = {
                let a = junctions[i];
                let b = junctions[j];
                (a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)
            };

            pairs.push(Pair {
                a: i,
                b: j,
                distance,
            });
        }
    }

    pairs.sort_unstable_by_key(|a| a.distance);

    pairs
}

struct Circuit {
    parent: usize,
    size: usize,
}

fn find(circuits: &mut [Circuit], id: usize) -> usize {
    if circuits[id].parent != id {
        find(circuits, circuits[id].parent)
    } else {
        id
    }
}

fn union(circuits: &mut [Circuit], id_a: usize, id_b: usize) -> usize {
    let a = find(circuits, id_a);
    let b = find(circuits, id_b);

    if a != b {
        circuits[b].parent = a;
        circuits[a].size += circuits[b].size;
    }

    // Return resulting size for part 2
    circuits[a].size
}

pub fn part_one(input: &str) -> Option<u64> {
    let junctions = parse_input(input);
    let pairs = calculate_pairs(&junctions);

    let mut circuits: Vec<Circuit> = (0..junctions.len())
        .map(|i| Circuit { parent: i, size: 1 })
        .collect();

    for pair in pairs.iter().take(1000) {
        union(&mut circuits, pair.a, pair.b);
    }

    let mut roots: Vec<usize> = circuits
        .iter()
        .enumerate()
        .filter_map(|(i, circuit)| {
            if circuit.parent == i {
                Some(circuit.size)
            } else {
                None
            }
        })
        .collect();

    roots.sort_unstable();

    let result = roots.iter().rev().take(3).product::<usize>();

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let junctions = parse_input(input);
    let pairs = calculate_pairs(&junctions);

    let mut circuits: Vec<Circuit> = (0..junctions.len())
        .map(|i| Circuit { parent: i, size: 1 })
        .collect();

    for pair in &pairs {
        let size = union(&mut circuits, pair.a, pair.b);

        if size == junctions.len() {
            let result = junctions[pair.a].x * junctions[pair.b].x;
            return Some(result as u64);
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
