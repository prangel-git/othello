use super::Board;

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        (self.turn == other.turn) && (self.tile_w == other.tile_w) && (self.tile_b == other.tile_b)
    }
}

impl Eq for Board {}
