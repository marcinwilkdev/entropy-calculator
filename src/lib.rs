pub mod entropy_calculator;
pub mod file_reader;
pub mod probabilities_calculator;
pub mod symbols_counter;

pub const CHUNK_SIZE: usize = 1024;

pub type Probabilities = [f64; u8::MAX as usize + 1];
pub type ConditionalProbabilities = std::collections::HashMap<(u8, u8), f64>;
pub type BytesChunk = (usize, [u8; CHUNK_SIZE]);
pub type ReadyProbabilities = (Probabilities, ConditionalProbabilities);

pub mod opt {
    use std::path::PathBuf;

    use structopt::StructOpt;

    #[derive(StructOpt, Debug)]
    #[structopt(name = "lab1")]
    pub struct Opt {
        #[structopt(short, long, parse(from_os_str))]
        pub file: PathBuf,
    }
}
