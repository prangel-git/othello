use std::hash::Hash;

use super::Board;

impl Hash for Board {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tiles_current.hash(state);
        self.tiles_opponent.hash(state);
        self.turn.hash(state);
    }
}
