use crate::{AgentId, Board};


/// Gives weights to an othello board. These weights will be used to calculate a reward
pub struct WeightedBoard {
    weights: [u8; 9],
}

impl WeightedBoard {
    /// Creates a new weighted board in which all squares have the same weight
    pub fn new() -> Self {
        WeightedBoard{ weights: [1; 9] }
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
            self.get_weight(3 - (i % 4) , j)
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
    pub fn reward(&self, env: &Board, agent: &AgentId) -> f64 {
        let mut total_reward = 0f64;

        let mut tiles_w = env.tiles_w();
        let mut tiles_b = env.tiles_b();

        let mut idx = 0u8;

        while (tiles_w | tiles_b) == 0 {
            let (i, j) = (idx % 8, idx / 8);

            if (tiles_w & 1) == 1 { 
                total_reward += self.get_weight(i, j) as f64;
            } else if (tiles_b & 1) == 1 {
                total_reward -= self.get_weight(i, j) as f64;
            }
            
            tiles_w >>= 1;
            tiles_b >>= 1;

            idx += 1;
        };

        if *agent == AgentId::White {
            total_reward
        } else {
            -total_reward
        }
    }
}