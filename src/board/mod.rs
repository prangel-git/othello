mod board;
mod display;
mod environment;
mod hash;
mod partial_eq;
mod utils;

use std::collections::HashSet;

use self::utils::*;

use super::Action;
use super::AgentId;

/// Represents a 64 bits vector. The value of index k, represents a state of the k-th square of a board
type Position = u64;

type SetIdx = HashSet<Action>;
type VecIdx = Vec<Action>;

/// Othello board
#[derive(Clone, Debug)]
pub struct Board {
    tile_w: Position,   // Set to 1 iff that square of the board is occupied by White.
    tile_b: Position,   // Set to 1 iff that square of the board is occupied by Black.
    turn: AgentId,      // Identity of the player allowed to make the next move.
    valid_v: VecIdx,    // Keeps a record of valid moves as vector.
    valid: Position,    // Keeps a record of valid moves as set.
    occupied: Position, // Keeps record of occupied spaces.
    borders: SetIdx,    // Keeps a record of the border.
    count_w: i8,        // Counts white tiles
    count_b: i8,        // Counts black tiles
    score: i8,          // White tiles minus black tiles.
}
