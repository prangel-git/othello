mod board;
mod actioniter;
mod display;
mod utils;

use super::AgentId;
use self::utils::*;


use std::collections::HashMap;
use std::collections::HashSet;



type Position = u64;            /// Represents a 64 bits vector. The value of index k, represents a state of the k-th square of a board
type Index = u8;                /// Index of a position
type Coordinate = (u8, u8);     /// Represents a coordinate for when we see a position as a matrix of 8 by 8.

type SetIdx = HashSet<Index>;
type ValidAnchors = HashMap<Index, SetIdx>;

/// Othello board
pub struct Board {
    tile_w: Position,       // Set to 1 iff that square of the board is occupied by White.
    filled: Position,       // Set to 1 iff that square of the board is filled by either White or Black.
    turn: AgentId,          // Identity of the player allowed to make the next move.
    valid_w: SetIdx,        // Keeps a record of valid White moves 
    valid_b: SetIdx,        // Keeps a record of valid Black moves
    anchors: ValidAnchors,  // Returns possible moves for a valid anchor.
}