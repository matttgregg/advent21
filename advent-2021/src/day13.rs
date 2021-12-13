use crate::{DaySolver, DayResult};
use std::collections::HashMap;
use std::time::SystemTime;

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let data = include_str!("data/day13.dat");
        let start = SystemTime::now();
        let transparency = Transparency::new(data);
        let single_folded = transparency.fold(1);
        let plot = plot(transparency.fold(0));
        let timed = SystemTime::now().duration_since(start).unwrap();
        let description = format!(
            "After one fold, there are {} points. After complete folding: \n{}",
            single_folded.len(),
            plot);

        DayResult {
            description,
            part1: format!("{}", single_folded.len()),
            part2: format!("{}", plot),
            timing_us: timed.as_micros(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point(u64, u64);

#[derive(Debug)]
enum Fold {
    FoldX(u64),
    FoldY(u64),
}

#[derive(Debug)]
struct Transparency {
    points: Vec<Point>,
    folds: Vec<Fold>,
}

fn plot(points: HashMap<Point, bool>) -> String {
    // Find boundaries of the system.
    let mut max_x = 0;
    let mut max_y = 0;
    for p in points.keys() {
        max_x = std::cmp::max(max_x, p.0);
        max_y = std::cmp::max(max_y, p.1);
    }

    // Now print!
    let mut output = vec![];
    for r in 0..=max_y {
        let mut line = vec![];
        for c in 0..=max_x {
            if *points.get(&Point(c, r)).unwrap_or(&false) {
                line.push("█");
            } else {
                line.push(" ");
            }
        }
        output.push(line.join(""));
    }

    output.join("\n")
}

impl Transparency {
    // Fold returns a map of marked points.
    fn fold(&self, folds: usize) -> HashMap<Point, bool> {
        let mut point_map = HashMap::new();

        let folds = if folds > 0 {
            std::cmp::min(folds, self.folds.len())
        } else {
            self.folds.len()
        };

        for p in &self.points {
            // Apply folds as necessary
            let mut x = p.0;
            let mut y = p.1;
            for fold_index in 0..folds {
                match self.folds[fold_index] {
                    Fold::FoldX(f) => {
                        if x > f {
                            x = 2 * f - x;
                        }
                    },
                    Fold::FoldY(f) => {
                        if y > f {
                            y = 2 * f - y;
                        }
                    },
                }
            }
            point_map.insert(Point(x, y), true);
        }
        point_map
    }

    fn new(data: &str) -> Self {
        let mut points = vec![];
        let mut folds = vec![];
        for line in data.lines() {
            if line.starts_with("fold along y=") {
                let val = line.split("=").collect::<Vec<&str>>()[1].parse::<u64>().unwrap();
                folds.push(Fold::FoldY(val))
            } else if line.starts_with("fold along x=") {
                let val = line.split("=").collect::<Vec<&str>>()[1].parse::<u64>().unwrap();
                folds.push(Fold::FoldX(val))
            } else if line.contains(",") {
                let vals = line.split(",").collect::<Vec<&str>>();
                points.push(Point(vals[0].parse::<u64>().unwrap(),
                                  vals[1].parse::<u64>().unwrap()));
            }
        }

        Self {
            points,
            folds,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("data/test_day13.dat");
        let transparency = Transparency::new(data);
        let single_folded = transparency.fold(1);
        let all_folded = transparency.fold(0);
        assert_eq!(single_folded.len(), 17);
        assert_eq!(all_folded.len(), 16);
    }
}

