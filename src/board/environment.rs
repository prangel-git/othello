use crate::{Action, Environment};

use super::*;

/// Implements environment for Othello
impl Environment<Action, AgentId> for Board {
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

    fn valid_actions(&self) -> Box<dyn Iterator<Item = Action>> {
        Box::new(PositionIter::new(&self.valid))
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
            if self.score() > 0 {
                Some(self.turn)
            } else if self.score() < 0 {
                Some(!self.turn)
            } else {
                None
            }
        }
    }
}
