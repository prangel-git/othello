use super::{Coordinate, Index, Position};
/// Determines the direction from tile
pub(super) enum Direction {
    East,
    NorthEast,
    North,
    NorthWest,
    None,
}

/// From index to coordinates
pub(super) fn idx_to_coord(idx: Index) -> Coordinate {
    (idx / 8, idx % 8)
}

/// From coordinates to index
pub(super) fn coord_to_idx(coo: Coordinate) -> Index {
    8 * coo.0 + coo.1
}

/// Read a given bit of position
pub(super) fn read_bit(pos: &Position, idx: &Index) -> bool {
    let mask = 1 << idx;
    mask & pos == mask
}

/// Set the bit of a Position at the index idx.
pub(super) fn set_bit(pos: &Position, idx: &Index) -> Position {
    let mask = 1 << idx;
    pos | mask
}

/// Clear the bit of a position at the index idx.
pub(super) fn clear_bit(pos: &Position, idx: &Index) -> Position {
    let mask = !(1 << idx);
    pos & mask
}

/// Toggle bit of a position at the index idx.
pub(super) fn toggle_bit(pos: &Position, idx: &Index) -> Position {
    let mask = 1 << idx;
    pos ^ mask
}

/// Finds the direction between two tiles.
pub(super) fn find_direction(idx_a: &Index, idx_b: &Index) -> Direction {
    let max_idx = idx_a.max(idx_b);
    let min_idx = idx_a.min(idx_b);

    let diff = max_idx - min_idx;

    if diff == 0 {
        Direction::None
    } else if max_idx / 8 == min_idx / 8 {
        Direction::East
    } else if diff % 7 == 0 {
        Direction::NorthWest
    } else if diff % 8 == 0 {
        Direction::North
    } else if diff % 9 == 0 {
        Direction::NorthEast
    } else {
        Direction::None
    }
}

/// Gives the index obtained by moving in a direction either forward or backward. If you go out of bounds, returns !0
pub(super) fn move_direction(&idx: &Index, dir: &Direction, &is_forward: &bool) -> Index {
    if idx >= 64 {
        !0
    } else {
        let coo = idx_to_coord(idx);

        match (dir, is_forward) {
            (Direction::East, true) => {
                if 7 == coo.1 {
                    !0
                } else {
                    idx + 1
                }
            }
            (Direction::East, false) => {
                if 0 == coo.1 {
                    !0
                } else {
                    idx - 1
                }
            }
            (Direction::NorthEast, true) => {
                if 7 == coo.0 || 7 == coo.1 {
                    !0
                } else {
                    idx + 9
                }
            }
            (Direction::NorthEast, false) => {
                if 0 == coo.0 || 0 == coo.1 {
                    !0
                } else {
                    idx - 9
                }
            }
            (Direction::North, true) => {
                if 7 == coo.0 {
                    !0
                } else {
                    idx + 8
                }
            }
            (Direction::North, false) => {
                if 0 == coo.0 {
                    !0
                } else {
                    idx - 8
                }
            }
            (Direction::NorthWest, true) => {
                if 7 == coo.0 || 0 == coo.1 {
                    !0
                } else {
                    idx + 7
                }
            }
            (Direction::NorthWest, false) => {
                if 0 == coo.0 || 7 == coo.1 {
                    !0
                } else {
                    idx - 7
                }
            }
            (Direction::None, _) => !0,
        }
    }
}
