use std::convert::TryInto;

pub type Codes = Vec<bool>;

pub fn bytes_from(bits: Codes) -> Vec<u8> {
    bits.chunks(8).map(bools_to_bits).collect()
}

fn bools_to_bits(chunk: &[bool]) -> u8 {
    let mut byte: u8 = 0;

    for (bit, value) in chunk.iter().enumerate() {
        if *value {
            byte |= 1 << bit;
        } else {
            byte &= !(1 << bit);
        }
    }

    byte
}

pub fn codes_from(bytes: Vec<u8>, _size: usize) -> Vec<bool> {
    let mut codes = Vec::with_capacity(bytes.len() * 8);

    for byte in bytes {
        for bit in &byte_to_bools(byte) {
            codes.push(*bit);
        }
    }

    codes[.._size].to_vec()
}

fn byte_to_bools(byte: u8) -> [bool; 8] {
    let mut codes = [true; 8];
    for bit_index in 0..8 {
        codes[bit_index] = (byte & (1 << bit_index)) != 0;
    }

    codes
}

pub fn usize_to_bytes(v: Vec<usize>) -> Vec<u8> {
    v.into_iter()
        .map(|u| u.to_be_bytes().to_vec())
        .flatten()
        .collect()
}

pub fn bytes_to_usize(v: &[u8]) -> Vec<usize> {
    v.chunks(8)
        .filter_map(|chunk| {
            chunk
                .try_into()
                .map(|bytes| usize::from_be_bytes(bytes))
                .ok()
        })
        .collect()
}

pub fn read_be_usize(input: &mut &[u8]) -> usize {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<usize>());
    *input = rest;
    usize::from_be_bytes(int_bytes.try_into().unwrap())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn there_and_back_again() {
        let expected = b"There and back again. A hobbits tale, by Bilbo Baggins".to_vec();
        let len = expected.len() * 8;
        let codes = codes_from(expected.clone(), len);
        let actual = bytes_from(codes);

        assert_eq!(expected, actual);
    }
}
