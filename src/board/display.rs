use std::fmt::{Display, Formatter, Result};

use super::*;

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let tile_b = self.tile_b;
        let tile_w = self.tile_w;

        let mut mask = 1u64;

        for idx in 0..63 {
            let is_black = (tile_b & mask) == mask;
            let is_white = (tile_w & mask) == mask;

            let is_valid = self.valid.contains(&idx);

            if idx % 8 == 0 {
                write!(f, "\n|")?;
            };

            match (is_black, is_white) {
                (true, false) => write!(f, " {} |", "B")?,
                (false, true) => write!(f, " {} |", "W")?,
                (false, false) => {
                    if is_valid {
                        write!(f, " {} |", idx)?
                    } else {
                        write!(f, " {} |", " ")?
                    }
                }
                (true, true) => write!(f, " {} |", "?")?,
            };

            mask = mask << 1;
        }
        write!(f, "\nEnd of Board")
    }
}
