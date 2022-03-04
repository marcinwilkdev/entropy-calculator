use std::fs::File;

use structopt::StructOpt;

use entropy_calculator::entropy_calculator::EntropyCalculator;
use entropy_calculator::file_reader::FileReader;
use entropy_calculator::opt::Opt;
use entropy_calculator::messages::{BytesChunk, CountedSymbols};
use entropy_calculator::counter_pool::CounterPool;

fn main() {
    let runtime = std::time::Instant::now();

    let opt = Opt::from_args();

    let file = File::open(&opt.file).expect("File doesn't exist.");

    let (bytes_tx, bytes_rx) = crossbeam_channel::bounded::<BytesChunk>(1);

    let file_reader = FileReader::new(file, bytes_tx);

    file_reader.read_file();

    let mut counter_pool = CounterPool::new(bytes_rx);

    let CountedSymbols {
        symbols,
        cond_symbols,
        count,
    } = counter_pool.count_symbols(2);

    let mut entropy_calculator = EntropyCalculator::new(symbols, cond_symbols, count);

    let hx = entropy_calculator.calculate_hx();
    let hyx = entropy_calculator.calculate_hyx();

    println!("hx {}", hx);
    println!("hyx {}", hyx);
    println!("hx - hyx {}", hx - hyx);

    println!("\nprogram runtime: {:?}", runtime.elapsed());
}
