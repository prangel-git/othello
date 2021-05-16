use std::hash::Hash;

use super::Board;

impl Hash for Board {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tile_current.hash(state);
        self.tile_opponent.hash(state);
        self.turn.hash(state);
    }
}
