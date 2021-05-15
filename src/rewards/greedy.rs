use crate::{AgentId, Board};

pub fn greedy_reward(env: &Board, agent: &AgentId) -> f64 {
    let score = env.score() as f64;

    if *agent == AgentId::White {
        score
    } else {
        -score
    }
}
