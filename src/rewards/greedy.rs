use crate::{AgentId, Board};

pub fn greedy_reward(env: &Board, _agent: &AgentId) -> f64 {
    env.score() as f64
}
