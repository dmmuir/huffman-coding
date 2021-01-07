mod huffman_tree;

use std::collections::{HashMap};
use std::fs;
use std::hash::Hash;

use huffman_tree::codes::{Codes, bytes_from};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let bytes: Vec<u8> = fs::read(&args[2]).unwrap();
    let size = bytes.len();
    let (tokens, freqs): (Vec<u8>, Vec<usize>) = freq_table(&bytes).into_iter().unzip();
    let token_set: Vec<u8>;
    let dictionary: Vec<(&&u8, Codes)> = match args[1].as_str() {
        "sort" => {
            let mut sorted_dict: Vec<(u8, usize)> =
                tokens.into_iter().zip(freqs.into_iter()).collect();
            sorted_dict.sort_by_key(|record| record.1);
            let (ts, freqs): (Vec<u8>, Vec<usize>) = sorted_dict.into_iter().unzip();
            token_set = ts;
            huffman_tree::with_vecdeque(&token_set, freqs, size)
        },
        "sorted" => huffman_tree::with_vecdeque(&tokens, freqs, size),
        _ => huffman_tree::with_min_heap(&tokens, freqs, size),
    };

    //std_out(dictionary);
    write_codes(dictionary, &bytes);
}

fn std_out(dictionary: Vec<(String, Codes)>) {
    let bytes: Vec<(String, Vec<u8>)> = dictionary
        .into_iter()
        .map(|(token, code)| (token, bytes_from(code)))
        .collect();
    println!("{:?}", bytes)
}

fn word_tokens_from<'a>(text: &'a str) -> Vec<&'a str> {
    text.split_whitespace().collect()
}

fn write_codes<T: Hash + Eq>(dict: Vec<(&&T, Codes)>, tokens: &[T]) {
    let map: HashMap<&&T, Codes> = dict.into_iter().collect();
    let mut buffer = Vec::with_capacity(tokens.len());
    for token in tokens {
        if let Some(code) = map.get(&token) {
            buffer.push(bytes_from(code.to_owned()));
        }
    }

    let args: Vec<String> = std::env::args().collect();
    let buffer: Vec<u8> = buffer.into_iter().flatten().collect();
    fs::write(&args[3], buffer).unwrap();
}

fn freq_table<'a, T>(data: &'a [T]) -> HashMap<&'a T, usize>
where
    T: std::hash::Hash + Eq + Sized,
{
    let mut map = HashMap::with_capacity(data.len());

    for token in data {
        let count = map.entry(token).or_insert(0);
        *count += 1;
    }

    map.shrink_to_fit();
    map
}
