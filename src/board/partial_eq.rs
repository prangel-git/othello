use super::Board;

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        (self.turn == other.turn)
            && (self.tile_current == other.tile_current)
            && (self.tile_opponent == other.tile_opponent)
    }
}

impl Eq for Board {}
