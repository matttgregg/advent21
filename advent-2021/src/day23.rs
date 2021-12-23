use crate::day23::Contents::Empty;
use crate::{DayResult, DaySolver};
use std::collections::HashMap;
use std::fmt;
use std::time::SystemTime;

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let start = SystemTime::now();

        let mut burrows = Burrows::mine();
        println!("Starting at:\n{}", burrows);
        let (best_cost, best_moves) = best_cost(&burrows);
        println!(
            "Best solution costs {} in {} moves.",
            best_cost,
            best_moves.len()
        );

        println!("[0] \n{}", burrows);
        for (i, mv) in best_moves.iter().enumerate() {
            println!("{}", mv);
            burrows = move_pod(&burrows, mv);
            println!("[{}] \n{}", i, burrows);
        }

        let timed = SystemTime::now().duration_since(start).unwrap();
        let description = format!("Can get all the amphipods back for a cost of {}", best_cost);

        DayResult {
            description,
            part1: format!("{}", best_cost),
            part2: format!("{}", 0),
            timing_us: timed.as_micros(),
        }
    }
}

// The burrow structure is quite complicated.
struct Burrows {
    // There are four burrows, two deep. These are indexed.
    burrows: [[Contents; 2]; 4],
    // There are seven places above where an amphipod may stop.
    above: [Contents; 7],
    // The cost taken to get to this configuration.
    cost: u64,
    // The moves which go to this state.
    moves: Vec<AMove>,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Contents {
    Empty,
    A,
    B,
    C,
    D,
}

impl Contents {
    fn cost(&self) -> u64 {
        match self {
            Contents::A => 1u64,
            Contents::B => 10u64,
            Contents::C => 100u64,
            Contents::D => 1000u64,
            Contents::Empty => panic!("cannot move an empty space!"),
        }
    }

    fn home(&self) -> usize {
        match self {
            Contents::A => 0,
            Contents::B => 1,
            Contents::C => 2,
            Contents::D => 3,
            Contents::Empty => panic!("cannot move an empty space!"),
        }
    }
}

impl fmt::Display for Contents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Contents::A => write!(f, "A"),
            Contents::B => write!(f, "B"),
            Contents::C => write!(f, "C"),
            Contents::D => write!(f, "D"),
            Contents::Empty => write!(f, "."),
        }
    }
}

impl std::fmt::Display for Burrows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "############# [{}]", self.cost)?;
        writeln!(
            f,
            "#{}{}.{}.{}.{}.{}{}#",
            self.above[0],
            self.above[1],
            self.above[2],
            self.above[3],
            self.above[4],
            self.above[5],
            self.above[6],
        )?;
        writeln!(
            f,
            "###{}#{}#{}#{}###",
            self.burrows[0][0], self.burrows[1][0], self.burrows[2][0], self.burrows[3][0],
        )?;
        writeln!(
            f,
            "  #{}#{}#{}#{}#  ",
            self.burrows[0][1], self.burrows[1][1], self.burrows[2][1], self.burrows[3][1],
        )?;
        writeln!(f, "  #########  ")
    }
}

impl Burrows {
    fn new(burrows: [[Contents; 2]; 4]) -> Self {
        Self {
            above: [Empty; 7],
            burrows,
            cost: 0,
            moves: vec![],
        }
    }

    fn mine() -> Self {
        Burrows::new([
            [Contents::B, Contents::D],
            [Contents::C, Contents::D],
            [Contents::C, Contents::A],
            [Contents::B, Contents::A],
        ])
    }

    fn test() -> Self {
        Burrows::new([
            [Contents::B, Contents::A],
            [Contents::C, Contents::D],
            [Contents::B, Contents::C],
            [Contents::D, Contents::A],
        ])
    }

    fn finished(&self) -> bool {
        for (i, pod) in [Contents::A, Contents::B, Contents::C, Contents::D]
            .iter()
            .enumerate()
        {
            if !(self.burrows[i][0] == *pod && self.burrows[i][1] == *pod) {
                return false;
            }
        }
        true
    }

