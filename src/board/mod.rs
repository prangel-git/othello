mod actioniter;
mod board;
mod display;
mod utils;

use self::utils::*;
use super::AgentId;

use std::collections::HashMap;
use std::collections::HashSet;

type Position = u64;
/// Represents a 64 bits vector. The value of index k, represents a state of the k-th square of a board
type Index = u8;
/// Index of a position
type Coordinate = (u8, u8);
/// Represents a coordinate for when we see a position as a matrix of 8 by 8.

type ValidAnchors = HashMap<Index, HashSet<Index>>;

/// Othello board
pub struct Board {
    tile_w: Position,      // set to 1 iff that square of the board is occupied by White.
    filled: Position, // set to 1 iff that square of the board is filled by either White or Black.
    turn: AgentId,    // Identity of the player allowed to make the next move.
    valid_w: ValidAnchors, // Stores the anchors of every valid move for White.
    valid_b: ValidAnchors, // Stores the anchors of every valud moves for Black.
}
