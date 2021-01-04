mod encode;
mod codes;

use std::collections::HashMap;
use std::fs;

use encode::huffman_codes;
use codes::Codes;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let text = fs::read_to_string(&args[1]).unwrap();
    let (tokens, freqs): (Vec<&str>, Vec<usize>) = freq_table(&text).into_iter().unzip();
    let size = tokens.len();
    let dictionary = huffman_codes(tokens, freqs, size); 

    std_out(dictionary);
}

fn std_out(dictionary: Vec<(String, Codes)>) {
    dictionary.iter().for_each(|(token, code)| {
        println!("{}: {}", token, code);
    });
}

fn freq_table<'a>(data: &'a str) -> HashMap<&'a str, usize> {
    let mut map = HashMap::with_capacity(data.len());

    for token in data.split_whitespace() {
        let count = map.entry(token).or_insert(0);
        *count += 1;
    }

    map.shrink_to_fit();
    map
}
