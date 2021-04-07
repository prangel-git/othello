use super::*;

/// Utility functions for Othello board
impl Board {
    /// Place a move in the board by the current player. Returns a vector
    /// with the indxes of tiles that need to be flipped
    pub(super) fn place_tile(&mut self, idx: &Action) -> bool {
        if self.valid.contains(idx) {
            let is_white_turn = self.turn == AgentId::White;

            for neighbour in find_neighbours(idx) {
                let tile_relevant = if is_white_turn {
                    self.tile_w
                } else {
                    self.tile_b
                };

                if read_bit(&&tile_relevant, &neighbour) {
                    for tiles in self.find_tiles_to_flip(idx, neighbour) {
                        self.flip(&tiles);
                    }
                    self.borders.remove(&neighbour);
                } else {
                    self.borders.insert(neighbour);
                }
            }

            self.borders.remove(&idx);

            if is_white_turn {
                self.tile_w = set_bit(&self.tile_w, idx);
                self.turn = AgentId::Black;
            } else {
                self.tile_b = set_bit(&self.tile_b, idx);
                self.turn = AgentId::White;
            }

            self.update_valid();

            true
        } else {
            false
        }
    }

    /// Returns a vector containing the tiles from Action of different color until an anchor.
    /// If there is no anchor, it returns the empty vector
    fn find_tiles_to_flip(&self, idx: &Action, neighbour: Action) -> Vec<Action> {
        let is_white_turn = self.turn == AgentId::White;
        let direction = neighbour - idx;

        let mut tiles = Vec::new();

        let mut next_idx = find_next_idx(idx, &direction);
        while (next_idx < 64)
            && (read_bit(&self.tile_w, &next_idx) != is_white_turn)
            && read_bit(&(self.tile_b | self.tile_w), &next_idx)
        {
            tiles.push(next_idx);
            next_idx = find_next_idx(&next_idx, &direction);
        }

        return tiles;
    }

    /// Flips a tile on the board. It returns true iff the tile was flipped correctly.
    fn flip(&mut self, idx: &Action) -> bool {
        if !read_bit(&(self.tile_w | self.tile_b), idx) {
            false
        } else {
            self.tile_w = toggle_bit(&self.tile_w, idx);
            self.tile_b = toggle_bit(&self.tile_b, idx);
            true
        }
    }

    /// Update valid moves
    fn update_valid(&mut self) {
        self.valid.clear();
    }
}
