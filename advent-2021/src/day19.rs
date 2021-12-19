use crate::{DayResult, DaySolver};
use std::collections::HashMap;
use std::time::SystemTime;

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let data = include_str!("data/test_day19.dat");
        let start = SystemTime::now();

        let scanners = load_scanners(data);
        println!("Read {} scanners: {:?}", scanners.len(), scanners);
        align(&scanners[0], &scanners[1]);

        let timed = SystemTime::now().duration_since(start).unwrap();
        let description = format!("");

        DayResult {
            description,
            part1: format!("{}", 0),
            part2: format!("{}", 0),
            timing_us: timed.as_micros(),
        }
    }
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
fn align(a: &Scanner, b: &Scanner) {
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
                for (i_a, a_p) in a.points.iter().enumerate() {
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
                    for try_p in &b.points {
                        let try_point_x = transformed(try_p, *permutation, offset, *flip)[0];
                        if x_lookup.contains_key(&try_point_x) {
                            aligned += 1;
                        }
                    }

                    if aligned >= 12 {
                        println!(
                            "Alignement found for Permutation {:?} Flips {:?} Offset {:?} ",
                            permutation, flip, offset
                        );
                        return full_align_from_x(a, b, i_a, i_b, *permutation, offset, *flip);
                    }
                }
            }
        }
    }
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
) {
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
            for p_b in &b.points {
                let try_point = transformed(p_b, *try_permutation, try_offset, *try_flip);
                if point_map.contains_key(&try_point) {
                    matched += 1;
                }
            }

            if matched >= 12 {
                println!(
                    "Found full alignment on {} points, Permutation {:?}, Offset {:?}, Flip {:?}",
                    matched, try_permutation, try_offset, try_flip
                );
                return;
            }
        }
    }
}

// Load all scanners from test input.
fn load_scanners(data: &str) -> Vec<Scanner> {
    let mut scanners = vec![];
    let mut current_scenner = Scanner::blank();
    for l in data.lines() {
        if l.is_empty() {
            continue;
        }

        let on_whitespace = l.split_whitespace().collect::<Vec<&str>>();
        if on_whitespace.len() >= 3 {
            if current_scenner.points.len() > 0 {
                scanners.push(current_scenner);
            }
            current_scenner = Scanner::blank();
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

            current_scenner.points.push(point);
        }
    }

    scanners
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {}
}
