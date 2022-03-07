use crossbeam_channel::{Receiver, Sender};

use crate::messages::{BytesChunk, CountedSymbols};
use crate::symbols_receiver::SymbolsReceiver;

pub struct CounterPool {
    bytes_rx: Receiver<BytesChunk>,
    symbols_tx: Sender<CountedSymbols>,
    symbols_rx: Receiver<CountedSymbols>,
}

impl CounterPool {
    pub fn new(bytes_rx: Receiver<BytesChunk>) -> CounterPool {
        let (symbols_tx, symbols_rx) = crossbeam_channel::unbounded::<CountedSymbols>();

        CounterPool {
            bytes_rx,
            symbols_tx,
            symbols_rx,
        }
    }

    pub fn count_symbols(&mut self, receiver_threads: usize) -> CountedSymbols {
        SymbolsReceiver::new(self.bytes_rx.clone(), self.symbols_tx.clone()).count_symbols();

        for _ in 0..receiver_threads - 1 {
            SymbolsReceiver::new(self.bytes_rx.clone(), self.symbols_tx.clone()).count_symbols();
        }

        let mut counted_symbols = self
            .symbols_rx
            .recv()
            .expect("Couldn't fetch counted symbols.");

        for _ in 0..receiver_threads - 1 {
            let next_counted_symbols = self
                .symbols_rx
                .recv()
                .expect("Couldn't fetch counted symbols.");

            counted_symbols.merge(next_counted_symbols);
        }

        counted_symbols
    }
}
