use super::{Action, Position};

/// Read a given bit of position
pub(super) fn read_bit(pos: &Position, idx: &Action) -> bool {
    let mask = 1 << idx;
    mask & pos == mask
}

/// Set the bit of a Position at the Action idx.
pub(super) fn set_bit(pos: &Position, idx: &Action) -> Position {
    let mask = 1 << idx;
    pos | mask
}

/// Toggle bit of a position at the Action idx.
pub(super) fn toggle_bit(pos: &Position, idx: &Action) -> Position {
    let mask = 1 << idx;
    pos ^ mask
}

/// Find neighbours of a given tile
pub(super) fn find_neighbours(idx: &Action) -> Vec<Action> {
    todo!();
}

/// Move to tile given by direction. If the movement goes out of bound, it returns !0.
pub(super) fn find_next_idx(idx: &Action, direction: &Action) -> Action {
    todo!();
}
