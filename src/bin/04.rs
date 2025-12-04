use advent_of_code::utils::{grid::Grid, point::Point};

advent_of_code::solution!(4);

fn parse_input(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn is_accessible(grid: &Grid<u8>, point: Point) -> bool {
    if grid[point] != b'@' {
        return false;
    }

    let mut count = 0;

    // Loop through orthogonal + diagonal neighbors
    for y_n in (point.y - 1).max(0)..=(point.y + 1).min(grid.height - 1) {
        for x_n in (point.x - 1).max(0)..=(point.x + 1).min(grid.width - 1) {
            let neighbor = Point::new(x_n, y_n);

            if point == neighbor {
                continue;
            }

            if grid[neighbor] == b'@' {
                count += 1;
            }
        }
    }

    count < 4
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let mut accessibles = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);

            if is_accessible(&grid, point) {
                accessibles += 1;
            }
        }
    }

    Some(accessibles as u64)
}

fn attempt_remove(grid: &mut Grid<u8>, point: Point) -> bool {
    if is_accessible(grid, point) {
        grid[point] = b'.';
        return true;
    }

    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_input(input);
    let mut touched;
    let mut count = 0;

    loop {
        touched = false;

        for y in 0..grid.height {
            for x in 0..grid.width {
                let point = Point::new(x, y);

                if attempt_remove(&mut grid, point) {
                    count += 1;
                    touched = true;

                    // As an optimization, re-check previous neighbors
                    // (2x speed up)
                    count += [
                        Point::new((x - 1).max(0), y),
                        Point::new(x, (y - 1).max(0)),
                        Point::new((x - 1).max(0), (y - 1).max(0)),
                        Point::new((x + 1).min(grid.width - 1), (y - 1).max(0)),
                    ]
                    .iter()
                    .filter(|n| attempt_remove(&mut grid, **n))
                    .count();
                }
            }
        }

        if !touched {
            break;
        }
    }
    Some(count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
