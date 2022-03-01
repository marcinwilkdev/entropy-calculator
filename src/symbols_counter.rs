use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::{FirstMessage, SecondMessage};

pub struct SymbolsCounter {
    receiver: Option<Receiver<FirstMessage>>,
    sender: Option<Sender<SecondMessage>>,
}

impl SymbolsCounter {
    pub fn new(receiver: Receiver<FirstMessage>, sender: Sender<SecondMessage>) -> SymbolsCounter {
        SymbolsCounter {
            receiver: Some(receiver),
            sender: Some(sender),
        }
    }

    pub fn count_symbols(&mut self) {
        if self.receiver.is_none() || self.sender.is_none() {
            eprintln!("Method already called");
            return;
        }

        let receiver = self.receiver.take().unwrap();
        let sender = self.sender.take().unwrap();

        thread::spawn(move || {
            let mut last_symbol = 0;
            let mut symbols_count = 0.0;

            let mut probabilities = [0.0; u8::MAX as usize + 1];
            let mut conditional_probabilites = HashMap::new();

            while let Ok((content_len, content)) = receiver.recv() {
                symbols_count += content_len as f64;

                let content = &content[..content_len];

                for &symbol in content {
                    probabilities[symbol as usize] += 1.0;

                    *conditional_probabilites
                        .entry((symbol, last_symbol))
                        .or_insert(0.0) += 1.0;

                    last_symbol = symbol;
                }
            }

            for prob in &mut probabilities {
                *prob /= symbols_count;
            }

            for (_, cond_prob) in &mut conditional_probabilites {
                *cond_prob /= symbols_count;
            }

            sender
                .send((probabilities, conditional_probabilites))
                .expect("Error sending message");
        });
    }
}
