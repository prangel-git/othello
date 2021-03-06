use std::fmt::{Display, Formatter, Result};

use super::*;

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let tile_b = self.tiles_b();
        let tile_w = self.tiles_w();

        let mut mask = 1u64;

        for idx in 0..64 {
            let is_black = (tile_b & mask) == mask;
            let is_white = (tile_w & mask) == mask;

            if idx % 8 == 0 {
                write!(f, "\n|")?;
            };

            match (is_black, is_white) {
                (true, false) => write!(f, " {} |", " B ")?,
                (false, true) => write!(f, " {} |", " W ")?,
                (false, false) => {
                    if read_bit(&self.valid, &idx) {
                        write!(f, " {:02}  |", idx)?
                    } else {
                        write!(f, " {} |", "   ")?
                    }
                }
                (true, true) => write!(f, " {} |", "?")?,
            };

            mask = mask << 1;
        }
        write!(f, "\nEnd of Board \n {:?}", self)
    }
}
