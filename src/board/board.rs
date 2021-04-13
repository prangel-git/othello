use super::*;

/// Utility functions for Othello board
impl Board {
    pub(super) fn new() -> Self {
        let mut tile_w = 0;
        let mut tile_b = 0;
        let turn = AgentId::Black;
        let mut valid_v = Vec::new();
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

        valid_v.push(coord_to_idx(&(2, 3)));
        valid_v.push(coord_to_idx(&(3, 2)));
        valid_v.push(coord_to_idx(&(5, 4)));
        valid_v.push(coord_to_idx(&(4, 5)));

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
            valid_v,
            valid,
            borders,
            score: 0,
        }
    }

    /// Place a move in the board by the current player. Returns a vector
    /// with the indxes of tiles that need to be flipped
    pub(super) fn place_tile(&mut self, idx: &Action) -> bool {
        if self.valid.contains(idx) {
            if self.turn == AgentId::White {
                self.score += 1;
                self.tile_w = set_bit(&self.tile_w, idx);
                self.turn = AgentId::Black;
            } else {
                self.score -= 1;
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

            self.borders.remove(idx);

            self.update_valid();

            true
        } else {
            false
        }
    }

    /// Returns a vector containing the tiles from Action of different color until an anchor.
    /// If there is no anchor, it returns the empty vector
    fn find_tiles_to_flip(&self, idx: &Action, neighbour: Action) -> Vec<Action> {
        let direction = find_direction(&neighbour, idx);

        let mut tiles = Vec::new();

        let mut next_idx = find_next_idx(idx, &direction);

        while (next_idx < 64) && self.are_different_color(idx, &next_idx) {
            tiles.push(next_idx);
            next_idx = find_next_idx(&next_idx, &direction);
        }

        if next_idx >= 64 {
            tiles.clear();
        } else if !read_bit(&(self.tile_b | self.tile_w), &next_idx) {
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

            if read_bit(&self.tile_w, idx) {
                self.score += 2;
            } else {
                self.score -= 2;
            }

            true
        }
    }

    /// Update valid moves
    fn update_valid(&mut self) {
        self.valid_v.clear();
        self.valid.clear();
        let mut attempts = 0u8;

        while attempts < 2 {
            let is_white_turn = self.turn == AgentId::White;
            let occupied = self.tile_w | self.tile_b;

            for idx in &self.borders {
                for neighbour in find_neighbours(idx) {
                    let direction = find_direction(&&neighbour, idx);
                    let mut new_idx = neighbour;
                    let mut is_occupied = read_bit(&occupied, &new_idx);
                    let mut is_oposite_color = read_bit(&self.tile_w, &new_idx) != is_white_turn;
                    let mut found_one_oposite = false;

                    while is_occupied && is_oposite_color {
                        found_one_oposite = true;
                        new_idx = find_next_idx(&new_idx, &direction);
                        if new_idx < 64 {
                            is_occupied = read_bit(&occupied, &new_idx);
                            is_oposite_color = read_bit(&self.tile_w, &new_idx) != is_white_turn;
                        } else {
                            break;
                        }
                    }

                    if found_one_oposite && is_occupied && !is_oposite_color {
                        self.valid_v.push(*idx);
                        self.valid.insert(*idx);
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

    /// Checks if two indexes are occupied by different players
    pub(super) fn are_different_color(&self, x: &Action, y: &Action) -> bool {
        (read_bit(&self.tile_w, x) == true) && (read_bit(&self.tile_b, y) == true)
            || (read_bit(&self.tile_b, x) == true) && (read_bit(&self.tile_w, y) == true)
    }

    /// Reads the current score
    pub fn score(&self) -> i8 {
        self.score
    }

    /// Counts the number of white tiles in a position
    pub fn count_white(&self) -> u8 {
        count_ones(&self.tile_w)
    }

    /// Counts the number of white tiles in a position
    pub fn count_black(&self) -> u8 {
        count_ones(&self.tile_b)
    }
}
