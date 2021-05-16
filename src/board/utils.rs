use super::{Action, Position};

// Operations on bits

/// Read a given bit of position
pub(super) fn read_bit(pos: &Position, idx: &Action) -> bool {
    let mask = 1 << idx;
    mask & pos == mask
}

/// Sets the bit of a Position at the Action idx.
pub(super) fn set_bit(pos: &mut Position, idx: &Action) {
    let mask = 1 << idx;
    *pos |= mask;
}

// Operations on indexes

/// Find neighbours of a given tile
pub(super) fn neighbours_mask(&idx: &Action) -> Position {
    if idx > 63 {
        0u64
    } else if idx == 63 {
        0b00000000_00000011_00000011u64 << (idx - 9)
    } else if idx > 56 {
        0b00000000_00000111_00000111u64 << (idx - 9)
    } else if idx == 56 {
        0b00000000_00000110_00000110u64 << (idx - 9)
    } else if idx == 0 {
        0b00000110_00000110_00000000u64 >> 9
    } else if idx == 8 {
        0b00000110_00000110_00000110u64 >> 1
    } else if (idx % 8) == 0 {
        0b00000110_00000110_00000110u64 << (idx - 9)
    } else if idx == 7 {
        0b00000011_00000011_00000000u64 >> 2
    } else if (idx % 8) == 7 {
        0b00000011_00000011_00000011u64 << (idx - 9)
    } else if idx < 7 {
        0b00000111_00000111_00000000u64 >> (9 - idx)
    } else {
        0b00000111_00000111_00000111u64 << (idx - 9)
    }
}

/// Move to tile given by direction. If the movement goes out of bound, it returns !0.
pub(super) fn find_next_idx(idx: &Action, direction: &Action) -> Action {
    let potential_next = (idx + direction) % 64;

    if idx < &64 && distance_l_inf(idx, &potential_next) == 1 {
        potential_next
    } else {
        !0
    }
}

/// Return a coordinate from an index
fn idx_to_coord(idx: &Action) -> (Action, Action) {
    (idx % 8, idx / 8)
}

/// Calculates taxi cab distance between indexes (seeing as coordinates)
pub(super) fn distance_l_inf(idx_a: &Action, idx_b: &Action) -> Action {
    let (coo_ax, coo_ay) = idx_to_coord(idx_a);
    let (coo_bx, coo_by) = idx_to_coord(idx_b);

    std::cmp::max(
        coo_ax.max(coo_bx) - coo_ax.min(coo_bx),
        coo_ay.max(coo_by) - coo_ay.min(coo_by),
    )
}
