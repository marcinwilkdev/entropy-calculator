use std::collections::HashMap;

use crate::{BytesChunk, ReadyProbabilities};

pub struct ProbabilitiesCalculator {
    last_symbol: u8,
    symbols_count: f64,
    probs: [f64; 256],
    cond_probs: HashMap<(u8, u8), f64>,
}

impl ProbabilitiesCalculator {
    pub fn new() -> ProbabilitiesCalculator {
        ProbabilitiesCalculator {
            last_symbol: 0,
            symbols_count: 0.0,
            probs: [0.0; 256],
            cond_probs: HashMap::with_capacity(65536),
        }
    }

    pub fn insert_byte_chunk(&mut self, bytes_chunk: BytesChunk) {
        let (chunk_len, bytes_chunk) = bytes_chunk;

        self.symbols_count += chunk_len as f64;

        let bytes_chunk = &bytes_chunk[..chunk_len];

        bytes_chunk
            .iter()
            .for_each(|&symbol| self.insert_symbol(symbol));
    }

    fn insert_symbol(&mut self, symbol: u8) {
        self.probs[symbol as usize] += 1.0;

        let cond_probs_entry = self
            .cond_probs
            .entry((symbol, self.last_symbol))
            .or_insert(0.0);

        *cond_probs_entry += 1.0;

        self.last_symbol = symbol;
    }

    fn calculate_probs(&mut self) {
        for prob in &mut self.probs {
            *prob /= self.symbols_count;
        }

        for (_, cond_prob) in &mut self.cond_probs {
            *cond_prob /= self.symbols_count;
        }
    }

    pub fn get_probs(mut self) -> ReadyProbabilities {
        self.calculate_probs();

        (self.probs, self.cond_probs)
    }
}
