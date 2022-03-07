use crate::messages::{BytesChunk, CountedSymbols};
use crate::{Symbols, SymbolsPairs};

pub struct SymbolsCounter {
    last_symbol: u8,
    symbols_count: f64,
    symbols: Symbols,
    symbols_pairs: SymbolsPairs,
}

impl SymbolsCounter {
    pub fn new() -> SymbolsCounter {
        SymbolsCounter {
            last_symbol: 0,
            symbols_count: 0.0,
            symbols: [0.0; 256],
            symbols_pairs: [[0.0; 256]; 256],
        }
    }

    pub fn insert_byte_chunk(&mut self, bytes_chunk: BytesChunk) {
        let BytesChunk { last_symbol, size, chunk } = bytes_chunk;

        self.last_symbol = last_symbol;

        let bytes_chunk = &chunk[..size];

        self.symbols_count += size as f64;

        bytes_chunk
            .iter()
            .for_each(|&symbol| self.insert_symbol(symbol));
    }

    fn insert_symbol(&mut self, symbol: u8) {
        self.symbols[symbol as usize] += 1.0;

        self.symbols_pairs[self.last_symbol as usize][symbol as usize] += 1.0;

        self.last_symbol = symbol;
    }

    pub fn get_counted_symbols(self) -> CountedSymbols {
        CountedSymbols {
            symbols: self.symbols,
            cond_symbols: self.symbols_pairs,
            count: self.symbols_count,
        }
    }
}

impl Default for SymbolsCounter {
    fn default() -> Self {
        Self::new()
    }
}
