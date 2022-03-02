use std::fs::File;
use std::path::PathBuf;

use structopt::StructOpt;

use lab1::entropy_calculator::EntropyCalculator;
use lab1::file_reader::FileReader;
use lab1::symbols_counter::SymbolsCounter;
use lab1::{BytesChunk, ReadyProbabilities};

#[derive(StructOpt, Debug)]
#[structopt(name = "lab1")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let file = File::open(&opt.file).expect("File doesn't exist.");

    let (bytes_tx, bytes_rx) = crossbeam_channel::unbounded::<BytesChunk>();

    let mut file_reader = FileReader::new(file, bytes_tx);

    file_reader.read_file();

    let (probs_tx, probs_rx) = crossbeam_channel::unbounded::<ReadyProbabilities>();

    let mut symbols_counter = SymbolsCounter::new(bytes_rx, probs_tx);

    symbols_counter.count_symbols();

    let (probs, cond_probs) = probs_rx
        .recv()
        .expect("Couldn't fetch ready probabilities.");

    let entropy_calculator = EntropyCalculator::new(probs, cond_probs);

    let hx = entropy_calculator.calculate_hx();
    let hyx = entropy_calculator.calculate_hyx();

    println!("hx {}", hx);
    println!("hyx {}", hyx);
    println!("hx - hyx {}", hx - hyx);
}
