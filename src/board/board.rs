use super::*;

/// Utility functions for Othello board
impl Board {
    pub(super) fn new() -> Self {
        Board {
            tile_w: 0b00001000_00010000_00000000_00000000_00000000,
            tile_b: 0b00010000_00001000_00000000_00000000_00000000,
            turn: AgentId::Black,
            tile_o: 0b00001000_00010000_00000000_00000000_00000000,
            tile_c: 0b00010000_00001000_00000000_00000000_00000000,
            valid_v: vec![20, 29, 34, 43],
            valid: 0b00001000_00000100_00100000_00010000_00000000_00000000,
            occ_bord: 0b00111100_00111100_00111100_00111100_00000000_00000000,
            count_b: 2,
            count_w: 2,
            score: 0,
        }
    }

    /// Executes the move provided by idx.
    pub(super) fn action(&mut self, idx: &Action) -> bool {
        if read_bit(&self.valid, idx) {
            for neighbour in find_neighbours(idx) {
                if read_bit(self.opponent_tiles(), &neighbour) {
                    let (tiles_to_flip, number_to_flip) = self.find_tiles_to_flip(idx, neighbour);
                    self.tile_b ^= tiles_to_flip;
                    self.tile_w ^= tiles_to_flip;

                    if self.turn == AgentId::White {
                        self.score += 2 * number_to_flip;
                        self.count_w += number_to_flip;
                        self.count_b -= number_to_flip;
                    } else {
                        self.score -= 2 * number_to_flip;
                        self.count_w -= number_to_flip;
                        self.count_b += number_to_flip;
                    }
                }
            }

            self.place_tile(idx);
            self.occ_bord |= neighbours_mask(&idx);
            self.update_valid();

            true
        } else {
            false
        }
    }

    /// Places a tile in an empty space
    fn place_tile(&mut self, idx: &Action) {
        if self.turn == AgentId::White {
            self.score += 1;
            self.count_w += 1;
            set_bit(&mut self.tile_w, idx);
            self.tile_o = self.tile_b;
            self.tile_c = self.tile_w;
        } else {
            self.score -= 1;
            self.count_b += 1;
            set_bit(&mut self.tile_b, idx);
            self.tile_o = self.tile_w;
            self.tile_c = self.tile_b;
        }
        self.switch_turn();
    }

    /// Returns a vector containing the tiles from Action of different color until an anchor.
    /// If there is no anchor, it returns the empty vector
    fn find_tiles_to_flip(&self, idx: &Action, neighbour: Action) -> (Position, i8) {
        let direction = find_direction(&neighbour, idx);

        let mut tiles_to_flip = 0u64;
        let mut number_to_flip = 0i8;

        let mut next_idx = find_next_idx(idx, &direction);

        while (next_idx < 64) && read_bit(self.opponent_tiles(), &next_idx) {
            set_bit(&mut tiles_to_flip, &next_idx);
            number_to_flip += 1;
            next_idx = find_next_idx(&next_idx, &direction);
        }

        if next_idx >= 64 {
            tiles_to_flip = 0;
            number_to_flip = 0;
        } else if !read_bit(self.current_tiles(), &next_idx) {
            tiles_to_flip = 0;
            number_to_flip = 0;
        }

        return (tiles_to_flip, number_to_flip);
    }

    /// Update valid moves
    fn update_valid(&mut self) {
        self.valid_v.clear();
        self.valid = 0;
        let mut attempts = 0u8;

        let occupied = self.tile_w | self.tile_b;

        while attempts < 2 {
            let mut idx = 0;
            let mut borders = self.occ_bord & !occupied;

            while borders != 0 {
                if read_bit(&borders, &0) {
                    for neighbour in find_neighbours(&idx) {
                        if read_bit(self.opponent_tiles(), &neighbour) {
                            let (tiles_to_flip, _) = self.find_tiles_to_flip(&idx, neighbour);
                            if tiles_to_flip != 0 {
                                self.valid_v.push(idx);
                                set_bit(&mut self.valid, &idx);
                                break;
                            }
                        }
                    }
                }
                borders >>= 1;
                idx += 1;
            }

            if self.valid == 0 {
                attempts += 1;
                self.switch_turn();
            } else {
                attempts = 3;
            }
        }
    }

    /// Switches turns
    fn switch_turn(&mut self) {
        self.turn = !self.turn;
        let tile_c = self.tile_o;
        self.tile_o = self.tile_c;
        self.tile_c = tile_c;
    }

    /// Returns current player tiles
    fn current_tiles(&self) -> &Position {
        &self.tile_c
    }

    /// Returns oponent player tiles
    fn opponent_tiles(&self) -> &Position {
        &self.tile_o
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

    /// Returns white tiles
    pub fn tiles_w(&self) -> Position {
        self.tile_w
    }

    /// Returns black tiles
    pub fn tiles_b(&self) -> Position {
        self.tile_b
    }
}
