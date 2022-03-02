pub mod entropy_calculator;
pub mod file_reader;
pub mod symbols_counter;

pub type Probabilities = [f64; u8::MAX as usize + 1];
pub type ConditionalProbabilities = std::collections::HashMap<(u8, u8), f64>;
pub type BytesChunk = (usize, [u8; 1024]);
pub type ReadyProbabilities = (Probabilities, ConditionalProbabilities);
