use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle};

pub struct FileReader {
    file: Option<File>,
    sender: Option<Sender<(usize, [u8; 1024])>>,
    join_handle: Option<JoinHandle<()>>,
}

impl FileReader {
    pub fn new(file_path: &Path, sender: Sender<(usize, [u8; 1024])>) -> FileReader {
        let file = File::open(file_path).expect("File doesn'st exist.");

        FileReader {
            file: Some(file),
            sender: Some(sender),
            join_handle: None,
        }
    }

    pub fn read_file(&mut self) {
        if self.file.is_none() || self.sender.is_none() {
            eprintln!("Method already called");
            return;
        }

        let mut file = self.file.take().unwrap();
        let sender = self.sender.take().unwrap();

        let join_handle = thread::spawn(move || loop {
            let mut buffer = [0; 1024];

            let read_len = file.read(&mut buffer).expect("Error reading file");

            if read_len == 0 {
                break;
            }

            sender.send((read_len, buffer)).expect("Couldn't send value");
        });

        self.join_handle = Some(join_handle);
    }

    pub fn wait_for_read(&mut self) {
        if self.join_handle.is_none() {
            eprintln!("You have to call read_file first");
            return;
        }

        let join_handle = self.join_handle.take().unwrap();

        if let Err(_) = join_handle.join() {
            eprintln!("Couldn't join thread");
        }
    }
}
