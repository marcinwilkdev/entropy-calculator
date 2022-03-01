use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "lab1")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    file: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    // get command line args

    let opt = Opt::from_args();

    // get file content

    let mut file = File::open(opt.file)?;

    let mut file_content = Vec::new();

    file.read_to_end(&mut file_content)?;

    // initialize variables

    let mut probabilities = HashMap::new();
    let mut conditional_probabilities = HashMap::new();

    let mut last_symbol = 0;
    let mut symbols_count = 0.0;

    // read statistics to variables

    for &symbol in &file_content {
        let probability_entry = probabilities.entry(symbol).or_insert(0.0);
        let conditional_probability_entry = conditional_probabilities
            .entry((last_symbol, symbol))
            .or_insert(0.0);

        last_symbol = symbol;

        *probability_entry += 1.0;
        *conditional_probability_entry += 1.0;
        symbols_count += 1.0;
    }

    // calculate probabilities

    for (_, probability) in &mut probabilities {
        *probability = *probability / symbols_count;
    }

    for (_, conditional_probability) in &mut conditional_probabilities {
        *conditional_probability = *conditional_probability / symbols_count;
    }

    // calculate entropies

    let hx = -1.0
        * probabilities
            .iter()
            .fold(0.0, |sum, (_, px): (_, &f64)| sum + px * px.log2());
    
    let hyx: f64 = -1.0
        * probabilities
            .iter()
            .map(|(x1, px)| {
                px * conditional_probabilities
                    .iter()
                    .filter(|((x2, _), _)| x2 == x1)
                    .fold(0.0, |sum, (_, &pyx): (_, &f64)| sum + pyx * pyx.log2())
            })
            .sum::<f64>();

    // print entropies

    println!("hx {}", hx);
    println!("hyx {}", hyx);
    println!("hx - hyx {}", hx - hyx);

    Ok(())
}
