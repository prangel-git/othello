/// An action will be a number from 0 to 63 representing a position in an 8 by 8 board
pub type Action = u8;

mod agentid;
pub use agentid::AgentId;

mod board;
pub use board::Board;

mod rewards;
pub use rewards::*;

pub use gts::abstractions::*;
pub use gts::agents::*;
pub use gts::tree_search::*;

pub use genetic::*;
