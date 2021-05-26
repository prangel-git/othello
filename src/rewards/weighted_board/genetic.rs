use super::*;

use bitvec::prelude::*;


use rand::Rng;

use crate::Genetic;

impl Genetic for WeightedBoard {
    type Chromosome = BitVec<Lsb0, u64>;

    fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let mut weights = [0; 9];

        for w in &mut weights {
            *w = rng.gen::<u8>();
        }

        WeightedBoard::new(weights)
    }

    fn choromosome(&self) -> Self::Chromosome {
        let mut lower = 0u64;

        for i in 0..8 {
            lower <<= 8;
            lower |= self.weights[i] as u64;
        }

        let upper = self.weights[8] as u64;

        let slice = [lower, upper];

        Self::Chromosome::from_slice(&slice).unwrap()
    }

    fn from_chromosome(chromosome: Self::Chromosome) -> Self {
        if chromosome.is_empty() {
            WeightedBoard::new([1; 9])
        } else {
            let weights_vec = chromosome.into_vec();
            let mut lower = weights_vec[0];

            let mut weights = [0u8; 9];

            for i in (0..8).rev() {
                weights[i] = lower as u8;
                lower >>= 8;
            }

            let upper = if weights_vec.len() >= 2 {
                weights_vec[1] as u8
            } else {
                0u8
            };

            weights[8] = upper;

            WeightedBoard::new(weights)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recovered_from_chromosome() {
        let board = WeightedBoard::new_random();
        let chromosome = board.choromosome();
        let recovered_board = WeightedBoard::from_chromosome(chromosome);

        assert_eq!(board, recovered_board);
    }
}
