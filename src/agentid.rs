use std::fmt::{Display, Formatter, Result};

/// Identity of Othello players
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum AgentId {
    Black,
    White,
}

impl Display for AgentId {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AgentId::Black => write!(f, "Black Player"),
            AgentId::White => write!(f, "White Player"),
        }
    }
}
