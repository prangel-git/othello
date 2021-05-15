use super::*;

use rand::Rng;

use crate::Chromosome;
use crate::Genetic;

impl Genetic for WeightedBoard {
    fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let mut weights = [0; 9];

        for w in &mut weights {
            *w = rng.gen::<u8>();
        }

        WeightedBoard { weights }
    }

    fn choromosome(&self) -> Chromosome {
        let mut lower = 0u64;

        for i in 0..8 {
            lower <<= 8;
            lower |= self.weights[i] as u64;
        }

        let upper = self.weights[8] as u64;

        let slice = [lower, upper];

        Chromosome::from_slice(&slice).unwrap()
    }

    fn from_chromosome(chromosome: Chromosome) -> Self {
        if chromosome.is_empty() {
            WeightedBoard::new()
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

            WeightedBoard { weights }
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
