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

/// Clears the bit of a Position at the Action idx.
pub(super) fn clear_bit(pos: &mut Position, idx: &Action) {
    let mask = !(1 << idx);
    *pos &= mask
}

/// Toggle bit of a position at the Action idx.
pub(super) fn toggle_bit(pos: &mut Position, idx: &Action) {
    let mask = 1 << idx;
    *pos ^= mask
}

// Operations on indexes

/// Find neighbours of a given tile
pub(super) fn find_neighbours(&idx: &Action) -> Vec<Action> {
    if idx > 63 {
        Vec::new()
    } else if idx == 63 {
        vec![62, 55, 54]
    } else if idx > 56 {
        vec![idx + 1, idx - 1, idx - 7, idx - 8, idx - 9]
    } else if idx == 56 {
        vec![57, 48, 49]
    } else if idx == 0 {
        vec![1, 8, 9]
    } else if (idx % 8) == 0 {
        vec![idx + 1, idx + 8, idx + 9, idx - 8, idx - 7]
    } else if idx == 7 {
        vec![6, 15, 14]
    } else if (idx % 8) == 7 {
        vec![idx - 1, idx - 8, idx - 9, idx + 8, idx + 7]
    } else if idx < 7 {
        vec![idx + 1, idx - 1, idx + 7, idx + 8, idx + 9]
    } else {
        vec![
            idx + 1,
            idx - 1,
            idx + 7,
            idx + 8,
            idx + 9,
            idx - 7,
            idx - 8,
            idx - 9,
        ]
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

/// Find direction between two indexes (That is, a number to use in combination with find_next_idx)
pub(super) fn find_direction(a: &Action, b: &Action) -> Action {
    let direction = a.max(b) - a.min(b);

    if a < b {
        64 - direction
    } else {
        direction
    }
}

/// Return an index from a coordinate
pub(super) fn coord_to_idx(&(coo_x, coo_y): &(Action, Action)) -> Action {
    coo_x + 8 * coo_y
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
