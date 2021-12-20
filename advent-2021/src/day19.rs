use crate::{DayResult, DaySolver};
use std::collections::HashMap;
use std::time::SystemTime;

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let data = include_str!("data/day19.dat");
        let start = SystemTime::now();

        let mut scanners = load_scanners(data);
        let (beacons, separation) = align_all(&mut scanners);

        let timed = SystemTime::now().duration_since(start).unwrap();
        let description = format!(
            "After alignment, deduced {} beacons. Largest scanner separation is {} .",
            beacons, separation
        );

        DayResult {
            description,
            part1: format!("{}", beacons),
            part2: format!("{}", separation),
            timing_us: timed.as_micros(),
        }
    }
}

fn align_all(scanners: &mut Vec<Scanner>) -> (usize, i64) {
    let scanner_count = scanners.len();
    let mut worklist = vec![0usize];
    let mut aligned = HashMap::new();
    let mut scanner_origins: Vec<Point> = vec![[0, 0, 0]];
    aligned.insert(0, true);

    while !worklist.is_empty() {
        let working = worklist.pop().unwrap();
        for i in 0..scanner_count {
            if aligned.contains_key(&i) {
                continue;
            }

            if let Some(transform) = align(&scanners[working], &scanners[i]) {
                scanners[i].offset = transform.offset;
                scanners[i].flips = transform.flip;
                scanners[i].permutation = transform.permutation;
                scanner_origins.push(transformed(
                    &[0, 0, 0],
                    transform.permutation,
                    transform.offset,
                    transform.flip,
                ));
                worklist.push(i);
                aligned.insert(i, true);
            }
        }
    }

    // Now try to find all the points.
    let mut all_points = HashMap::new();
    for scanner in scanners {
        for i in 0..scanner.points.len() {
            all_points.insert(scanner.transformed_point(i), true);
        }
    }

    let mut max_distance = 0;
    for i in 0..scanner_origins.len() {
        for j in 0..scanner_origins.len() {
            let separation = manhattan(&scanner_origins[i], &scanner_origins[j]);
            if separation > max_distance {
                max_distance = separation;
            }
        }
    }

    (all_points.len(), max_distance)
}

fn manhattan(a: &Point, b: &Point) -> i64 {
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs()
}

type Point = [i64; 3];

// A scanner is a collection of points
#[derive(Debug)]
struct Scanner {
    points: Vec<Point>,
    permutation: [usize; 3],
    flips: [bool; 3],
    offset: Point,
}

impl Scanner {
    fn blank() -> Self {
        Self {
            points: vec![],
            permutation: [0, 1, 2],
            flips: [false, false, false],
            offset: [0, 0, 0],
        }
    }

    fn transformed_point(&self, i: usize) -> Point {
        transformed(&self.points[i], self.permutation, self.offset, self.flips)
    }
}

#[derive(Debug)]
struct Transform {
    permutation: [usize; 3],
    offset: [i64; 3],
    flip: [bool; 3],
}

fn transformed(p: &Point, permutation: [usize; 3], offset: [i64; 3], flip: [bool; 3]) -> Point {
    let mut point = [0; 3];

    for i in 0..3 {
        point[i] = p[permutation[i]];
        point[i] *= if flip[i] { -1 } else { 1 };
        point[i] += offset[i];
    }

    point
}

