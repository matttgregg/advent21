use std::cmp::{min, max};
use std::collections::HashMap;
use std::str::FromStr;

// Solve the day.
pub fn solve() {
    let data = include_str!("./data/day5.dat");
    let connections: Vec<Connection> = data.lines().map(|s| s.parse::<Connection>().unwrap()).collect();
    let danger_points = crossings(&connections, false);
    println!("[5] There are {} danger points.", danger_points);
    let diagonal_danger_points = crossings(&connections, true);
    println!("[5] There are {} diagonal danger points.", diagonal_danger_points);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("./data/test_day5.dat");
        let connections: Vec<Connection> = data.lines().map(|s| s.parse::<Connection>().unwrap()).collect();
        let danger_points = crossings(&connections, false);
        assert_eq!(danger_points, 5);
        let diagonal_danger_points = crossings(&connections, true);
        assert_eq!(diagonal_danger_points, 12);

    }
}

fn crossings(connections: &Vec<Connection>, with_diagonals: bool) -> i32 {
    let mut covered: HashMap<Point, i32> = HashMap::new();
    // We walk each connection filling its points
    for conn in connections {
        if conn.from.x == conn.to.x {
            let x = conn.from.x;
            let fromy = min(conn.from.y, conn.to.y);
            let toy = max(conn.from.y, conn.to.y);
            for y in fromy..=toy {
                let pt = Point { x, y };
                let pt_count = covered.entry(pt).or_insert(0);
                *pt_count += 1;
            }
        } else if conn.from.y == conn.to.y {
            let y = conn.from.y;
            let fromx = min(conn.from.x, conn.to.x);
            let tox = max(conn.from.x, conn.to.x);
            for x in fromx..=tox {
                let pt = Point { x, y };
                let pt_count = covered.entry(pt).or_insert(0);
                *pt_count += 1;
            }
        } else if with_diagonals {
            // We want to consider diagonals.
            let incr_x =  if conn.to.x > conn.from.x { 1 } else { -1 };
            let incr_y =  if conn.to.y > conn.from.y { 1 } else { -1 };

            let mut x = conn.from.x;
            let mut y = conn.from.y;

            // We want to include the end, hence the off by one check.
            while (x - incr_x) != conn.to.x {
                let pt = Point { x, y };
                let pt_count = covered.entry(pt).or_insert(0);
                *pt_count += 1;
                x += incr_x;
                y += incr_y;
            }
        }
    }

    // Now just check which points are crossed multiple times.
    let mut danger_points = 0;
    for (_, v) in covered.iter() {
        if *v > 1 {
            danger_points += 1;
        }
    }
    danger_points
}

// point is a 2 dimensional point.
#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // We expect a pair of int values
        let parts: Vec<&str> = s.split(",").collect();
        if parts.len() != 2 {
            Err(format!("Could not parse {} as a point", s))
        } else {
            Ok(Self {
                x: parts[0].parse::<i32>().unwrap(),
                y: parts[1].parse::<i32>().unwrap(),
            })
        }
    }
}

// A line between two points.
#[derive(Debug)]
struct Connection {
    from: Point,
    to: Point,
}

impl FromStr for Connection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // We expect a pair of int values
        let parts: Vec<&str> = s.split(" -> ").collect();
        if parts.len() != 2 {
            Err(format!("Could not parse {} as a connection", s))
        } else {
            let from = parts[0].parse::<Point>()?;
            let to = parts[1].parse::<Point>()?;
            Ok(Self {
                from,
                to,
            })
        }
    }
}