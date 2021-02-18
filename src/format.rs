use crate::bytes::{bytes_to_usize, read_be_usize};

pub fn read_dictionary(source: &[u8]) -> (Vec<u8>, Vec<usize>) {
    let (tokens_len, remaining) = read_be_usize(source);
    let hits_len = tokens_len * 8;
    let (_compression_size, remaining) = read_be_usize(remaining);

    let tokens = remaining[..tokens_len].to_vec();
    let hits = bytes_to_usize(&remaining[tokens_len..tokens_len + hits_len]);

    (tokens, hits)
}

pub fn read_sizes(source: &[u8]) -> (usize, usize, &[u8]) {
    let (tokens_len, remaining) = read_be_usize(&source);
    let dictionary_size = tokens_len + tokens_len * 8;
    let (size_when_compressed, remaining) = read_be_usize(&remaining);

    (dictionary_size, size_when_compressed, remaining)
}