    fn above_clear(&self, i: usize) -> bool {
        self.above[i] == Contents::Empty
    }

    fn connected(&self, burrow: usize, above: usize) -> bool {
        // Is there a free path between these two locations?
        match above {
            0 => self.above_clear(1) && self.connected(burrow, 1),
            1 => match burrow {
                0 => true,
                1 => self.above_clear(2),
                2 => self.above_clear(2) && self.above_clear(3),
                3 => self.above_clear(2) && self.above_clear(3) && self.above_clear(4),
                _ => panic!("unknown burrow {}", burrow),
            },
            2 => match burrow {
                0 => true,
                1 => true,
                2 => self.above_clear(3),
                3 => self.above_clear(3) && self.above_clear(4),
                _ => panic!("unknown burrow {}", burrow),
            },
            3 => match burrow {
                0 => self.above_clear(2),
                1 => true,
                2 => true,
                3 => self.above_clear(4),
                _ => panic!("unknown burrow {}", burrow),
            },
            4 => match burrow {
                0 => self.above_clear(2) && self.above_clear(3),
                1 => self.above_clear(3),
                2 => true,
                3 => true,
                _ => panic!("unknown burrow {}", burrow),
            },
            5 => match burrow {
                0 => self.above_clear(2) && self.above_clear(3) && self.above_clear(4),
                1 => self.above_clear(3) && self.above_clear(4),
                2 => self.above_clear(4),
                3 => true,
                _ => panic!("unknown burrow {}", burrow),
            },
            6 => self.above_clear(5) && self.connected(burrow, 5),
            _ => panic!("Unexpected corridor location {}", above),
        }
    }

    // What are all the available moves?
    fn available_moves(&self) -> Vec<AMove> {
        let mut moves = vec![];
        // Lets do the 'out' moves first.
        for burrow in 0..4 {
            for depth in 0..=1 {
                if self.burrows[burrow][depth] == Contents::Empty {
                    // Nothing here to move.
                    continue;
                }

                if depth == 0
                    // Both this and the pod below are home. (Otherwise might have to move 
                    // for the pod below.)
                    && self.burrows[burrow][0].home() == burrow
                        && self.burrows[burrow][1].home() == burrow
                {
                    continue;
                } else if depth == 1
                    && (
                        // The space above is occupied!
                        self.burrows[burrow][0] != Contents::Empty
                    // This pod is home. (We're not blocking the upper pod, so don't care.
                        || self.burrows[burrow][1].home() == burrow
                    )
                {
                    continue;
                }

                // We have something that wants to move! Where can it go?
                for above in 0..7 {
                    if self.above[above] != Contents::Empty {
                        // Location isn't free!
                        continue;
                    }

                    if self.connected(burrow, above) {
                        moves.push(AMove {
                            burrow: burrow,
                            depth,
                            above,
                            cost: self.burrows[burrow][depth].cost()
                                * (depth as u64 + move_distance(burrow, above)),
                            direction: MoveDirection::Out,
                        });
                    }
                }
            }
        }

        // Otherwise, lets look at the moves in.
        for above in 0..7 {
            if self.above[above] == Contents::Empty {
                continue;
            }

            // Otherwise, can we move it home? (This is the only permitted move.)
            let burrow = self.above[above].home();
            if !self.connected(burrow, above) {
                continue;
            }

            // Check the bottom.
            if self.burrows[burrow][0] == Contents::Empty {
                // We can move it to the bottom.
                if self.burrows[burrow][1] == Contents::Empty {
                    moves.push(AMove {
                        burrow,
                        depth: 1,
                        above,
                        cost: self.above[above].cost() * (1 + move_distance(burrow, above)),
                        direction: MoveDirection::In,
                    });
                } else if self.burrows[burrow][1].home() == burrow {
                    // Only move it to the top, if the one underneath is already home.
                    moves.push(AMove {
                        burrow,
                        depth: 0,
                        above,
                        cost: self.above[above].cost() * move_distance(burrow, above),
                        direction: MoveDirection::In,
                    });
                }
            }
        }
        moves
    }
}

