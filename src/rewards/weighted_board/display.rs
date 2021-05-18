use super::WeightedBoard;
use std::fmt::{Display, Formatter, Result};

impl Display for WeightedBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for idx in 0..64 {
            if idx % 8 == 0 {
                write!(f, "\n|")?
            }
            write!(f, " {:03} |", self.get_weight(idx))?
        }
        Ok(())
    }
}
