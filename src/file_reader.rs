use std::io::Read;
use std::thread;

use crossbeam_channel::Sender;

use crate::messages::BytesChunk;
use crate::CHUNK_SIZE;

pub struct FileReader<R> {
    file: R,
    bytes_tx: Sender<BytesChunk>,
}

impl<R> FileReader<R>
where
    R: Read + Send + 'static,
{
    pub fn new(file: R, bytes_tx: Sender<BytesChunk>) -> Self {
        FileReader { file, bytes_tx }
    }

    pub fn read_file(mut self) {
        thread::spawn(move || loop {
            let mut chunk = [0; CHUNK_SIZE];

            let size = self.file.read(&mut chunk).expect("Couldn't read file.");

            if size == 0 {
                break;
            }

            self.bytes_tx
                .send(BytesChunk { size, chunk })
                .expect("Couldn't send bytes chunk.");
        });
    }
}
