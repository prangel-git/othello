use gts::abstractions::Environment;

use crate::{AgentId, Board};

/// Gives weights to an othello board. These weights will be used to calculate a reward
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct WeightedBoard {
    pub(super) weights: [u8; 9],
    weights_full: [u8; 64],
}

impl WeightedBoard {
    /// Creates a new weighted board in which all squares have the same weight
    pub fn new(weights: [u8; 9]) -> Self {
        let mut weights_full = [0u8; 64];
        for i in 0..8 {
            for j in 0..8 {
                let idx = (8 * i + j) as usize;
                weights_full[idx] = calculate_weight(&weights, i, j)
            }
        }

        WeightedBoard {
            weights,
            weights_full,
        }
    }

    /// Get weight from index
    pub fn get_weight(&self, idx: u8) -> u8 {
        self.weights_full[idx as usize]
    }

    /// Calculates the reward for a particular board based on the given weights.
    pub fn reward(&self, env: &Board, &agent: &AgentId) -> f64 {
        let mut total_reward = 0i64;

        let mut tiles_current = *env.tiles_current();
        let mut tiles_opponent = *env.tiles_opponent();

        let mut idx = 0u8;

        while (tiles_current | tiles_opponent) != 0 {
            if (tiles_current & 1) == 1 {
                total_reward += self.get_weight(idx) as i64;
            } else if (tiles_opponent & 1) == 1 {
                total_reward -= self.get_weight(idx) as i64;
            }

            tiles_current >>= 1;
            tiles_opponent >>= 1;

            idx += 1;
        }

        if env.turn() == agent {
            total_reward as f64
        } else {
            -total_reward as f64
        }
    }
}

/// Gets the weight of a square in the board. Note that the weights preserve the symmetries of the board.
fn calculate_weight(weights: &[u8; 9], i: u8, j: u8) -> u8 {
    if i > 7 || j > 7 {
        0
    } else if i > 3 {
        calculate_weight(weights, 3 - (i % 4), j)
    } else if j > 3 {
        calculate_weight(weights, i, 3 - (j % 4))
    } else if j > i {
        calculate_weight(weights, j, i)
    } else if j == 0 {
        weights[i as usize]
    } else if j == 1 {
        weights[(i + 3) as usize]
    } else if j == 2 {
        weights[(i + 5) as usize]
    } else {
        1
    }
}
