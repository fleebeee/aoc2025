use advent_of_code::utils::{
    grid::Grid,
    point::{DOWN, LEFT, Point, RIGHT},
};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = Grid::parse(input);
    let mut splits = 0;

    for y in 0..grid.height - 1 {
        for x in 0..grid.width {
            let point = Point::new(x, y);

            match grid[point] {
                b'S' | b'|' => {
                    let next = point + DOWN;
                    match grid[next] {
                        b'^' => {
                            grid[next + LEFT] = b'|';
                            grid[next + RIGHT] = b'|';
                            splits += 1;
                        }
                        _ => grid[next] = b'|',
                    }
                }
                _ => (),
            }
        }
    }

    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::parse(input);
    let mut counts = Grid::new(grid.width, grid.height, 0u64);

    for y in 0..grid.height - 1 {
        for x in 0..grid.width {
            let point = Point::new(x, y);

            let count = match grid[point] {
                b'S' => 1,
                b'|' => counts[point],
                _ => 0,
            };

            match grid[point] {
                b'S' | b'|' => {
                    let next = point + DOWN;
                    match grid[next] {
                        b'^' => {
                            grid[next + LEFT] = b'|';
                            grid[next + RIGHT] = b'|';
                            counts[next + LEFT] += count;
                            counts[next + RIGHT] += count;
                        }
                        _ => {
                            grid[next] = b'|';
                            counts[next] += count;
                        }
                    }
                }
                _ => (),
            }
        }
    }

    let total = (0..counts.width)
        .map(|x| counts[Point::new(x, counts.height - 1)])
        .sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
