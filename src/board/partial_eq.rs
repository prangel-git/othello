use super::Board;

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        (self.turn == other.turn)
            && (self.tiles_current == other.tiles_current)
            && (self.tiles_opponent == other.tiles_opponent)
    }
}

impl Eq for Board {}
