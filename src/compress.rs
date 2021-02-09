use std::collections::HashMap;
use std::convert::TryInto;

use super::{
    huffman_tree,
    huffman_tree::codes::{bytes_from, codes_from, Codes},
};

pub fn encode(source: &[u8]) -> Vec<u8> {
    let freq_table = freq_table(source);
    let (tokens, hits): (Vec<u8>, Vec<usize>) = sort_map(freq_table.clone()).into_iter().unzip();
    eprintln!("tokens: {:#?}, hits: {:#?}", tokens, hits);

    let tree = huffman_tree::with_vecdeque(&tokens, &hits, source.len());
    let key_pairs = match tree {
        Some(tree) => tree.stream_codes(),
        None => Vec::new(),
    };
    println!(
        "{:#?}",
        key_pairs
            .iter()
            .map(|(t, c)| (t, c.len()))
            .collect::<Vec<(&u8, usize)>>()
    );
    let size = calculate_compression_size(freq_table, &key_pairs);
    println!(
        "Tokens len: {}, Hits len: {}, Size: {}",
        tokens.len(),
        hits.len() * 8,
        size
    );
    println!("{:#?}", key_pairs);
    let lengths = usize_to_bytes(vec![tokens.len(), hits.len() * 8, size]);
    let buffer = swap_codes(source, key_pairs, size);
    eprintln!("{:#?}", buffer);
    let hits_as_bytes = usize_to_bytes(hits);

    vec![lengths, tokens, hits_as_bytes, buffer].concat()
}

pub fn decode(source: &[u8]) -> Vec<u8> {
    let mut source = source;
    let tokens_len = read_be_usize(&mut source);
    let hits_len = read_be_usize(&mut source);
    let compression_size = read_be_usize(&mut source);
    println!(
        "Tokens len: {}, Hits len: {}, Size: {}",
        tokens_len, hits_len, compression_size
    );

    let tokens = source[..tokens_len].to_vec();
    let hits = bytes_to_usize(&source[tokens_len..tokens_len + hits_len]);
    eprintln!("tokens: {:#?}, hits: {:#?}", tokens, hits);
    let size = hits.iter().sum::<usize>();
    let tree = huffman_tree::with_vecdeque(&tokens, &hits, size);
    println!(
        "{:#?}",
        huffman_tree::with_vecdeque(&tokens, &hits, size)
            .unwrap()
            .stream_codes()
    );
    let compressed_source = source[tokens_len + hits_len..].to_vec();
    eprintln!("{:#?}", compressed_source);
    let codes = codes_from(compressed_source, compression_size);

    match tree {
        Some(tree) => tree.read(codes),
        None => Vec::new(),
    }
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
    v.chunks(8)
        .filter_map(|chunk| {
            chunk
                .try_into()
                .map(|bytes| usize::from_be_bytes(bytes))
                .ok()
        })
        .collect()
}

fn read_be_usize(input: &mut &[u8]) -> usize {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<usize>());
    *input = rest;
    usize::from_be_bytes(int_bytes.try_into().unwrap())
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
