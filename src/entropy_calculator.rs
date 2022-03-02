use crate::{ConditionalProbabilities, Probabilities};

pub struct EntropyCalculator {
    probs: Probabilities,
    cond_probs: ConditionalProbabilities,
}

impl EntropyCalculator {
    pub fn new(probs: Probabilities, cond_probs: ConditionalProbabilities) -> EntropyCalculator {
        EntropyCalculator { probs, cond_probs }
    }

    pub fn calculate_hx(&self) -> f64 {
        -1.0 * self
            .probs
            .iter()
            .filter(|px| **px > 0.0)
            .fold(0.0, |sum, px| sum + px * px.log2())
    }

    pub fn calculate_hyx(&self) -> f64 {
        self.probs
            .iter()
            .enumerate()
            .filter(|(_, px)| **px > 0.0)
            .map(|(x1, px)| {
                let px_log2 = px.log2();

                self.cond_probs
                    .iter()
                    .filter(|(_, pyx)| **pyx > 0.0)
                    .filter(|((_, x2), _)| *x2 as usize == x1)
                    .fold(0.0, |partial_sum, (_, pyx)| {
                        partial_sum + pyx * (px_log2 - pyx.log2())
                    })
            })
            .sum::<f64>()
    }
}