// Try to align scanner b with scanner a.
fn align(a: &Scanner, b: &Scanner) -> Option<Transform> {
    // Hash all the a x-values for checking for alignment.
    let mut x_lookup = HashMap::new();
    for a_p in &a.points {
        let a_point = transformed(&a_p, a.permutation, a.offset, a.flips);
        x_lookup.insert(a_point[0], true);
    }

    for permutation in [[0usize, 1, 2], [1, 2, 0], [2, 0, 1]].iter() {
        for flip in [[false, false, false], [true, false, false]].iter() {
            //  Try an align on x coords first.
            for (i_b, b_p) in b.points.iter().enumerate() {
                if b.points.len() - i_b < 12 {
                    // If there was alignment, we'd have found by now.
                    break;
                }
                for (i_a, a_p) in a.points.iter().enumerate() {
                    if a.points.len() - i_a < 12 {
                        // If there was alignment, we'd have found by now.
                        break;
                    }
                    // If we align these two points, how many other points align?
                    let a_point = transformed(a_p, a.permutation, a.offset, a.flips);
                    let b_point = transformed(b_p, *permutation, [0, 0, 0], *flip);

                    let offset = [a_point[0] - b_point[0], 0, 0];

                    assert_eq!(
                        a_point[0],
                        transformed(&b_p, *permutation, offset, *flip)[0]
                    );

                    // How many other points align when we do this?
                    let mut aligned = 0;
                    for (p_i, try_p) in b.points.iter().enumerate() {
                        let try_point_x = transformed(try_p, *permutation, offset, *flip)[0];
                        if x_lookup.contains_key(&try_point_x) {
                            aligned += 1;
                        }

                        if aligned + b.points.len() - p_i < 12 {
                            // No point in checking the rest, we can't reach 12.
                            break;
                        }
                    }

                    if aligned >= 12 {
                        let full_transform =
                            full_align_from_x(a, b, i_a, i_b, *permutation, offset, *flip);
                        if full_transform.is_some() {
                            return full_transform;
                        }
                    }
                }
            }
        }
    }
    None
}

// Given an alignment on the x-axis, find an alignment of the remaining points.
fn full_align_from_x(
    a: &Scanner,
    b: &Scanner,
    i_a: usize,
    i_b: usize,
    permutation: [usize; 3],
    offset: [i64; 3],
    flip: [bool; 3],
) -> Option<Transform> {
    // There are only two further permutations.
    let permutations = [
        [permutation[0], permutation[1], permutation[2]],
        [permutation[0], permutation[2], permutation[1]],
    ];
    // There are four potential flips.
    let flips = [
        [flip[0], false, false],
        [flip[0], true, false],
        [flip[0], false, true],
        [flip[0], true, true],
    ];

    let point_a = a.transformed_point(i_a);

    let mut point_map = HashMap::new();
    for i in 0..a.points.len() {
        point_map.insert(a.transformed_point(i), true);
    }

    // Now work through transformations to try an match remaining.
    for try_permutation in permutations.iter() {
        for try_flip in flips.iter() {
            let point_b = transformed(&b.points[i_b], *try_permutation, offset, *try_flip);

            let try_offset = [offset[0], point_a[1] - point_b[1], point_a[2] - point_b[2]];

            assert_eq!(
                point_a,
                transformed(&b.points[i_b], *try_permutation, try_offset, *try_flip)
            );

            let mut matched = 0;
            for (tries, p_b) in b.points.iter().enumerate() {
                let try_point = transformed(p_b, *try_permutation, try_offset, *try_flip);
                if point_map.contains_key(&try_point) {
                    matched += 1;
                }
                if matched + b.points.len() - tries < 12 {
                    // Not enough points remaining to match.
                    break;
                }
            }

            if matched >= 12 {
                return Some(Transform {
                    permutation: *try_permutation,
                    offset: try_offset,
                    flip: *try_flip,
                });
            }
        }
    }

    None
}

// Load all scanners from test input.
fn load_scanners(data: &str) -> Vec<Scanner> {
    let mut scanners = vec![];
    let mut current_scanner = Scanner::blank();
    for l in data.lines() {
        if l.is_empty() {
            continue;
        }

        let on_whitespace = l.split_whitespace().collect::<Vec<&str>>();
        if on_whitespace.len() >= 3 {
            if current_scanner.points.len() > 0 {
                scanners.push(current_scanner);
            }
            current_scanner = Scanner::blank();
        } else {
            let p = l
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            assert_eq!(p.len(), 3);
            let mut point = [0; 3];
            for i in 0..3 {
                point[i] = p[i];
            }

            current_scanner.points.push(point);
        }
    }

    if current_scanner.points.len() > 0 {
        scanners.push(current_scanner);
    }

    scanners
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("data/test_day19.dat");
        let mut scanners = load_scanners(data);
        let (beacons, separation) = align_all(&mut scanners);
        assert_eq!(beacons, 79);
        assert_eq!(separation, 3621);
    }
}
