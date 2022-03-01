use std::io::Read;
use std::sync::mpsc::Sender;
use std::thread;

pub struct FileReader<T> {
    file: Option<T>,
    sender: Option<Sender<(usize, [u8; 1024])>>,
}

impl<T> FileReader<T>
where
    T: Read + Send + 'static,
{
    pub fn new(reader: T, sender: Sender<(usize, [u8; 1024])>) -> Self {
        FileReader {
            file: Some(reader),
            sender: Some(sender),
        }
    }

    pub fn read_file(&mut self) {
        if self.file.is_none() || self.sender.is_none() {
            eprintln!("Method already called");
            return;
        }

        let mut file = self.file.take().unwrap();
        let sender = self.sender.take().unwrap();

        thread::spawn(move || loop {
            let mut buffer = [0; 1024];

            let read_len = file.read(&mut buffer).expect("Error reading file");

            if read_len == 0 {
                break;
            }

            sender
                .send((read_len, buffer))
                .expect("Couldn't send value");
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
