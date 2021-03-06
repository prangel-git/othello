mod board;
mod display;
mod environment;
mod hash;
mod partial_eq;
mod position_iter;
mod utils;

use self::utils::*;

use super::Action;
use super::AgentId;

use position_iter::PositionIter;

/// Represents a 64 bits vector. The value of index k, represents a state of the k-th square of a board
type Position = u64;

/// Othello board
#[derive(Clone, Debug)]
pub struct Board {
    turn: AgentId,            // Identity of the player allowed to make the next move.
    tiles_current: Position, // Set to 1 iff that square of the board is occupied by current turn player.
    tiles_opponent: Position, // Set to 1 iff that square of the board is occupied by opposite turn player.
    valid: Position,          // Keeps a record of valid moves as set.
    occ_bord: Position,       // Keeps record of occupied spaces with border.
    count_current: i8,        // Counts white tiles
    count_opponent: i8,       // Counts black tiles
    score: i8,                // White tiles minus black tiles.
}
