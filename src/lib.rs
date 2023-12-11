use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader};

pub mod day1;

pub fn get_reader() -> Box<dyn BufRead> {
    match env::args().nth(1) {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap())),
    }
}

pub fn get_lines() -> Box<dyn Iterator<Item = String>> {
    Box::new(get_reader().lines().map(|l| l.unwrap()))
}
