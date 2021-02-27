use crate::bytes::{bytes_to_usize, read_be_u8, read_be_usize};

pub fn read_dictionary(source: &[u8]) -> (Vec<u8>, Vec<usize>) {
    let (tokens_len, remaining) = read_be_usize(source);
    let (_compression_size, remaining) = read_be_usize(remaining);
    let (byte_size, remaining) = read_be_u8(remaining);
    let hits_len = tokens_len * (byte_size / 8);

    let tokens = remaining[..tokens_len].to_vec();
    let hits = bytes_to_usize(byte_size, &remaining[tokens_len..tokens_len + hits_len]);

    (tokens, hits)
}

pub fn read_sizes(source: &[u8]) -> (usize, usize, &[u8]) {
    let (tokens_len, remaining) = read_be_usize(&source);
    let (size_when_compressed, remaining) = read_be_usize(&remaining);
    let (byte_size, remaining) = read_be_u8(&remaining);
    let dictionary_size = tokens_len + tokens_len * (byte_size / 8);

    (dictionary_size, size_when_compressed, &remaining)
}
