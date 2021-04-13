mod board;
mod display;
mod environment;
mod hash;
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    tile_w: Position, // Set to 1 iff that square of the board is occupied by White.
    tile_b: Position, // Set to 1 iff that square of the board is occupied by Black.
    turn: AgentId,    // Identity of the player allowed to make the next move.
    valid_v: VecIdx,  // Keeps a record of valid moves as vector.
    valid: SetIdx,    // Keeps a record of valid moves as set.
    borders: SetIdx,  // Keeps a record of the border.
    score: i8,        // Counts the difference between white tiles and black tiles.
}
