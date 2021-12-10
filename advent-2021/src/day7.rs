use std::collections::HashMap;
use std::time::SystemTime;
use crate::{DayResult, DaySolver};

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let data = include_str!("data/day7.dat");
        let crabs: Vec<i32> = data.split(",").map(|c| c.parse::<i32>().unwrap()).collect();
        let start = SystemTime::now();
        let (target, fuel) = least_fuel(&crabs);
        let (target_crab, fuel_crab) = least_fuel_crabwise(&crabs);
        let timed = SystemTime::now().duration_since(start).unwrap();
        let desc1 = format!("The crabs can reach {} with {} fuel.", target, fuel);
        let desc2 = format!("In crab mode, the crabs can reach {} with {} fuel.",
                 target_crab, fuel_crab);
        DayResult{
            description: format!("{}\n{}", desc1, desc2),
            part1: format!("{}", fuel),
            part2: format!("{}", fuel_crab),
            timing_us: timed.as_micros(),
        }
    }
}

pub fn least_fuel(crabs: &[i32]) -> (i32, i32) {
    let mut sorted_crabs: Vec<i32> = vec![0; crabs.len()];

    // Initial scan of the crabs.
    let mut fuel_to_zero = 0;
    let mut crabs_at: HashMap<i32, i32> = HashMap::new();
    for (i, crab) in crabs.iter().enumerate() {
        fuel_to_zero += crab;
        let crabs_at_i = crabs_at.entry(*crab).or_insert(0);
        *crabs_at_i += 1;
        sorted_crabs[i] = *crab;
    }

    sorted_crabs.sort();

    // Now find the optimum.
    let mut target = sorted_crabs[0];
    let mut below = *crabs_at.get(&target).unwrap_or(&0);
    let mut above = (crabs.len() as i32) - below;
    let mut fuel = fuel_to_zero;

    // While below is smaller, we always improve by increasing.
    while below < above && target < 500 {
        target += 1;
        // Everything below costs, everything above reduces cost.
        fuel = fuel + below - above;
        let crossing = crabs_at.get(&target).unwrap_or(&0);
        above -= crossing;
        below += crossing;
    }

    (target, fuel)
}

pub fn least_fuel_crabwise(crabs: &[i32]) -> (i32, i32) {
    let mut sorted_crabs: Vec<i32> = vec![0; crabs.len()];

    // Initial scan of the crabs.
    let mut fuel_to_zero = 0;
    // The initial cost of moving all crabs one step towards zero.
    let mut init_above_cost = 0;
    let mut crabs_at: HashMap<i32, i32> = HashMap::new();
    for (i, crab) in crabs.iter().enumerate() {
        // The fuel to zero is now 1 + 2 + ... + crab = (1 + crab)crab / 2
        fuel_to_zero += ((1 + crab) * crab) / 2;
        init_above_cost += crab;
        let crabs_at_i = crabs_at.entry(*crab).or_insert(0);
        *crabs_at_i += 1;
        sorted_crabs[i] = *crab;
    }

    sorted_crabs.sort();

    // Now find the optimum.
    let mut target = 0;
    let mut below = *crabs_at.get(&target).unwrap_or(&0);
    let mut above = (crabs.len() as i32) - below;
    let mut above_cost = init_above_cost; // We need to ignore the crabs at zero.
    let mut below_cost = below; // The initial cost of moving from zero, is all the crabs at zero.
    let mut fuel = fuel_to_zero;

    // While below is smaller, we always improve by increasing.
    while below_cost < above_cost {
        target += 1;
        // Everything below costs, everything above reduces cost.
        fuel = fuel + below_cost - above_cost;
        let crossing = crabs_at.get(&target).unwrap_or(&0);
        // The above cost reduces by one for everything that was *previously* above.
        above_cost -= above;
        // The number above and below are adjusted.
        above -= crossing;
        below += crossing;
        // The below cost increase by one for everything that is *now* below.
        below_cost += below;

    }

    (target, fuel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("data/test_day7.dat");
        let crabs: Vec<i32> = data.split(",").map(|c| c.parse::<i32>().unwrap()).collect();
        let (target, fuel) = least_fuel(&crabs);
        assert_eq!(target, 2);
        assert_eq!(fuel, 37);
        let (target_crab, fuel_crab) = least_fuel_crabwise(&crabs);
        // Sanity check fuel calculation
        assert_eq!(cost_to(&crabs, target_crab), fuel_crab);
        assert_eq!(target_crab, 5);
        assert_eq!(fuel_crab, 168);
    }

    fn cost_to(crabs: &[i32], target: i32) -> i32 {
        let mut fuel = 0;
        for crab in crabs {
            let distance = (target - crab).abs();
            fuel += (distance * (distance + 1)) / 2;
        }
        fuel
    }
}
