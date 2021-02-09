#[macro_use]
extern crate clap;

mod cli;
mod compress;
mod huffman_tree;

use std::fs;

use compress::{decode, encode};

fn main() {
    let matches = cli::app();

    let input_file = matches.value_of("filepath").unwrap();
    let source = fs::read(input_file).unwrap();
    let filename: String;

    let result = if matches.is_present("decode") {
        filename = input_file.replace(".huff", "");
        decode(&source)
    } else {
        filename = format!("{}.huff", input_file);
        encode(&source)
    };

    fs::write(filename, result).unwrap();
}
