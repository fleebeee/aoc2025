use std::ops::RangeInclusive;

use advent_of_code::utils::{parse::parse_unsigned, point::Point};
// use rangemap::RangeInclusiveMap;

advent_of_code::solution!(9);

fn parse_input(input: &str) -> Vec<Point> {
    parse_unsigned(input)
        .chunks_exact(2)
        .map(|c| Point::new(c[0], c[1]))
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse_input(input);

    let mut largest = 0;

    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let a = points[i];
            let b = points[j];
            let size = (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1);
            largest = largest.max(size);
        }
    }

    Some(largest)
}

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn range_intersect(
    a: &RangeInclusive<i64>,
    b: &RangeInclusive<i64>,
) -> Option<RangeInclusive<i64>> {
    if b.end() < a.start() || a.end() < b.start() {
        return None;
    }

    let start = *std::cmp::max(a.start(), b.start());
    let end = *std::cmp::min(a.end(), b.end());

    Some(start..=end)
}

fn bb_intersect(bb: &(Point, Point), point: Point) -> bool {
    bb.0.x <= point.x && bb.1.x >= point.x && bb.0.y <= point.y && bb.1.y >= point.y
}

fn validate(bounding_boxes: &[(Point, Point)], a: Point, b: Point) -> bool {
    // Validate that given area is completely within one or more
    // bounding boxes

    // nw >>>> ne
    // v       v
    // sw >>>> se
    let (nw, ne, sw, se) = (
        Point::new(a.x.min(b.x), a.y.min(b.y)),
        Point::new(a.x.max(b.x), a.y.min(b.y)),
        Point::new(a.x.min(b.x), a.y.max(b.y)),
        Point::new(a.x.max(b.x), a.y.max(b.y)),
    );

    // nw -> ne
    let mut current = nw;
    while current.x < ne.x {
        let bb = bounding_boxes.iter().find(|bb| bb_intersect(bb, current));

        if let Some(bb) = bb {
            current.x = bb.1.x + 1;
        } else {
            return false;
        }
    }

    // nw -> sw
    let mut current = nw;
    while current.y < sw.y {
        let bb = bounding_boxes.iter().find(|bb| bb_intersect(bb, current));

        if let Some(bb) = bb {
            current.y = bb.1.y + 1;
        } else {
            return false;
        }
    }

    // ne -> se
    let mut current = ne;
    while current.y < se.y {
        let bb = bounding_boxes.iter().find(|bb| bb_intersect(bb, current));

        if let Some(bb) = bb {
            current.y = bb.1.y + 1;
        } else {
            return false;
        }
    }

    // sw -> se
    let mut current = sw;
    while current.x < se.x {
        let bb = bounding_boxes.iter().find(|bb| bb_intersect(bb, current));

        if let Some(bb) = bb {
            current.x = bb.1.x + 1;
        } else {
            return false;
        }
    }

    true
}

pub fn part_two(input: &str) -> Option<u64> {
    // I couldn't figure out a clever solution for this one,
    // but this does produce the correct output so there's that

    // Split area into smaller bounding rectangles and require
    // the chosen area to intersect with it fully
    let mut points = parse_input(input);
    points.push(points[0]);

    let mut columns = vec![];
    let mut ceilings = vec![];
    let mut floors = vec![];
    let mut last_direction = None;
    let mut lefts = 0;
    let mut rights = 0;

    // Draw out line and figure out "ceilings" and "floors"
    // Also count lefts and rights to determine which side
    // is inside
    for line in points.windows(2) {
        let [a, b] = line else { unreachable!() };

        let direction = match (a, b) {
            (a, b) if a.x < b.x => Direction::Right,
            (a, b) if a.x > b.x => Direction::Left,
            (a, b) if a.y < b.y => Direction::Down,
            (a, b) if a.y > b.y => Direction::Up,
            _ => unreachable!(),
        };

        match (last_direction, direction) {
            (Some(Direction::Right), Direction::Up)
            | (Some(Direction::Down), Direction::Right)
            | (Some(Direction::Left), Direction::Down)
            | (Some(Direction::Up), Direction::Left) => lefts += 1,
            (None, _) => (),
            _ => rights += 1,
        };

        if a.x != b.x {
            let start = a.x.min(b.x);
            let end = a.x.max(b.x);
            let y = a.y;
            let range = (start..=end, y);

            if a.x < b.x {
                ceilings.push(range);
            } else {
                floors.push(range);
            }
        } else {
            columns.push((RangeInclusive::new(a.y.min(b.y) + 1, a.y.max(b.y) - 1), a.x));
        }

        last_direction = Some(direction);
    }

    if lefts > rights {
        (floors, ceilings) = (ceilings, floors);
    }

    // Split puzzle area into bounding boxes
    // They might overlap but it shouldn't matter
    let mut bounding_boxes = vec![];

    ceilings.sort_unstable_by_key(|(_, y)| -*y);
    floors.sort_unstable_by_key(|(_, y)| *y);

    while let Some((ceiling, y_ceiling)) = ceilings.pop() {
        let (idx_floor, (floor, y_floor)) = floors
            .iter()
            .enumerate()
            .find(|(_, (floor, _))| {
                let intersection = range_intersect(&ceiling, floor);

                if let Some(range) = intersection
                    && range.end() - range.start() > 1
                {
                    true
                } else {
                    false
                }
            })
            .unwrap();

        let intersection = range_intersect(&ceiling, floor).unwrap();

        let bb = (
            Point::new(*intersection.start(), y_ceiling),
            Point::new(*intersection.end(), *y_floor),
        );

        bounding_boxes.push(bb);

        // Splinters are left behind from intersections. Add these
        // to ceiling and floor to be processed later
        let mut splinters_ceiling = vec![];
        let mut splinters_floor = vec![];

        if ceiling.start() < intersection.start() {
            splinters_ceiling.push((*ceiling.start()..=*intersection.start(), y_ceiling));
        }

        if intersection.end() < ceiling.end() {
            splinters_ceiling.push((*intersection.end()..=*ceiling.end(), y_ceiling));
        }

        if floor.start() < intersection.start() {
            splinters_floor.push((*floor.start()..=*intersection.start(), *y_floor));
        }

        if intersection.end() < floor.end() {
            splinters_floor.push((*intersection.end()..=*floor.end(), *y_floor));
        }

        floors.swap_remove(idx_floor);
        floors = [floors, splinters_floor].concat();
        ceilings = [ceilings, splinters_ceiling].concat();

        ceilings.sort_unstable_by_key(|(_, y)| -*y);
        floors.sort_unstable_by_key(|(_, y)| *y);
    }

    // Same as part 1, but validate that the resulting
    // area is completely within bounding boxes
    let mut largest = 0;

    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let a = points[i];
            let b = points[j];
            let size = (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1);
            if size > largest && validate(&bounding_boxes, a, b) {
                largest = size;
            }
        }
    }

    Some(largest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
