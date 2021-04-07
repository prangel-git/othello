use super::*;

/// Utility functions for Othello board
impl Board {
    pub(super) fn new() -> Self {
        let mut tile_w = 0;
        let mut tile_b = 0;
        let turn = AgentId::Black;
        let mut valid = HashSet::new();
        let mut borders = HashSet::new();

        tile_w = toggle_bit(&tile_w, &coord_to_idx(&(3, 3)));
        tile_w = toggle_bit(&tile_w, &coord_to_idx(&(4, 4)));

        tile_b = toggle_bit(&tile_b, &coord_to_idx(&(3, 4)));
        tile_b = toggle_bit(&tile_b, &coord_to_idx(&(4, 3)));

        valid.insert(coord_to_idx(&(2, 3)));
        valid.insert(coord_to_idx(&(3, 2)));
        valid.insert(coord_to_idx(&(5, 4)));
        valid.insert(coord_to_idx(&(4, 5)));

        borders.insert(coord_to_idx(&(2, 2)));
        borders.insert(coord_to_idx(&(2, 3)));
        borders.insert(coord_to_idx(&(2, 4)));
        borders.insert(coord_to_idx(&(2, 5)));
        borders.insert(coord_to_idx(&(3, 2)));
        borders.insert(coord_to_idx(&(3, 5)));
        borders.insert(coord_to_idx(&(4, 2)));
        borders.insert(coord_to_idx(&(4, 5)));
        borders.insert(coord_to_idx(&(5, 2)));
        borders.insert(coord_to_idx(&(5, 3)));
        borders.insert(coord_to_idx(&(5, 4)));
        borders.insert(coord_to_idx(&(5, 5)));

        Board {
            tile_w,
            tile_b,
            turn,
            valid,
            borders,
        }
    }

    /// Place a move in the board by the current player. Returns a vector
    /// with the indxes of tiles that need to be flipped
    pub(super) fn place_tile(&mut self, idx: &Action) -> bool {
        if self.valid.contains(idx) {
            if self.turn == AgentId::White {
                self.tile_w = set_bit(&self.tile_w, idx);
                self.turn = AgentId::Black;
            } else {
                self.tile_b = set_bit(&self.tile_b, idx);
                self.turn = AgentId::White;
            }

            let filled = self.tile_b | self.tile_w;
            for neighbour in find_neighbours(idx) {
                if read_bit(&filled, &neighbour) {
                    for tiles in self.find_tiles_to_flip(idx, neighbour) {
                        self.flip(&tiles);
                    }
                    self.borders.remove(&neighbour);
                } else {
                    self.borders.insert(neighbour);
                }
            }

            self.borders.remove(&idx);

            self.update_valid();

            true
        } else {
            false
        }
    }

    /// Returns a vector containing the tiles from Action of different color until an anchor.
    /// If there is no anchor, it returns the empty vector
    fn find_tiles_to_flip(&self, idx: &Action, neighbour: Action) -> Vec<Action> {
        let is_idx_white = read_bit(&self.tile_w, idx);
        let direction = neighbour - idx;

        let mut tiles = Vec::new();

        let mut next_idx = find_next_idx(idx, &direction);
        let mut is_next_white = read_bit(&self.tile_w, &next_idx);
        let mut is_next_occupied = read_bit(&(self.tile_b | self.tile_w), &next_idx);

        while (next_idx < 64) && is_next_white != is_idx_white && is_next_occupied {
            tiles.push(next_idx);
            next_idx = find_next_idx(&next_idx, &direction);
            is_next_white = read_bit(&self.tile_w, &next_idx);
            is_next_occupied = read_bit(&(self.tile_b | self.tile_w), &next_idx);
        }

        if (next_idx >= 64) | !is_next_occupied {
            tiles.clear();
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
        let mut attempts = 0u8;

        while attempts < 2 {
            let is_white_turn = self.turn == AgentId::White;
            let occupied = self.tile_w | self.tile_b;
            let tile_relevant = if is_white_turn {
                self.tile_w
            } else {
                self.tile_b
            };

            for idx in &self.borders {
                for neighbour in find_neighbours(idx) {
                    let is_occupied = read_bit(&occupied, &neighbour);
                    if is_occupied {
                        let is_oposite_color =
                            read_bit(&tile_relevant, &neighbour) != is_white_turn;
                        if is_oposite_color {
                            self.valid.insert(*idx);
                        }
                    }
                }
            }

            if self.valid.is_empty() {
                attempts += 1;
                self.turn = !self.turn;
            } else {
                attempts = 3;
            }
        }
    }

    /// Counts the number of white tiles in a position
    pub(super) fn count_white(&self) -> u8 {
        count_ones(&self.tile_w)
    }

    /// Counts the number of white tiles in a position
    pub(super) fn count_black(&self) -> u8 {
        count_ones(&self.tile_b)
    }
}
