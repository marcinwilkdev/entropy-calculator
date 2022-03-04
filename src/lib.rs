pub mod counter_pool;
pub mod entropy_calculator;
pub mod symbols_reader;
pub mod symbols_counter;
pub mod symbols_receiver;
pub mod messages;

pub const CHUNK_SIZE: usize = 1024;

pub type Symbols = [f64; 256];
pub type SymbolsPairs = [[f64; 256]; 256]; // first index -> symbol before, second index -> symbol after

pub mod opt {
    use std::path::PathBuf;

    use structopt::StructOpt;

    #[derive(StructOpt, Debug)]
    #[structopt(name = "entropy_calculator")]
    pub struct Opt {
        #[structopt(short, long, parse(from_os_str))]
        pub file: PathBuf,
        #[structopt(short, long, default_value = "1")]
        pub threads: usize,
    }
}
