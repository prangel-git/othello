use super::Position;

// Implements an iterator from a position
pub struct PositionIter {
    position: Position,
    index: u8,
}

impl PositionIter {
    pub fn new(&position: &Position) -> Self {
        PositionIter { position, index: 0 }
    }
}

impl Iterator for PositionIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position == 0 {
            None
        } else {
            while (self.position & 1) == 0 {
                self.position >>= 1;
                self.index += 1;
            }

            let output = self.index;

            self.position >>= 1;
            self.index += 1;

            return Some(output);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn position_iterator() {
        let index: [u8; 7] = [1, 3, 6, 7, 8, 9, 11];
        let pos = 0b1011_1100_1010u64;

        for (i, &j) in PositionIter::new(&pos).zip(index.iter()) {
            assert_eq!(i, j);
        }
    }
}
