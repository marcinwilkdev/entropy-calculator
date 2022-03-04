pub mod counter_pool;
pub mod entropy_calculator;
pub mod file_reader;
pub mod symbols_counter;
pub mod symbols_receiver;
pub mod messages;

pub const CHUNK_SIZE: usize = 1024;

pub type Probabilities = [f64; u8::MAX as usize + 1];
pub type ConditionalProbabilities = [[f64; 256]; 256];

pub mod opt {
    use std::path::PathBuf;

    use structopt::StructOpt;

    #[derive(StructOpt, Debug)]
    #[structopt(name = "entropy_calculator")]
    pub struct Opt {
        #[structopt(short, long, parse(from_os_str))]
        pub file: PathBuf,
    }
}
