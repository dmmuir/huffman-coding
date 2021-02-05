#[macro_use]
extern crate clap;

mod cli;
mod compress;
mod huffman_tree;

use std::fs;

use compress::encode;

fn main() {
    let matches = cli::app();

    let input_file = matches.value_of("filepath").unwrap();
    let source = fs::read(input_file).unwrap();
    let action = matches.value_of("decode");

    let result = match action {
        None => encode(&source),
        Some(_) => vec![],
    };

    fs::write(format!("{}.huff", input_file), result).unwrap();
}
