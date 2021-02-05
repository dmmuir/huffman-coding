use std::collections::HashMap;

use super::{
    huffman_tree,
    huffman_tree::codes::{codes_from, bytes_from, Codes},
};

pub fn encode(source: &[u8]) -> Vec<u8> {
    let freq_table = freq_table(source);
    let (tokens, hits): (Vec<u8>, Vec<usize>) = sort_map(freq_table.clone()).into_iter().unzip();
    let key_pairs = huffman_tree::with_vecdeque(&tokens, &hits, source.len());
    let size = calculate_compression_size(freq_table, &key_pairs);
    let buffer = swap_codes(source, key_pairs, size);
    let hits_as_bytes = usize_to_bytes(hits);

    vec![tokens, hits_as_bytes, buffer].concat()
}

pub fn decode(source: &[u8]) -> Vec<u8> {
    let token_len = 0;
    let hits_len = 0;
    let tokens = source[..token_len].to_vec();
    let hits = bytes_to_usize(&source[token_len..hits_len]);
    let size = hits.iter().sum::<usize>();
    let key_pairs = huffman_tree::with_vecdeque(&tokens, &hits, size);
    let compressed_source = source[token_len + hits_len..].to_vec();

    let codes = codes_from(compressed_source);
    vec![]
}

fn freq_table(data: &[u8]) -> HashMap<u8, usize> {
    let mut map = HashMap::with_capacity(data.len());

    for token in data {
        let count = map.entry(*token).or_insert(0);
        *count += 1;
    }

    map.shrink_to_fit();
    map
}

fn sort_map(map: HashMap<u8, usize>) -> Vec<(u8, usize)> {
    let mut table: Vec<(u8, usize)> = map.into_iter().collect();
    table.sort_by_key(|record| record.1);
    table
}

fn swap_codes(source: &[u8], key_pairs: Vec<(u8, Codes)>, size: usize) -> Vec<u8> {
    let key_map: HashMap<u8, Codes> = key_pairs.into_iter().collect();
    let mut buffer = Vec::with_capacity(size);

    for t in source {
        if let Some(code) = key_map.get(t) {
            code.iter().for_each(|bit| buffer.push(*bit));
        }
    }

    bytes_from(buffer)
}

fn usize_to_bytes(v: Vec<usize>) -> Vec<u8> {
    v.into_iter()
        .map(|u| u.to_be_bytes().to_vec())
        .flatten()
        .collect()
}

fn bytes_to_usize(v: &[u8]) -> Vec<usize> {
    use std::convert::TryInto;
    v.chunks(8)
        .map(|chunk| usize::from_be_bytes(chunk.try_into().unwrap()))
        .collect()
}

fn calculate_compression_size(
    freq_table: HashMap<u8, usize>,
    key_pairs: &[(u8, Vec<bool>)],
) -> usize {
    key_pairs
        .iter()
        .filter_map(|(k, codes)| freq_table.get(k).map(|count| count * codes.len()))
        .sum::<usize>()
}
