use std::collections::HashMap;
use std::thread;

use crossbeam_channel::{Receiver, Sender};

use crate::{BytesChunk, ReadyProbabilities};

pub struct SymbolsCounter {
    bytes_rx: Option<Receiver<BytesChunk>>,
    probs_tx: Option<Sender<ReadyProbabilities>>,
}

impl SymbolsCounter {
    pub fn new(
        bytes_rx: Receiver<BytesChunk>,
        probs_tx: Sender<ReadyProbabilities>,
    ) -> SymbolsCounter {
        SymbolsCounter {
            bytes_rx: Some(bytes_rx),
            probs_tx: Some(probs_tx),
        }
    }

    // change to taking self instead of reference
    pub fn count_symbols(&mut self) {
        if self.bytes_rx.is_none() || self.probs_tx.is_none() {
            eprintln!("Method already called.");
            return;
        }

        let bytes_rx = self.bytes_rx.take().unwrap();
        let probs_tx = self.probs_tx.take().unwrap();

        thread::spawn(move || {
            let mut last_symbol = 0;
            let mut symbols_count = 0.0;

            let mut probs = [0.0; u8::MAX as usize + 1];
            let mut cond_probs = HashMap::new();

            while let Ok((chunk_len, bytes_chunk)) = bytes_rx.recv() {
                symbols_count += chunk_len as f64;

                let bytes_chunk = &bytes_chunk[..chunk_len];

                for &symbol in bytes_chunk {
                    probs[symbol as usize] += 1.0;

                    let cond_probs_entry = cond_probs
                        .entry((symbol, last_symbol))
                        .or_insert(0.0);

                    *cond_probs_entry += 1.0;

                    last_symbol = symbol;
                }
            }

            for prob in &mut probs {
                *prob /= symbols_count;
            }

            for (_, cond_prob) in &mut cond_probs {
                *cond_prob /= symbols_count;
            }

            probs_tx
                .send((probs, cond_probs))
                .expect("Couldn't send ready probabilites.");
        });
    }
}
