use std::io::Read;
use std::thread;

use crossbeam_channel::Sender;

use crate::messages::BytesChunk;
use crate::CHUNK_SIZE;

pub struct SymbolsReader<R> {
    source: R,
    bytes_tx: Sender<BytesChunk>,
}

impl<R> SymbolsReader<R>
where
    R: Read + Send + 'static,
{
    pub fn new(source: R, bytes_tx: Sender<BytesChunk>) -> Self {
        SymbolsReader { source, bytes_tx }
    }

    pub fn read_symbols(mut self) {
        thread::spawn(move || loop {
            let mut chunk = [0; CHUNK_SIZE];

            let size = self.source.read(&mut chunk).expect("Couldn't read file.");

            if size == 0 {
                break;
            }

            self.bytes_tx
                .send(BytesChunk { size, chunk })
                .expect("Couldn't send bytes chunk.");
        });
    }
}
