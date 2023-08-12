use std::io;
use std::io::Write;

#[derive(Debug)]
pub struct Bars {
    pub barvec: Vec<u8>,
    pub bar_unit: char,
}

impl Bars {
    // create new struct instance
    pub fn new(barvec: Vec<u8>, bar_unit: char) -> Bars {
        Bars { barvec, bar_unit }
    }

    // for displaying the musical bars
    pub fn show_bars(&self) {
        let vec_size = &self.barvec.len();
        for i in 0..*vec_size {
            let val = &self.barvec[i];
            for _ in 0..*val {
                print!("{}", self.bar_unit);
                io::stdout().flush().unwrap();
            }
            println!("");
        }
    }
}
