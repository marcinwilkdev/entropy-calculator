use std::thread;

use crossbeam_channel::{Receiver, Sender};

use crate::symbols_counter::SymbolsCounter;
use crate::messages::{BytesChunk, CountedSymbols};

pub struct SymbolsReceiver {
    bytes_rx: Receiver<BytesChunk>,
    symbols_tx: Sender<CountedSymbols>,
}

impl SymbolsReceiver {
    pub fn new(
        bytes_rx: Receiver<BytesChunk>,
        symbols_tx: Sender<CountedSymbols>,
    ) -> SymbolsReceiver {
        SymbolsReceiver { bytes_rx, symbols_tx }
    }

    pub fn count_symbols(self) {
        thread::spawn(move || {
            let mut symbols_counter = SymbolsCounter::new();

            self.bytes_rx
                .iter()
                .for_each(|bytes_chunk| symbols_counter.insert_byte_chunk(bytes_chunk));

            let counted_symbols = symbols_counter.get_counted_symbols();

            self.symbols_tx
                .send(counted_symbols)
                .expect("Couldn't send counted symbols.");
        });
    }
}
