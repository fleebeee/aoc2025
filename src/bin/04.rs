use advent_of_code::utils::{
    grid::Grid,
    point::{DIAGONAL, Point},
};

advent_of_code::solution!(4);

fn parse_input(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let mut total = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);

            if grid[point] != b'@' {
                continue;
            }

            let count = DIAGONAL
                .iter()
                .filter(|d| {
                    let n = point + **d;
                    n.x >= 0 && n.y >= 0 && n.x < grid.width && n.y < grid.height && grid[n] == b'@'
                })
                .count();

            if count < 4 {
                total += 1;
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    // New grid that holds the number of neighboring rolls
    // for each roll cell
    let mut count_grid = Grid::new(grid.width, grid.height, i32::MAX);
    let mut todo = vec![];

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);

            if grid[point] != b'@' {
                continue;
            }

            let count = DIAGONAL
                .iter()
                .filter(|d| {
                    let n = point + **d;
                    n.x >= 0 && n.y >= 0 && n.x < grid.width && n.y < grid.height && grid[n] == b'@'
                })
                .count();

            count_grid[point] = count as i32;

            if count < 4 {
                todo.push(point);
            }
        }
    }

    let mut total = 0;

    // Start deleting rolls
    while let Some(point) = todo.pop() {
        total += 1;

        for neighbor in DIAGONAL
            .into_iter()
            .map(|d| point + d)
            .filter(|n| n.x >= 0 && n.y >= 0 && n.x < grid.width && n.y < grid.height)
        {
            // If a roll causes a neighbor roll to become
            // accessible, add that to todo
            if count_grid[neighbor] == 4 {
                todo.push(neighbor);
            }

            count_grid[neighbor] -= 1;
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
