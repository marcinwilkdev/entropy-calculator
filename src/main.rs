use std::fs::File;

use structopt::StructOpt;

use entropy_calculator::entropy_calculator::EntropyCalculator;
use entropy_calculator::file_reader::FileReader;
use entropy_calculator::opt::Opt;
use entropy_calculator::messages::BytesChunk;
use entropy_calculator::counter_pool::CounterPool;

fn main() {
    let runtime = std::time::Instant::now();

    let Opt { file, threads } = Opt::from_args();

    let file = File::open(&file).expect("File doesn't exist.");

    let (bytes_tx, bytes_rx) = crossbeam_channel::bounded::<BytesChunk>(1);

    let file_reader = FileReader::new(file, bytes_tx);

    file_reader.read_file();

    let mut counter_pool = CounterPool::new(bytes_rx);

    let counted_symbols = counter_pool.count_symbols(threads);

    let mut entropy_calculator = EntropyCalculator::new(counted_symbols);

    let hx = entropy_calculator.calculate_hx();
    let hyx = entropy_calculator.calculate_hyx();

    println!("hx {}", hx);
    println!("hyx {}", hyx);
    println!("hx - hyx {}", hx - hyx);

    println!("\nprogram runtime: {:?}", runtime.elapsed());
}
