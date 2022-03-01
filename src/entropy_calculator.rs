use std::collections::HashMap;

pub struct EntropyCalculator {
    probabilities: [f64; u8::MAX as usize + 1],
    conditional_probabilities: HashMap<(u8, u8), f64>,
}

impl EntropyCalculator {
    pub fn new(
        probabilities: [f64; u8::MAX as usize + 1],
        conditional_probabilities: HashMap<(u8, u8), f64>,
    ) -> EntropyCalculator {
        EntropyCalculator {
            probabilities,
            conditional_probabilities,
        }
    }

    pub fn calculate_hx(&self) -> f64 {
        -1.0 * self
            .probabilities
            .iter()
            .filter(|px| **px > 0.0)
            .fold(0.0, |sum, px| sum + px * px.log2())
    }

    pub fn calculate_hyx(&self) -> f64 {
        self.probabilities
            .iter()
            .enumerate()
            .filter(|(_, px)| **px > 0.0)
            .map(|(x1, px)| {
                let px_ln = px.log2();

                self.conditional_probabilities
                    .iter()
                    .filter(|(_, pyx)| **pyx > 0.0)
                    .filter(|((_, x2), _)| *x2 as usize == x1)
                    .fold(0.0, |sum, (_, pyx)| sum + pyx * (px_ln - pyx.log2()))
            })
            .sum::<f64>()
    }
}
