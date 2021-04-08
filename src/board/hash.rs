use std::hash::Hash;

use super::Board;

impl Hash for Board {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tile_b.hash(state);
        self.tile_w.hash(state);
        self.turn.hash(state);
    }
}
