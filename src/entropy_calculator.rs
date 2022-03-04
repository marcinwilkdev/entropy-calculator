use crate::messages::CountedSymbols;

pub struct EntropyCalculator {
    counted_symbols: CountedSymbols,
    count_x_log_2_cache: [f64; 256],
}

impl EntropyCalculator {
    pub fn new(counted_symbols: CountedSymbols) -> EntropyCalculator {
        EntropyCalculator {
            counted_symbols,
            count_x_log_2_cache: [0.0; 256],
        }
    }

    pub fn calculate_hx(&mut self) -> f64 {
        let CountedSymbols { symbols, count, .. } = self.counted_symbols;

        let log_2_count_all = count.log2();

        symbols
            .iter()
            .enumerate()
            .filter(|(_, count_x)| **count_x > 0.0)
            .fold(0.0, |sum, (x, count_x)| {
                let count_x_log_2 = count_x.log2();
                self.count_x_log_2_cache[x as usize] = count_x_log_2; // caching

                sum + count_x * (log_2_count_all - count_x_log_2)
            })
            / count
    }

    pub fn calculate_hyx(&self) -> f64 {
        let CountedSymbols {
            symbols,
            cond_symbols,
            count,
        } = self.counted_symbols;

        symbols
            .iter()
            .enumerate()
            .filter(|(_, count_x)| **count_x > 0.0)
            .map(|(x1, _)| {
                let count_x_log_2 = self.count_x_log_2_cache[x1]; // cache

                cond_symbols[x1]
                    .iter()
                    .filter(|count_yx| **count_yx > 0.0)
                    .fold(0.0, |partial_sum, count_yx| {
                        partial_sum + count_yx * (count_x_log_2 - count_yx.log2())
                    })
            })
            .sum::<f64>()
            / count
    }
}
