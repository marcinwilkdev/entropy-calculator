
use crate::{ConditionalProbabilities, Probabilities, CHUNK_SIZE};

pub struct BytesChunk {
    pub size: usize,
    pub chunk: [u8; CHUNK_SIZE],
}

pub struct CountedSymbols {
    pub symbols: Probabilities,
    pub cond_symbols: ConditionalProbabilities,
    pub count: f64,
}

impl CountedSymbols {
    pub fn merge(&mut self, other: CountedSymbols) {
        self.symbols
            .iter_mut()
            .enumerate()
            .for_each(|(symbol, symbol_count)| {
                *symbol_count = *symbol_count + other.symbols[symbol]
            });

        self.cond_symbols
            .iter_mut()
            .enumerate()
            .for_each(|(symbol, symbol_count)| {
                symbol_count.iter_mut().enumerate().for_each(
                    |(after_symbol, after_sumbol_count)| {
                        *after_sumbol_count =
                            *after_sumbol_count + other.cond_symbols[symbol][after_symbol]
                    },
                )
            });

        self.count += other.count;
    }
}
