use super::*;

/// Utility functions for Othello board
impl Board {
    /// Flips a tile on the board. It returns true iff the tile was flipped correctly.
    fn flip(&mut self, idx: &Index) -> bool {
        if read_bit(&self.filled, idx) {
            false
        } else {
            // TODO: write logic to change anchors and valid moves based on the flip

            self.tile_w = toggle_bit(&self.tile_w, idx);
            true
        }
    }

    /// Given an index and a direction, it finds the first index that doesn't match
    /// the current tile color in that direction. It returns !0 if cannot find such tile.
    fn find_unmatching_tile_in_direction(
        &self, 
        idx: &Index, 
        dir: Direction, 
        is_forward: bool) -> Index {
            !0
    }

}
