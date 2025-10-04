// Ranges are exclusive i.e. [start, end[
#[derive(Debug, Clone, Copy)]
pub struct Range<T> {
    pub start: T,
    pub end: T,
}

impl<T> Range<T> {
    pub fn new(start: T, end: T) -> Self {
        Range { start, end }
    }
}

impl<T> Range<T>
where
    T: Ord + Copy,
{
    pub fn intersect(&self, other: &Range<T>) -> Option<(T, T)> {
        if other.end <= self.start || self.end <= other.start {
            return None;
        }

        let start = std::cmp::max(self.start, other.start);
        let end = std::cmp::min(self.end, other.end);

        Some((start, end))
    }

    pub fn union(&self, other: &Range<T>) -> Vec<Range<T>> {
        if self.overlap(other) {
            return vec![Range {
                start: std::cmp::min(self.start, other.start),
                end: std::cmp::max(self.end, other.end),
            }];
        }

        vec![*self, *other]
    }

    pub fn overlap(&self, other: &Range<T>) -> bool {
        !(other.end <= self.start || self.end <= other.start)
    }
}

pub fn union_vecs<T: Ord + Copy>(a: &[Range<T>], b: &[Range<T>]) -> Vec<Range<T>> {
    let mut ranges = a.to_vec();
    ranges.extend_from_slice(b);

    union_vec(&ranges)
}

pub fn union_vec<T: Ord + Copy>(ranges: &[Range<T>]) -> Vec<Range<T>> {
    if ranges.is_empty() {
        return vec![];
    }

    // Sort ranges by start
    let mut sorted_ranges = ranges.to_vec();
    sorted_ranges.sort_by_key(|r| r.start);

    let mut result = vec![];
    let mut current = sorted_ranges[0];

    for range in sorted_ranges.iter().skip(1) {
        if current.end >= range.start {
            // Merge overlapping or adjacent ranges
            current.end = std::cmp::max(current.end, range.end);
        } else {
            result.push(current);
            current = *range;
        }
    }

    result.push(current);

    result
}
