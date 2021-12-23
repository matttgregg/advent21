use crate::day23::Contents::Empty;
use crate::{DayResult, DaySolver};
use std::cmp::{Ord, Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::fmt;
use std::time::SystemTime;

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let start = SystemTime::now();

        let burrows = Burrows::mine();
        let unfolded_burrows = burrows.unfold();

        let (best_cost, _best_moves) = find_best_moves(&burrows);
        let (best_cost_unfolded, _best_moves_unfolded) = find_best_moves(&unfolded_burrows);

        let timed = SystemTime::now().duration_since(start).unwrap();
        let description = format!(
            "Can get all the amphipods back for a cost of {}. After \
        unfolding, minimum energy is {}",
            best_cost, best_cost_unfolded
        );

        DayResult {
            description,
            part1: format!("{}", best_cost),
            part2: format!("{}", best_cost_unfolded),
            timing_us: timed.as_micros(),
        }
    }
}

// The burrow structure is quite complicated.
#[derive(Eq)]
struct Burrows {
    // There are four burrows, two deep. These are indexed.
    burrows: [Vec<Contents>; 4],
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
        writeln!(f, "{} = {}", self.state(), self.cost)?;
        writeln!(f, "#############")?;
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
        for i in 0..self.burrows[0].len() {
            writeln!(
                f,
                "###{}#{}#{}#{}###",
                self.burrows[0][i], self.burrows[1][i], self.burrows[2][i], self.burrows[3][i],
            )?;
        }
        writeln!(f, "  #########  ")
    }
}

impl Ord for Burrows {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Burrows {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Burrows {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state() && self.cost == other.cost
    }
}

impl Burrows {
    fn new(burrows: [Vec<Contents>; 4]) -> Self {
        Self {
            above: [Empty; 7],
            burrows,
            cost: 0,
            moves: vec![],
        }
    }

    fn mine() -> Self {
        Burrows::new([
            vec![Contents::B, Contents::D],
            vec![Contents::C, Contents::D],
            vec![Contents::C, Contents::A],
            vec![Contents::B, Contents::A],
        ])
    }

    #[cfg(test)]
    fn test() -> Self {
        Burrows::new([
            vec![Contents::B, Contents::A],
            vec![Contents::C, Contents::D],
            vec![Contents::B, Contents::C],
            vec![Contents::D, Contents::A],
        ])
    }

    fn unfold(&self) -> Self {
        // Unlfolding inserts the rows
        //   #D#C#B#A#
        //   #D#B#A#C#
        let mut new_state = Self {
            above: self.above,
            cost: 0,
            burrows: [vec![], vec![], vec![], vec![]],
            moves: vec![],
        };

        for (i, to_add) in [
            [Contents::D, Contents::D],
            [Contents::C, Contents::B],
            [Contents::B, Contents::A],
            [Contents::A, Contents::C],
        ]
        .iter()
        .enumerate()
        {
            new_state.burrows[i].push(self.burrows[i][0]);
            for a in to_add {
                new_state.burrows[i].push(*a);
            }
            new_state.burrows[i].push(self.burrows[i][1]);
        }

        new_state
    }

