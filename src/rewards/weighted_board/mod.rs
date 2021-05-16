mod genetic;

use crate::{AgentId, Board};

/// Gives weights to an othello board. These weights will be used to calculate a reward

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct WeightedBoard {
    weights: [u8; 9],
}

impl WeightedBoard {
    /// Creates a new weighted board in which all squares have the same weight
    pub fn new() -> Self {
        WeightedBoard { weights: [1; 9] }
    }

    /// Sets the reward for one of the 9 principal squares.
    pub fn set_reward(&mut self, idx: u8, value: u8) {
        let idx = (idx % 9) as usize;
        self.weights[idx] = value;
    }

    /// Gets the weight of a square in the board. Note that the weights preserve the symmetries of the board.
    pub fn get_weight(&self, i: u8, j: u8) -> u8 {
        if i > 7 || j > 7 {
            0
        } else if i > 3 {
            self.get_weight(3 - (i % 4), j)
        } else if j > 3 {
            self.get_weight(i, 3 - (j % 4))
        } else if j > i {
            self.get_weight(j, i)
        } else if j == 0 {
            self.weights[i as usize]
        } else if j == 1 {
            self.weights[(i + 3) as usize]
        } else if j == 2 {
            self.weights[(i + 5) as usize]
        } else {
            1
        }
    }

    /// Calculates the reward for a particular board based on the given weights.
    pub fn reward(&self, env: &Board, _agent: &AgentId) -> f64 {
        
        let mut total_reward = 0i64;

        let mut tiles_current = *env.tiles_current();
        let mut tiles_opponent = *env.tiles_opponent();

        let mut idx = 0u8;

        while (tiles_current | tiles_opponent) == 0 {
            let (i, j) = (idx % 8, idx / 8);

            if (tiles_current & 1) == 1 {
                total_reward += self.get_weight(i, j) as i64;
            } else if (tiles_opponent & 1) == 1 {
                total_reward -= self.get_weight(i, j) as i64;
            }

            tiles_current >>= 1;
            tiles_opponent >>= 1;

            idx += 1;
        }

        return total_reward as f64;
    }
}
