use std::fmt::{Display, Formatter, Result};

use super::*;

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let filled = self.filled;
        let tile_w = self.tile_w;

        let valid = if self.turn == AgentId::White {
            &self.valid_w
        } else {
            &self.valid_b
        };

        let mut mask = 1u64;

        for idx in 0..63 {
            let is_filled = (filled & mask) == mask;
            let is_white = (tile_w & mask) == mask;

            let is_valid = valid.contains(&idx);

            if idx % 8 == 0 {
                write!(f, "\n|")?;
            };

            match (is_filled, is_white) {
                (true, false) => write!(f, " {} |", "B")?,
                (true, true) => write!(f, " {} |", "W")?,
                (false, false) => {
                    if is_valid {
                        write!(f, " {} |", idx)?
                    } else {
                        write!(f, " {} |", " ")?
                    }
                }
                (false, true) => write!(f, " {} |", "?")?,
            };

            mask = mask << 1u64;
        }
        write!(f, "\nEnd of Board")
    }
}