    fn finished(&self) -> bool {
        // All burrows must be full, and all burrows must be home.
        for i in 0..4 {
            for b in &self.burrows[i] {
                if *b == Contents::Empty {
                    return false;
                }
                if b.home() != i {
                    return false;
                }
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
            for (depth, occupant) in self.burrows[burrow].iter().enumerate() {
                if *occupant == Contents::Empty {
                    // Empty!
                    continue;
                }

                // If non-empty, if home and all below are home, we'll leave this burrow as is.
                if occupant.home() == burrow
                    && self.burrows[burrow]
                        .iter()
                        .skip(depth + 1)
                        .all(|u| u.home() == burrow)
                {
                    break;
                }

                // We have something that wants to move! Where can it go?
                for above in 0..7 {
                    if self.above[above] != Contents::Empty {
                        // Location isn't free!
                        continue;
                    }

                    if self.connected(burrow, above) {
                        moves.push(AMove {
                            burrow,
                            depth,
                            above,
                            cost: self.burrows[burrow][depth].cost()
                                * (depth as u64 + move_distance(burrow, above)),
                            direction: MoveDirection::Out,
                        });
                    }
                }

                // We couldn't move from this burrow, so need to move to the next.
                break;
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

            // Find the next empty slot.
            for (depth, occupant) in self.burrows[burrow].iter().enumerate().rev() {
                if *occupant == Contents::Empty {
                    // We've got a gap, so make a move.
                    moves.push(AMove {
                        burrow,
                        depth,
                        above,
                        cost: self.above[above].cost()
                            * (depth as u64 + move_distance(burrow, above)),
                        direction: MoveDirection::In,
                    });
                    break;
                } else if occupant.home() != burrow {
                    // There's a non-home pod here, so can't move.
                    break;
                }
                // There's a home pod here, so keep looking for a gap.
            }
        }
        moves
    }

    fn state(&self) -> String {
        format!(
            "{}:{}",
            self.above
                .iter()
                .map(|s| format!("{}", s))
                .collect::<Vec<String>>()
                .join(""),
            self.burrows
                .iter()
                .map(|b| b
                    .iter()
                    .map(|s| format!("{}", s))
                    .collect::<Vec<String>>()
                    .join(""),)
                .collect::<Vec<String>>()
                .join(":"),
        )
    }
}

fn find_best_moves(from: &Burrows) -> (u64, Vec<AMove>) {
    let mut seen_states = HashMap::new();
    let mut working = BinaryHeap::new();
    for b in from
        .available_moves()
        .iter()
        .map(|m| move_pod(from, m, false))
    {
        working.push(Reverse(b));
    }

    let mut best = u64::max_value();
    let mut best_moves = vec![];

    while !working.is_empty() {
        let try_state = working.pop().unwrap().0;

        if seen_states.contains_key(&try_state.state()) {
            continue;
        }
        seen_states.insert(try_state.state(), true);

        if try_state.cost >= best {
            // Already too expensive, so stop.
            break;
        }

        if try_state.finished() {
            best = try_state.cost;
            best_moves = vec![AMove::none(); try_state.moves.len()];
            best_moves.clone_from_slice(&try_state.moves);
            continue;
        }

        // Otherwise, lets try all the moves from here.
        for next in try_state
            .available_moves()
            .iter()
            .map(|m| move_pod(&try_state, m, false))
        {
            working.push(Reverse(next));
        }
    }

    (best, best_moves)
}

fn move_pod(state: &Burrows, m: &AMove, caching: bool) -> Burrows {
    let mut new_state = Burrows {
        burrows: [vec![], vec![], vec![], vec![]],
        above: state.above,
        cost: state.cost + m.cost,
        moves: vec![],
    };

    for i in 0..4 {
        new_state.burrows[i] = vec![Contents::Empty; state.burrows[i].len()];
        new_state.burrows[i].clone_from_slice(&state.burrows[i]);
    }

    // The move consists of swapping the two locations.
    new_state.burrows[m.burrow][m.depth] = state.above[m.above];
    new_state.above[m.above] = state.burrows[m.burrow][m.depth];

    if caching {
        new_state.moves = vec![AMove::none(); state.moves.len()];
        new_state.moves.clone_from_slice(&state.moves);
        new_state.moves.push(m.clone());
    }

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

#[derive(Clone, PartialEq, Eq)]
enum MoveDirection {
    In,
    Out,
}

#[derive(Clone, PartialEq, Eq)]
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
        match self.direction {
            MoveDirection::Out => {
                write!(
                    f,
                    "Out of the {} level of burrow {} to {}, at a cost of {}",
                    self.depth, self.burrow, self.above, self.cost
                )
            }
            MoveDirection::In => {
                write!(
                    f,
                    "Into the {} level of burrow {} from {}, at a cost of {}",
                    self.depth, self.burrow, self.above, self.cost
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let burrows = Burrows::test();
        let unfolded_burrows = burrows.unfold();

        let (best_cost, _best_moves) = find_best_moves(&burrows);
        let (best_cost_unfolded, _best_moves_unfolded) = find_best_moves(&unfolded_burrows);
        assert_eq!(best_cost, 12521);
        assert_eq!(best_cost_unfolded, 44169);
    }
}
