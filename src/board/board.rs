use super::*;

/// Utility functions for Othello board
impl Board {
    pub(super) fn new() -> Self {
        let mut output = Board {
            tile_w: 0,
            tile_b: 0,
            turn: AgentId::Black,
            valid_v: Vec::new(),
            valid: 0,
            occupied: 0,
            borders: 0,
            count_b: 2,
            count_w: 2,
            score: 0,
        };

        output.place_tile(&coord_to_idx(&(3, 3)));
        output.place_tile(&coord_to_idx(&(3, 4)));
        output.place_tile(&coord_to_idx(&(4, 4)));
        output.place_tile(&coord_to_idx(&(4, 3)));

        output.initialize_border();

        output.update_valid();

        return output;
    }

    /// Executes the move provided by idx.
    pub(super) fn action(&mut self, idx: &Action) -> bool {
        if read_bit(&self.valid, idx) {
            self.place_tile(idx);

            for neighbour in find_neighbours(idx) {
                if read_bit(&self.occupied, &neighbour) {
                    for tiles in self.find_tiles_to_flip(idx, neighbour) {
                        self.flip(&tiles);
                    }
                    self.borders = clear_bit(&self.borders, &neighbour);
                } else {
                    set_bit(&mut self.borders, &neighbour);
                }
            }

            self.borders = clear_bit(&self.borders, idx);

            self.update_valid();

            true
        } else {
            false
        }
    }

    /// Initializes border of board
    fn initialize_border(&mut self) {
        set_bit(&mut self.borders, &coord_to_idx(&(2, 2)));
        set_bit(&mut self.borders, &coord_to_idx(&(2, 3)));
        set_bit(&mut self.borders, &coord_to_idx(&(2, 4)));
        set_bit(&mut self.borders, &coord_to_idx(&(2, 5)));
        set_bit(&mut self.borders, &coord_to_idx(&(3, 2)));
        set_bit(&mut self.borders, &coord_to_idx(&(3, 5)));
        set_bit(&mut self.borders, &coord_to_idx(&(4, 2)));
        set_bit(&mut self.borders, &coord_to_idx(&(4, 5)));
        set_bit(&mut self.borders, &coord_to_idx(&(5, 2)));
        set_bit(&mut self.borders, &coord_to_idx(&(5, 3)));
        set_bit(&mut self.borders, &coord_to_idx(&(5, 4)));
        set_bit(&mut self.borders, &coord_to_idx(&(5, 5)));
    }

    /// Places a tile in an empty space
    fn place_tile(&mut self, idx: &Action) {
        if !read_bit(&self.occupied, idx) {
            if self.turn == AgentId::White {
                self.score += 1;
                self.count_w += 1;
                set_bit(&mut self.tile_w, idx);
            } else {
                self.score -= 1;
                self.count_b += 1;
                set_bit(&mut self.tile_b, idx);
            }
            self.turn = !self.turn;
            set_bit(&mut self.occupied, idx);
        };
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
        } else if !read_bit(&self.occupied, &next_idx) {
            tiles.clear();
        }

        return tiles;
    }

    /// Flips a tile on the board. It returns true iff the tile was flipped correctly.
    fn flip(&mut self, idx: &Action) {
        if read_bit(&self.occupied, idx) {
            self.tile_w = toggle_bit(&self.tile_w, idx);
            self.tile_b = toggle_bit(&self.tile_b, idx);

            if read_bit(&self.tile_w, idx) {
                self.score += 2;
                self.count_w += 1;
                self.count_b -= 1;
            } else {
                self.score -= 2;
                self.count_w -= 1;
                self.count_b += 1;
            }
        }
    }

    /// Update valid moves
    fn update_valid(&mut self) {
        self.valid_v.clear();
        self.valid = 0;
        let mut attempts = 0u8;

        while attempts < 2 {
            let is_white_turn = self.turn == AgentId::White;

            let mut idx = 0;
            let mut borders = self.borders;

            while borders != 0 {
                if read_bit(&borders, &0) {
                    for neighbour in find_neighbours(&idx) {
                        let direction = find_direction(&neighbour, &idx);
                        let mut new_idx = neighbour;
                        let mut is_occupied = read_bit(&self.occupied, &new_idx);
                        let mut is_oposite_color =
                            read_bit(&self.tile_w, &new_idx) != is_white_turn;
                        let mut found_one_oposite = false;

                        while is_occupied && is_oposite_color {
                            found_one_oposite = true;
                            new_idx = find_next_idx(&new_idx, &direction);
                            if new_idx < 64 {
                                is_occupied = read_bit(&self.occupied, &new_idx);
                                is_oposite_color =
                                    read_bit(&self.tile_w, &new_idx) != is_white_turn;
                            } else {
                                break;
                            }
                        }

                        if found_one_oposite && is_occupied && !is_oposite_color {
                            self.valid_v.push(idx);
                            set_bit(&mut self.valid, &idx);
                        }
                    }
                }
                borders >>= 1;
                idx += 1;
            }

            if self.valid == 0 {
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

    /// Counts the number of white tiles in a position
    pub fn count_white(&self) -> i8 {
        self.count_w
    }

    /// Counts the number of white tiles in a position
    pub fn count_black(&self) -> i8 {
        self.count_b
    }

    /// Returns current score
    pub fn score(&self) -> i8 {
        self.score
    }
}
