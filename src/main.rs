use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::sync::mpsc;

use structopt::StructOpt;

use lab1::file_reader::FileReader;
use lab1::symbols_counter::SymbolsCounter;
use lab1::entropy_calculator::EntropyCalculator;
use lab1::{FirstMessage, SecondMessage};

#[derive(StructOpt, Debug)]
#[structopt(name = "lab1")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    file: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let file = File::open(&opt.file).expect("File doesn't exist");

    let (first_sender, first_receiver) = mpsc::channel::<FirstMessage>();

    let mut file_reader = FileReader::new(file, first_sender);

    file_reader.read_file();

    let (second_sender, second_receiver) = mpsc::channel::<SecondMessage>();

    let mut symbols_counter = SymbolsCounter::new(first_receiver, second_sender);

    symbols_counter.count_symbols();

    let (probabilities, conditional_probabilities) = second_receiver.recv()?;

    let entropy_calculator = EntropyCalculator::new(probabilities, conditional_probabilities);

    let hx = entropy_calculator.calculate_hx();
    let hyx = entropy_calculator.calculate_hyx();

    println!("hx {}", hx);
    println!("hyx {}", hyx);
    println!("hx - hyx {}", hx - hyx);

    Ok(())
}
