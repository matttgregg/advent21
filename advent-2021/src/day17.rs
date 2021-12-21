use crate::{DayResult, DaySolver};
use std::collections::HashMap;
use std::time::SystemTime;
use std::collections::hash_map::Entry;

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let start = SystemTime::now();

        let (max_y, valid) = max_height(143, 177, -106, -71);

        let timed = SystemTime::now().duration_since(start).unwrap();
        let description = format!(
            "Best trickshot height is {}. In total {} possible trajectories.",
            max_y, valid
        );

        DayResult {
            description,
            part1: format!("{}", max_y),
            part2: format!("{}", valid),
            timing_us: timed.as_micros(),
        }
    }
}

fn max_height(x_min: i64, x_max: i64, y_min: i64, y_max: i64) -> (i64, usize) {
    let (max_dy, valid) = max_dy(x_min, x_max, y_min, y_max);
    (limit_for(max_dy as f64) as i64, valid)
}

fn limit_for(dy: f64) -> i64 {
    // The 'best' t is found by finding the stable point.
    ((dy * dy) - 0.5 * dy * (dy - 1.0)).round() as i64
}

fn max_dy(x_min: i64, x_max: i64, y_min: i64, y_max: i64) -> (i64, usize) {
    let mut max_dy = 0;
    let mut all_valid: Vec<(i64, i64)> = vec![];

    for dy in y_min..=(-y_min) {
        let mut valid = try_target_dy(x_min, x_max, y_min, y_max, dy);

        if !valid.is_empty() {
            max_dy = dy;
            all_valid.append(&mut valid);
        }
    }

    (max_dy as i64, all_valid.len())
}

fn try_target_dy(x_min: i64, x_max: i64, y_min: i64, y_max: i64, dy: i64) -> Vec<(i64, i64)> {
    let mut seen: HashMap<i64, bool> = HashMap::new();
    let mut acceptable: Vec<(i64, i64)> = vec![];
    // What range of times hit the target?
    if let Some((time_min, time_max)) = time_for_target_range(y_min, y_max, dy) {
        let mut t = time_min;
        while t <= time_max {
            if let Some((dx_min, dx_max)) = dx_for_target_range(x_min, x_max, t) {
                for dx in dx_min..=dx_max {
                    // Check that we're not in a case where t has reached its limit.
                    if dx >= t {
                        if let Entry::Vacant(e) = seen.entry(dx as i64) {
                            e.insert(true);
                            acceptable.push((dx, dy));
                        }
                    }
                }
            }

            // Also check whether we can add limiting dx values. (i.e. where we slow to a stop).
            let dxs = limiting_dx(x_min, x_max, t);
            for dx in dxs {
                if let Entry::Vacant(e) = seen.entry(dx as i64) {
                    e.insert( true);
                    acceptable.push((dx, dy));
                }
            }
            t += 1;
        }
    }

    acceptable
}

fn dx_for_target(x: i64, target_time: i64) -> f64 {
    let target_time_float = target_time as f64;
    0.5 * (target_time_float - 1.0) + (x as f64 / target_time_float)
}

fn limiting_dx(x_min: i64, x_max: i64, allowed_time: i64) -> Vec<i64> {
    let mut valid_t = vec![];
    // Given min/max, what are the possible 'terminal' xs that will get us there?
    let t_min = (0.5 * ((1.0 + 8.0 * x_min as f64).sqrt() - 1.0)).ceil() as i64;
    let t_max = (0.5 * ((1.0 + 8.0 * x_max as f64).sqrt() - 1.0)).floor() as i64;

    for t in t_min..=t_max {
        if t <= allowed_time {
            valid_t.push(t);
        }
    }

    valid_t
}

fn dx_for_target_range(x_min: i64, x_max: i64, target_time: i64) -> Option<(i64, i64)> {
    let mut dx_min = dx_for_target(x_min, target_time).ceil() as i64;
    let dx_max = dx_for_target(x_max, target_time).floor() as i64;

    if dx_min < 1 {
        dx_min = 1;
    }

    if dx_max >= dx_min {
        Some((dx_min, dx_max))
    } else {
        None
    }
}

fn time_for_target_range(target_min: i64, target_max: i64, delta: i64) -> Option<(i64, i64)> {
    let time_min = time_for_target(target_max as f64, delta as f64).ceil() as i64;
    let time_max = time_for_target(target_min as f64, delta as f64).floor() as i64;

    if time_max >= time_min {
        Some((time_min, time_max))
    } else {
        None
    }
}

fn time_for_target(target: f64, delta: f64) -> f64 {
    // Note that we always select the larger time
    // Given we'll start with positive dy, and negative target.
    let discriminant_squared: f64 = (2.0 * delta + 1.0) * (2.0 * delta + 1.0) - 8.0 * target;
    let discriminant = discriminant_squared.sqrt();
    0.5 * (1.0 + 2.0 * delta + discriminant)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let (test_max_d, test_valid) = max_height(20, 30, -10, -5);
        assert_eq!(test_max_d, 45);
        assert_eq!(test_valid, 112);
    }
}
