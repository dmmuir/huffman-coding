use std::collections::HashMap;

use super::huffman_tree::tree;
use crate::bytes::{bytes_from, codes_from, usize_to_bytes, usize_to_smallest_bytes, Codes};
use crate::format::{read_dictionary, read_sizes};

pub fn encode(source: &[u8]) -> Vec<u8> {
    let freq_table = freq_table(source);
    let (tokens, hits): (Vec<u8>, Vec<usize>) = sort_map(freq_table.clone()).into_iter().unzip();

    let tree = tree::with_vecdeque(&tokens, &hits, source.len());
    let key_pairs = match tree {
        Some(tree) => tree.stream_codes(),
        None => Vec::new(),
    };

    let size_when_compressed = calculate_compression_size(freq_table, &key_pairs);
    let buffer = swap_codes(source, key_pairs, size_when_compressed);
    let lengths = usize_to_bytes(vec![tokens.len(), size_when_compressed]);
    let (byte_size, hits_as_bytes) = usize_to_smallest_bytes(hits);

    vec![lengths, vec![byte_size], tokens, hits_as_bytes, buffer].concat()
}

pub fn decode(source: &[u8]) -> Vec<u8> {
    let (tokens, hits) = read_dictionary(source);
    let (dictionary_size, size_when_compressed, remaining_source) = read_sizes(source);

    let size = hits.iter().sum::<usize>();
    let tree = tree::with_vecdeque(&tokens, &hits, size);
    let compressed_source = &remaining_source[dictionary_size..];
    let codes = codes_from(compressed_source, size_when_compressed);

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
    table.sort_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(&a.0)));

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

fn calculate_compression_size(
    freq_table: HashMap<u8, usize>,
    key_pairs: &[(u8, Vec<bool>)],
) -> usize {
    key_pairs
        .iter()
        .filter_map(|(k, codes)| freq_table.get(k).map(|count| count * codes.len()))
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn abcde() {
        let input = b"aaaaaabccccccddeeeee";
        let codes = encode(input);
        let decode = decode(&codes);

        assert_eq!(&input[..], decode);
    }

    #[test]
    fn geeksforgeeks() {
        let input = b"geeksforgeeks";
        let codes = encode(input);
        let decode = decode(&codes);

        assert_eq!(&input[..], decode);
    }
}
