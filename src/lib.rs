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

use entropy_calculator::EntropyCalculator;
use counter_pool::CounterPool;
use std::fs::File;
use std::path::Path;
use symbols_reader::SymbolsReader;
use messages::BytesChunk;

pub fn show_file_entropy(filename: &Path) {
    let file = File::open(filename).expect("File doesn't exist.");

    let (bytes_tx, bytes_rx) = crossbeam_channel::bounded::<BytesChunk>(1);

    SymbolsReader::new(file, bytes_tx).read_symbols();

    let mut counter_pool = CounterPool::new(bytes_rx);

    let counted_symbols = counter_pool.count_symbols(1);

    let mut entropy_calculator = EntropyCalculator::new(counted_symbols);

    let hx = entropy_calculator.calculate_hx();

    println!("Source entropy: {}", hx);
}
