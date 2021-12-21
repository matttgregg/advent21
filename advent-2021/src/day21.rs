use crate::{DayResult, DaySolver};
use std::collections::HashMap;
use std::time::SystemTime;

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let start = SystemTime::now();

        let winning = play(10, 6, 1, 100);

        let mut dirac = DiracGame::new();
        let init_state = GameState::new(6, 1);
        let wins = dirac.play(&init_state);
        println!("Full dirac completed with :: {} vs {}", wins.0, wins.1);
        let best_wins = if wins.0 > wins.1 { wins.0 } else { wins.1 };

        let timed = SystemTime::now().duration_since(start).unwrap();
        let description = format!(
            "Winning combination to 1000 is {}. Dirac universe wins are \
        {} vs {}",
            winning, wins.0, wins.1
        );

        DayResult {
            description,
            part1: format!("{}", winning),
            part2: format!("{}", best_wins),
            timing_us: timed.as_micros(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct GameState {
    player: u8,   // 4
    p1_pos: u8,   // 16
    p2_pos: u8,   // 16
    p1_score: u8, // 32
    p2_score: u8, // 32
}

impl GameState {
    fn new(p1: u8, p2: u8) -> Self {
        Self {
            player: 0,
            p1_pos: p1 - 1,
            p2_pos: p2 - 1,
            p1_score: 0,
            p2_score: 0,
        }
    }
}

struct DiracGame {
    // The cache returns (Player 1 wins, Player 2 wins). (0,0) is undecided.
    cache: HashMap<GameState, (u128, u128)>,
}

impl DiracGame {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn play(&mut self, state: &GameState) -> (u128, u128) {
        // Check for a cache hit:
        if let Some(wins) = self.cache.get(state) {
            return *wins;
        }

        //println!("Playing game: {:?}", state);

        // Otherwise we need to work it out.

        // Check if one player has won.
        if state.p1_score >= 21 {
            return (1, 0);
        }
        if state.p2_score >= 21 {
            return (0, 1);
        }

        let mut wins = (0, 0);
        // Otherwise we need to play all the possible roll states.
        // Note here that I've explicitly created all 27 combinations of 3 rolls [1,2,3].
        for (roll, freq) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)].iter() {
            // For simplicity, handle the two players separately.
            if state.player == 0 {
                let new_pos = (state.p1_pos + *roll) % 10;
                let new_state = GameState {
                    player: 1, // Player
                    p1_pos: new_pos,
                    p2_pos: state.p2_pos,
                    p1_score: state.p1_score + (new_pos + 1),
                    p2_score: state.p2_score,
                };
                let sub_wins = self.play(&new_state);
                wins.0 += sub_wins.0 * freq;
                wins.1 += sub_wins.1 * freq;
            } else {
                let new_pos = (state.p2_pos + *roll) % 10;
                let new_state = GameState {
                    player: 0, // Player
                    p1_pos: state.p1_pos,
                    p2_pos: new_pos,
                    p1_score: state.p1_score,
                    p2_score: state.p2_score + (new_pos + 1),
                };
                let sub_wins = self.play(&new_state);
                wins.0 += sub_wins.0 * freq;
                wins.1 += sub_wins.1 * freq;
            }
        }

        self.cache.insert(state.clone(), wins);
        wins
    }
}

fn play(board_size: usize, p1: usize, p2: usize, die_size: usize) -> u128 {
    let mut next_die = 1;
    let mut rolls = 0;
    let mut player = 0usize;
    let mut player_scores = [0u128; 2];
    let mut player_pos = [p1 - 1, p2 - 1];

    loop {
        // There's some nicety around the end of the dice roll.
        let mut three_roll = 0;
        if next_die + 2 < die_size {
            three_roll = (3 * next_die + 3) % board_size; // d, d+1, d+2 = 3d + 3
            next_die = next_die + 3;
        } else {
            // We need to carefully handle the wrap around.
            for _ in 0..3 {
                three_roll += next_die;
                next_die = (next_die % die_size) + 1;
            }
            three_roll = three_roll % board_size;
        }
        rolls += 3;

        player_pos[player] = (player_pos[player] + three_roll) % board_size;
        player_scores[player] += player_pos[player] as u128 + 1u128;

        if player_scores[player] >= 1000 {
            // We take the losing players score.
            player = (player + 1) % 2;
            return player_scores[player] * rolls;
        }

        // Otherwise move to the next player
        player = (player + 1) % 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        assert_eq!(play(10, 4, 8, 100), 739785);

        let mut dirac = DiracGame::new();
        let init_state = GameState::new(4, 8);
        let wins = dirac.play(&init_state);
        assert_eq!(wins, (444356092776315, 341960390180808));
    }
}
