use crate::{Symbols, SymbolsPairs, CHUNK_SIZE};

pub struct BytesChunk {
    pub last_symbol: u8,
    pub size: usize,
    pub chunk: [u8; CHUNK_SIZE],
}

pub struct CountedSymbols {
    pub symbols: Symbols,
    pub cond_symbols: SymbolsPairs,
    pub count: f64,
}

impl CountedSymbols {
    pub fn merge(&mut self, other: CountedSymbols) {
        self.symbols
            .iter_mut()
            .enumerate()
            .for_each(|(symbol, symbol_count)| *symbol_count += other.symbols[symbol]);

        // not working, fix needed
        self.cond_symbols
            .iter_mut()
            .enumerate()
            .for_each(|(symbol, symbol_count)| {
                symbol_count.iter_mut().enumerate().for_each(
                    |(after_symbol, after_sumbol_count)| {
                        *after_sumbol_count += other.cond_symbols[symbol][after_symbol]
                    },
                )
            });

        self.count += other.count;
    }
}
