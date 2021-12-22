use crate::{DayResult, DaySolver};
use std::time::SystemTime;

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let data = include_str!("data/day22.dat");
        let start = SystemTime::now();

        let init_cubes = cuboids_from(data, 50);
        let init_lit = combine_cuboids(&init_cubes);
        let all_cubes = cuboids_from(data, 0);
        let all_lit = combine_cuboids(&all_cubes);

        let timed = SystemTime::now().duration_since(start).unwrap();
        let description = format!(
            "On initialisation {} cubes are lit. Extending to \
        the full array {} cubes are lit.",
            init_lit, all_lit
        );

        DayResult {
            description,
            part1: format!("{}", init_lit),
            part2: format!("{}", all_lit),
            timing_us: timed.as_micros(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn origin() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

#[derive(Debug, Clone)]
struct Cuboid {
    from: Point,
    to: Point,
    status: bool,
    contribution: i64,
}

impl Cuboid {
    // Note the '+1' as cuboid boundaries are inclusive.
    fn enclosed_cubes(&self) -> i64 {
        (1 + self.to.x - self.from.x)
            * (1 + self.to.y - self.from.y)
            * (1 + self.to.z - self.from.z)
    }
}

// Find the intersection portion of two lines.
fn intersect_lines(from1: i64, to1: i64, from2: i64, to2: i64) -> Option<(i64, i64)> {
    // Simplify by assuming from1 is the minimum value.
    if from1 > from2 {
        return intersect_lines(from2, to2, from1, to1);
    }

    if to1 < from2 {
        // Line 1 finishes before line 2. No intersection.
        return None;
    }

    // The lines do intersect.
    Some((from2, std::cmp::min(to1, to2)))
}

fn combine_cuboids(cuboids: &[Cuboid]) -> i128 {
    let mut combined = vec![];
    for cuboid in cuboids {
        // We need to try to combine with all existing cuboids.
        let mut to_add = vec![];
        for existing in &combined {
            if let Some(intersection) = intersect_cuboids(cuboid, existing) {
                to_add.push(intersection);
            }
        }

        // We don't add 'off' cuboids. Their contributions have now been accounted for.
        if cuboid.status {
            to_add.push(cuboid.clone());
        }
        combined.append(&mut to_add);
        //println!("{} lit.", volume_of(&combined));
    }

    volume_of(&combined)
}

fn volume_of(combined: &[Cuboid]) -> i128 {
    // Find the total volume.
    let mut total_volume = 0;
    for c in combined {
        total_volume += (c.contribution * c.enclosed_cubes()) as i128
    }

    total_volume
}

// When two cuboids intersect, we can represent the union as a number of disjoint cuboids.
fn intersect_cuboids(c1: &Cuboid, c2: &Cuboid) -> Option<Cuboid> {
    let intersect_x = intersect_lines(c1.from.x, c1.to.x, c2.from.x, c2.to.x)?;
    let intersect_y = intersect_lines(c1.from.y, c1.to.y, c2.from.y, c2.to.y)?;
    let intersect_z = intersect_lines(c1.from.z, c1.to.z, c2.from.z, c2.to.z)?;

    // The contribution conditions:
    // If ON, ON the contribution is -1 (i.e. avoid double counting.)
    // If OFF, ON the contribution is -1 (i.e. turning off.)
    // If ON, OFF the contribution is 1. (We're adding to a gap.)
    // If OFF, OFF the contribution is +1 (i.e. We're turning 'off' a correction - this gets ignored.)

    let contribution = match (c1.contribution, c2.contribution) {
        (1, 1) => -1,
        (-1, 1) => -1,
        (1, -1) => 1,
        (-1, -1) => 1,
        _ => panic!(
            "unexpected contributions ({}, {})",
            c1.contribution, c2.contribution
        ),
    };

    if contribution == 0 {
        None
    } else {
        Some(Cuboid {
            from: Point {
                x: intersect_x.0,
                y: intersect_y.0,
                z: intersect_z.0,
            },
            to: Point {
                x: intersect_x.1,
                y: intersect_y.1,
                z: intersect_z.1,
            },
            status: false,
            contribution,
        })
    }
}

fn cuboids_from(data: &str, limit: i64) -> Vec<Cuboid> {
    let mut cubes = vec![];
    for l in data.lines() {
        let l_wspace = l.split_whitespace().collect::<Vec<&str>>();
        let status = l_wspace[0] == "on";
        let mut from = Point::origin();
        let mut to = Point::origin();
        let coords = l_wspace[1].split(',');
        for c in coords {
            let coord_bounds = c.split('=').collect::<Vec<&str>>();
            let mut min_max = coord_bounds[1]
                .split("..")
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            min_max.sort_unstable();
            match coord_bounds[0] {
                "x" => {
                    if limit > 0 {
                        from.x = std::cmp::max(-limit, min_max[0]);
                        to.x = std::cmp::min(limit, min_max[1]);
                    } else {
                        from.x = min_max[0];
                        to.x = min_max[1];
                    }
                }
                "y" => {
                    if limit > 0 {
                        from.y = std::cmp::max(-limit, min_max[0]);
                        to.y = std::cmp::min(limit, min_max[1]);
                    } else {
                        from.y = min_max[0];
                        to.y = min_max[1];
                    }
                }
                "z" => {
                    if limit > 0 {
                        from.z = std::cmp::max(-limit, min_max[0]);
                        to.z = std::cmp::min(limit, min_max[1]);
                    } else {
                        from.z = min_max[0];
                        to.z = min_max[1];
                    }
                }
                _ => panic!("Unexpected coordinate symbol {}", coord_bounds[0]),
            }
        }

        let out_of_range = limit > 0
            && (from.x > limit
                || from.y > limit
                || from.z > limit
                || to.x < -limit
                || to.y < -limit
                || to.z < -limit);

        let contribution = if status { 1 } else { -1 };
        if !out_of_range {
            cubes.push(Cuboid {
                from,
                to,
                status,
                contribution,
            });
        }
    }
    cubes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("data/test_day22b.dat");

        let init_cubes = cuboids_from(data, 50);
        let init_lit = combine_cuboids(&init_cubes);
        assert_eq!(init_lit, 474140);
        let all_cubes = cuboids_from(data, 0);
        let all_lit = combine_cuboids(&all_cubes);
        assert_eq!(all_lit, 2758514936282235);
    }
}