fn best_cost(from: &Burrows) -> (u64, Vec<AMove>) {
    let mut seen_states = HashMap::new();
    let mut working = from
        .available_moves()
        .iter()
        .map(|m| move_pod(from, m))
        .collect::<Vec<Burrows>>();
    let mut best = u64::max_value();
    let mut best_moves = vec![];
    let mut counter = 0;
    let mut worst = 0;

    while !working.is_empty() {
        counter += 1;
        working.sort_by_key(|b| u64::max_value() - b.cost);
        let try_state = working.pop().unwrap();

        if seen_states.contains_key(&(try_state.burrows, try_state.above)) {
            continue;
        }
        seen_states.insert((try_state.burrows, try_state.above), true);

        /*
        println!("[{}] Checking {}", counter, try_state);

        let mut ret = String::new();
        std::io::stdin()
            .read_line(&mut ret)
            .expect("Failed to read from stdin");

        if try_state.cost > worst {
            worst = try_state.cost;
            println!("Looking at costs: {}", worst);
        }
         */

        if try_state.cost >= best {
            // Already too expensive, so stop.
            break;
        }

        if try_state.finished() {
            println!("Reached end state:\n{}", try_state);
            best = try_state.cost;
            best_moves = vec![AMove::none(); try_state.moves.len()];
            best_moves.clone_from_slice(&try_state.moves);
            continue;
        }

        // Otherwise, lets try all the moves from here.
        let mut next_moves = try_state
            .available_moves()
            .iter()
            .map(|m| move_pod(&try_state, m))
            .collect::<Vec<Burrows>>();
        working.append(&mut next_moves);
    }

    (best, best_moves)
}

fn move_pod(state: &Burrows, m: &AMove) -> Burrows {
    let mut new_state = Burrows {
        burrows: state.burrows,
        above: state.above,
        cost: state.cost + m.cost,
        moves: vec![AMove::none(); state.moves.len()],
    };

    // The move consists of swapping the two locations.
    new_state.burrows[m.burrow][m.depth] = state.above[m.above];
    new_state.above[m.above] = state.burrows[m.burrow][m.depth];

    new_state.moves.clone_from_slice(&state.moves);
    new_state.moves.push(m.clone());

    new_state
}

fn move_distance(burrow: usize, above: usize) -> u64 {
    // Add 1 to get out/into the burrow.
    ((match above {
        0 => (burrow * 2) + 2,
        1 => (burrow * 2) + 1,
        2 => match burrow {
            0 | 1 => 1,
            2 => 3,
            3 => 5,
            _ => panic!("unexpected burrow {}", burrow),
        },
        3 => match burrow {
            0 => 3,
            1 | 2 => 1,
            3 => 3,
            _ => panic!("unexpected burrow {}", burrow),
        },
        4 => match burrow {
            0 => 5,
            1 => 3,
            2 | 3 => 1,
            _ => panic!("unexpected burrow {}", burrow),
        },
        5 => ((3 - burrow) * 2) + 1,
        6 => ((3 - burrow) * 2) + 2,
        _ => panic!("unexpected corridor location {}", above),
    }) + 1) as u64
}

#[derive(Clone)]
enum MoveDirection {
    In,
    Out,
}

#[derive(Clone)]
struct AMove {
    burrow: usize,
    depth: usize,
    above: usize,
    cost: u64,
    direction: MoveDirection,
}

impl AMove {
    fn none() -> Self {
        Self {
            burrow: 0,
            depth: 0,
            above: 0,
            cost: 0,
            direction: MoveDirection::In,
        }
    }
}

impl fmt::Display for AMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bottom_top = match self.depth {
            0 => "top",
            1 => "bottom",
            _ => panic!("unexpected depth {}", self.depth),
        };
        match self.direction {
            MoveDirection::Out => {
                write!(
                    f,
                    "Out of the {} of burrow {} to {}, at a cost of {}",
                    bottom_top, self.burrow, self.above, self.cost
                )
            }
            MoveDirection::In => {
                write!(
                    f,
                    "Into the {} of burrow {} from {}, at a cost of {}",
                    bottom_top, self.burrow, self.above, self.cost
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {}
}
