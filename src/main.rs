#[macro_use]
extern crate clap;
#[macro_use]
extern crate prettytable;

mod bytes;
mod cli;
mod compress;
mod format;
mod huffman_tree;
mod stats;

use std::fs;

use compress::{decode, encode};

fn main() {
    let matches = cli::app();

    let input_file = matches.value_of("filepath").unwrap();
    let source = fs::read(input_file).unwrap();
    let mut filename = String::new();

    let result = if matches.is_present("decode") {
        filename = input_file.replace(".huff", "");
        decode(&source)
    } else if matches.is_present("stats") {
        stats::print(&source);
        Vec::new()
    } else {
        filename = format!("{}.huff", input_file);
        encode(&source)
    };

    if !result.is_empty() {
        fs::write(filename, result).unwrap();
    }
}
