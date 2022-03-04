use crate::messages::{BytesChunk, CountedSymbols};

pub struct SymbolsCounter {
    last_symbol: u8,
    symbols_count: f64,
    probs: [f64; 256],
    cond_probs: [[f64; 256]; 256],
}

impl SymbolsCounter {
    pub fn new() -> SymbolsCounter {
        SymbolsCounter {
            last_symbol: 0,
            symbols_count: 0.0,
            probs: [0.0; 256],
            cond_probs: [[0.0; 256]; 256],
        }
    }

    pub fn insert_byte_chunk(&mut self, bytes_chunk: BytesChunk) {
        let BytesChunk { size, chunk } = bytes_chunk;

        let bytes_chunk = &chunk[..size];

        self.symbols_count += size as f64;

        bytes_chunk
            .iter()
            .for_each(|&symbol| self.insert_symbol(symbol));
    }

    fn insert_symbol(&mut self, symbol: u8) {
        self.probs[symbol as usize] += 1.0;

        self.cond_probs[self.last_symbol as usize][symbol as usize] += 1.0;

        self.last_symbol = symbol;
    }

    pub fn get_probs(self) -> CountedSymbols {
        CountedSymbols {
            symbols: self.probs,
            cond_symbols: self.cond_probs,
            count: self.symbols_count,
        }
    }
}

impl Default for SymbolsCounter {
    fn default() -> Self {
        Self::new()
    }
}
