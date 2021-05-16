use super::*;

/// Utility functions for Othello board
impl Board {
    /// Initializes an othello board with the standard initial position.
    pub(super) fn new() -> Self {
        Board {
            turn: AgentId::Black,
            tiles_current: 0b00010000_00001000_00000000_00000000_00000000,
            tiles_opponent: 0b00001000_00010000_00000000_00000000_00000000,
            valid: 0b00001000_00000100_00100000_00010000_00000000_00000000,
            occ_bord: 0b00111100_00111100_00111100_00111100_00000000_00000000,
            count_current: 2,
            count_opponent: 2,
            score: 0,
        }
    }

    /// Executes the move provided by idx.
    pub(super) fn action(&mut self, idx: &Action) -> bool {
        if read_bit(&self.valid, idx) {
            for direction in find_directions_iter(idx, &self.tiles_opponent) {
                let (tiles_to_flip, number_to_flip) = self.find_tiles_to_flip(idx, direction);

                // Flips tiles
                self.tiles_current ^= tiles_to_flip;
                self.tiles_opponent ^= tiles_to_flip;

                // Update score and counts
                self.score += 2 * number_to_flip;
                self.count_current += number_to_flip;
                self.count_opponent -= number_to_flip;
            }

            self.place_tile(idx);
            self.occ_bord |= neighbours_mask(&idx);
            self.update_valid();

            true
        } else {
            false
        }
    }

    /// Places a tile and switches turns.
    fn place_tile(&mut self, idx: &Action) {
        self.score += 1;
        self.count_current += 1;
        set_bit(&mut self.tiles_current, idx);

        self.switch_turn();
    }

    /// Given an index and neighbour, it finds a mask with the tiles to flip.
    /// If no tiles can be flipped, it returns zero.
    fn find_tiles_to_flip(&self, idx: &Action, direction: Action) -> (Position, i8) {
        let mut tiles_to_flip = 0u64;
        let mut number_to_flip = 0i8;

        let mut next_idx = find_next_idx(idx, &direction);

        while (next_idx < 64) && read_bit(&self.tiles_opponent, &next_idx) {
            set_bit(&mut tiles_to_flip, &next_idx);
            number_to_flip += 1;
            next_idx = find_next_idx(&next_idx, &direction);
        }

        if next_idx >= 64 {
            tiles_to_flip = 0;
            number_to_flip = 0;
        } else if !read_bit(&self.tiles_current, &next_idx) {
            tiles_to_flip = 0;
            number_to_flip = 0;
        }

        return (tiles_to_flip, number_to_flip);
    }

    /// Update valid moves
    fn update_valid(&mut self) {
        self.valid = 0;
        let mut attempts = 0u8;

        let occupied = self.tiles_current | self.tiles_opponent;
        let borders = self.occ_bord & !occupied;

        while attempts < 2 {
            for idx in PositionIter::new(&borders) {
                for direction in find_directions_iter(&idx, &self.tiles_opponent) {
                    let (tiles_to_flip, _) = self.find_tiles_to_flip(&idx, direction);
                    if tiles_to_flip != 0 {
                        set_bit(&mut self.valid, &idx);
                        break;
                    }
                }
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
        self.score = -self.score;

        let tile_current = self.tiles_opponent;
        self.tiles_opponent = self.tiles_current;
        self.tiles_current = tile_current;

        let count_current = self.count_opponent;
        self.count_opponent = self.count_current;
        self.count_current = count_current;
    }

    /// Returns current player tiles
    pub fn tiles_current(&self) -> &Position {
        &self.tiles_current
    }

    /// Returns oponent player tiles
    pub fn tiles_opponent(&self) -> &Position {
        &self.tiles_opponent
    }

    /// Counts the number of white tiles in a position
    pub fn count_white(&self) -> i8 {
        if self.turn == AgentId::White {
            self.count_current
        } else {
            self.count_opponent
        }
    }

    /// Counts the number of white tiles in a position
    pub fn count_black(&self) -> i8 {
        if self.turn == AgentId::Black {
            self.count_current
        } else {
            self.count_opponent
        }
    }

    /// Returns current score
    pub fn score(&self) -> i8 {
        self.score
    }

    /// Returns white tiles
    pub fn tiles_w(&self) -> Position {
        if self.turn == AgentId::White {
            self.tiles_current
        } else {
            self.tiles_opponent
        }
    }

    /// Returns black tiles
    pub fn tiles_b(&self) -> Position {
        if self.turn == AgentId::Black {
            self.tiles_current
        } else {
            self.tiles_opponent
        }
    }
}
