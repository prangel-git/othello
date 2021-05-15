use crate::{Action, Environment};

use super::*;

/// Implements environment for Othello
impl Environment<Action, AgentId> for Board {
    type ActionIter = std::vec::IntoIter<u8>;

    fn initial_state() -> Self {
        Board::new()
    }

    fn update(&mut self, a: &Action) -> bool {
        self.action(a)
    }

    fn what_if(&self, a: &Action) -> Self {
        let mut board = self.clone();
        board.update(a);
        return board;
    }

    fn valid_actions(&self) -> Self::ActionIter {
        self.valid_v.clone().into_iter()
    }

    fn is_valid(&self, action: &Action) -> bool {
        read_bit(&self.valid, action)
    }

    fn is_terminal(&self) -> bool {
        self.valid == 0
    }

    fn turn(&self) -> AgentId {
        self.turn
    }

    fn winner(&self) -> Option<AgentId> {
        if self.valid != 0 {
            None
        } else {
            if self.score < 0 {
                Some(AgentId::Black)
            } else if self.score > 0 {
                Some(AgentId::White)
            } else {
                None
            }
        }
    }
}
