use gts::abstractions::Environment;

use crate::{AgentId, Board};

pub fn greedy_reward(env: &Board, &agent: &AgentId) -> f64 {
    if agent == env.turn() {
        env.score() as f64
    } else {
        -env.score() as f64
    }
}
