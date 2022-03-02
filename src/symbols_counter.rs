use std::thread;

use crossbeam_channel::{Receiver, Sender};

use crate::{BytesChunk, ReadyProbabilities};
use crate::probabilities_calculator::ProbabilitiesCalculator;

pub struct SymbolsCounter {
    bytes_rx: Receiver<BytesChunk>,
    probs_tx: Sender<ReadyProbabilities>,
}

impl SymbolsCounter {
    pub fn new(
        bytes_rx: Receiver<BytesChunk>,
        probs_tx: Sender<ReadyProbabilities>,
    ) -> SymbolsCounter {
        SymbolsCounter { bytes_rx, probs_tx }
    }

    pub fn count_symbols(self) {
        thread::spawn(move || {
            let mut probabilities_calculator = ProbabilitiesCalculator::new();

            self.bytes_rx
                .iter()
                .for_each(|bytes_chunk| probabilities_calculator.insert_byte_chunk(bytes_chunk));

            let ready_probabilities = probabilities_calculator.get_probs();

            self.probs_tx
                .send(ready_probabilities)
                .expect("Couldn't send ready probabilites.");
        });
    }
}
