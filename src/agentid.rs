use std::fmt::{Display, Formatter, Result};
use std::ops::Not;

/// Identity of Othello players
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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

impl Not for AgentId {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self == AgentId::Black {
            AgentId::White
        } else {
            AgentId::Black
        }
    }
}
