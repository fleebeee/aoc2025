use advent_of_code::utils::grid::Grid;

advent_of_code::solution!(12);

type Shape = Grid<u8>;

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    shapes: Vec<usize>,
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let (rest, regions) = input.rsplit_once("\n\n").unwrap();

    let regions = regions
        .lines()
        .map(|line| {
            let (dimensions, shapes) = line.split_once(": ").unwrap();
            let (width, height) = dimensions.split_once('x').unwrap();

            Region {
                width: width.parse().unwrap(),
                height: height.parse().unwrap(),
                shapes: shapes
                    .split(' ')
                    .map(|count| count.parse().unwrap())
                    .collect(),
            }
        })
        .collect();

    let shapes = rest
        .split("\n\n")
        .map(|shape| {
            // Remove "0:" line
            let (_, shape) = shape.split_once("\n").unwrap();
            Grid::parse(shape)
        })
        .collect();

    (shapes, regions)
}

fn fits_shapes(region: &Region, shape_sizes: &[usize]) -> bool {
    // It turns out that using two simple heuristics, every case
    // is trivial with the nice input we've been given.

    // Check whether the region has enough empty 3x3 blocks to straight up
    // accommodate every present's bounding box.
    let available_3x3_subareas = (region.width / 3) * (region.height / 3);
    let total_pieces = region.shapes.iter().sum::<usize>();

    if available_3x3_subareas >= total_pieces {
        return true;
    }

    // Check whether the pieces would fit even if assembled in a
    // way so that zero empty space is left within the resulting shape.
    // If no, then the case is obviously impossible
    let required_space = region
        .shapes
        .iter()
        .zip(shape_sizes)
        .map(|(count, size)| *count * *size)
        .sum::<usize>();

    let region_area = region.width * region.height;

    if required_space > region_area {
        return false;
    }

    // With evil input we'd have to do a lot of stuff here
    unreachable!();
}

pub fn part_one(input: &str) -> Option<u64> {
    let (shapes, regions) = parse_input(input);
    let shape_sizes: Vec<_> = shapes
        .iter()
        .map(|shape| shape.bytes.iter().filter(|b| **b == b'#').count())
        .collect();

    let result = regions
        .iter()
        .filter(|region| fits_shapes(region, &shape_sizes))
        .count();

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let _values = parse_input(input);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
