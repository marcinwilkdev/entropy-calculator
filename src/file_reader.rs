use std::io::Read;
use std::thread;

use crossbeam_channel::Sender;

use crate::{BytesChunk, CHUNK_SIZE};

pub struct FileReader<R> {
    file: R,
    bytes_tx: Sender<BytesChunk>,
}

impl<R> FileReader<R>
where
    R: Read + Send + 'static,
{
    pub fn new(file: R, bytes_tx: Sender<BytesChunk>) -> Self {
        FileReader {
            file,
            bytes_tx,
        }
    }

    pub fn read_file(mut self) {
        thread::spawn(move || loop {
            let mut bytes_chunk = [0; CHUNK_SIZE];

            let chunk_len = self.file.read(&mut bytes_chunk).expect("Couldn't reading file.");

            if chunk_len == 0 {
                break;
            }

            self.bytes_tx
                .send((chunk_len, bytes_chunk))
                .expect("Couldn't send bytes chunk.");
        });
    }
}
