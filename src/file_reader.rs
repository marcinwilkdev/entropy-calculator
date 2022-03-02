use std::io::Read;
use std::thread;

use crossbeam_channel::Sender;

use crate::BytesChunk;

pub struct FileReader<R> {
    file: Option<R>,
    bytes_tx: Option<Sender<BytesChunk>>,
}

impl<R> FileReader<R>
where
    R: Read + Send + 'static,
{
    pub fn new(reader: R, bytes_tx: Sender<BytesChunk>) -> Self {
        FileReader {
            file: Some(reader),
            bytes_tx: Some(bytes_tx),
        }
    }

    pub fn read_file(&mut self) {
        if self.file.is_none() || self.bytes_tx.is_none() {
            eprintln!("Method already called.");
            return;
        }

        let mut file = self.file.take().unwrap();
        let sender = self.bytes_tx.take().unwrap();

        thread::spawn(move || loop {
            let mut bytes_chunk = [0; 1024];

            let chunk_len = file.read(&mut bytes_chunk).expect("Couldn't reading file.");

            if chunk_len == 0 {
                break;
            }

            sender
                .send((chunk_len, bytes_chunk))
                .expect("Couldn't send bytes chunk.");
        });
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use std::sync::mpsc;

//     #[test]
//     fn buffer_sends_correctly() {
//         let (sender, receiver) = mpsc::channel::<(usize, [u8; 1024])>();

//         let buffer = [0; 1024];

//         let mut file_reader = FileReader::new(buffer, sender);
//     }
// }
