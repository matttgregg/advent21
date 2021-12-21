use std::cmp::max;
use std::time::SystemTime;
use std::collections::HashMap;
use crate::{DayResult, DaySolver};

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let data = include_str!("data/day9.dat");
        let start = SystemTime::now();
        let (danger, lowest, map) = find_lowest(data);
        let (s1, s2, s3) = biggest_three(&lowest, &map);
        let timed = SystemTime::now().duration_since(start).unwrap();
        let desc1 = format!("Danger in the area {} .", danger);
        let desc2 = format!("Found biggest sinks {} * {} * {} = {}", s1, s2, s3, (s1 * s2 * s3));

        DayResult{
            description: format!("{}\n{}", desc1, desc2),
            part1: format!("{}", danger),
            part2: format!("{}", s1 * s2 * s3),
            timing_us: timed.as_micros(),
        }
    }
}

fn biggest_three(sinks: &[Point], map: &[Vec<u8>]) -> (u64, u64, u64) {
    let mut sizes = all_sizes(sinks, map);
    sizes.sort_unstable();
    (sizes[sizes.len() - 1], sizes[sizes.len() - 2], sizes[sizes.len() -3])
}

fn all_sizes(sinks: &[Point], map: &[Vec<u8>]) -> Vec<u64> {
    let mut sizes =vec![];
    for sink in sinks {
        sizes.push(size(sink, map));
    }

    sizes
}

fn size(from: &Point, map: &[Vec<u8>]) -> u64 {
    size_iter(from, map, &mut HashMap::new())
}

fn size_iter(from: &Point, map: &[Vec<u8>], seen: &mut HashMap<Point, bool>) -> u64 {
    seen.insert(from.clone(), true);
    // Casees for skipping
    // : On the edge
    // : Seen before
    // : Neighbour is a 9
    // If these are all false, recurse to the neighbour.
    let n0 = if from.i == 0 || seen.contains_key(&from.shift(-1, 0)) || map[from.i - 1][from.j] == 9 {
        0 } else {
        size_iter(&from.shift(-1, 0), map, seen)
    };
    let n1 = if from.i + 1 == map.len() || seen.contains_key(&from.shift(1, 0)) || map[from.i + 1][from.j] == 9 {
        0 } else {
        size_iter(&from.shift(1, 0), map, seen)
    };
    let n2 = if from.j == 0 || seen.contains_key(&from.shift(0, -1)) || map[from.i][from.j - 1] == 9 {
        0 } else {
        size_iter(&from.shift(0, -1), map, seen)
    };
    let n3 = if from.j + 1 == map[from.i].len() || seen.contains_key(&from.shift(0, 1)) || map[from.i][from.j + 1] == 9 {
        0 } else {
        size_iter(&from.shift(0, 1), map, seen)
    };

    1 + n0 + n1 + n2 + n3
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    i: usize,
    j: usize
}

impl Point {
   fn new(i: usize, j: usize) -> Self {
       Self{i, j}
   }

    fn shift(&self, i: i64, j: i64) -> Self {
        Self{
            i: max(self.i as i64 + i, 0) as usize,
            j: max(self.j as i64 + j, 0) as usize,
        }
    }
}

fn find_lowest(data: &str) -> (u64, Vec<Point>, Vec<Vec<u8>>) {
    let lines = data.lines();
    let mut heights = vec![];
    for line in lines {
        let mut row = vec![];
        for c in line.chars() {
            let val = c.to_string().parse::<u8>().unwrap();
            row.push(val);
        }
        heights.push(row);
    }

    // Now find the low points.
    let mut danger = 0;
    let mut lowest = vec![];
    for (i, row) in heights.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if (i == 0 || heights[i - 1][j] > *val) &&
                (i + 1 == heights.len() || heights[i + 1][j] > *val) &&
                (j == 0 || heights[i][j - 1] > *val) &&
                (j + 1 == heights[i].len() || heights[i][j + 1] > *val) {
                danger += (val + 1) as u64;
                lowest.push(Point::new(i, j));
            }
        }
    }

    (danger, lowest, heights)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("data/test_day9.dat");
        let (danger, lowest, map) = find_lowest(data);
        let (s1, s2, s3) = biggest_three(&lowest, &map);
        assert_eq!(danger, 15);
        assert_eq!(s1, 14);
        assert_eq!(s2, 9);
        assert_eq!(s3, 9);
    }
}