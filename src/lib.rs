pub mod file_reader;
pub mod symbols_counter;
pub mod gluer;

pub type FirstMessage = (usize, [u8; 1024]);
pub type SecondMessage = ([f64; u8::MAX as usize + 1], std::collections::HashMap<(u8, u8), f64>);
