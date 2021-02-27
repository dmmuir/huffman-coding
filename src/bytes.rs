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

pub fn codes_from(bytes: &[u8], _size: usize) -> Vec<bool> {
    let mut codes = Vec::with_capacity(bytes.len() * 8);

    for byte in bytes {
        for bit in &byte_to_bools(*byte) {
            codes.push(*bit);
        }
    }

    codes[.._size].to_vec()
}

fn byte_to_bools(byte: u8) -> [bool; 8] {
    let mut codes = [true; 8];
    for (bit_index, bit) in codes.iter_mut().enumerate() {
        *bit = (byte & (1 << bit_index)) != 0;
    }

    codes
}

pub fn usize_to_bytes(v: Vec<usize>) -> Vec<u8> {
    v.into_iter()
        .map(|u| u.to_be_bytes().to_vec())
        .flatten()
        .collect()
}

pub fn usize_to_smallest_bytes(v: Vec<usize>) -> (u8, Vec<u8>) {
    let byte_size = smallest_byte_representation(&v);
    let bytes = v
        .into_iter()
        .map(|u| match byte_size {
            8 => (u as u8).to_be_bytes().to_vec(),
            16 => (u as u16).to_be_bytes().to_vec(),
            32 => (u as u32).to_be_bytes().to_vec(),
            _ => u.to_be_bytes().to_vec(),
        })
        .flatten()
        .collect();

    (byte_size, bytes)
}

fn smallest_byte_representation(v: &[usize]) -> u8 {
    let max = *v.iter().max().unwrap_or(&0);

    if max <= u16::MAX as usize {
        if max <= u8::MAX as usize {
            8
        } else {
            16
        }
    } else if max <= u32::MAX as usize {
        32
    } else {
        64
    }
}

pub fn bytes_to_usize(bit_count: usize, v: &[u8]) -> Vec<usize> {
    let byte_size = bit_count / 8;
    v.chunks(byte_size)
        .map(|chunk| {
            let mut buffer = vec![0u8; 8 - byte_size];
            for b in chunk {
                buffer.push(*b)
            }
            buffer
        })
        .filter_map(|chunk| chunk.try_into().map(usize::from_be_bytes).ok())
        .collect()
}

pub fn read_be_usize(input: &[u8]) -> (usize, &[u8]) {
    let (int_bytes, remaining) = input.split_at(std::mem::size_of::<usize>());
    (
        usize::from_be_bytes(int_bytes.try_into().unwrap()),
        remaining,
    )
}

pub fn read_be_u8(input: &[u8]) -> (usize, &[u8]) {
    let (int_bytes, remaining) = input.split_at(std::mem::size_of::<u8>());
    (
        usize::from_be_bytes([0, 0, 0, 0, 0, 0, 0, int_bytes[0]]),
        remaining,
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn there_and_back_again() {
        let expected = b"There and back again. A hobbits tale, by Bilbo Baggins";
        let len = expected.len() * 8;
        let codes = codes_from(expected, len);
        let actual = bytes_from(codes);

        assert_eq!(expected, actual.as_slice());
    }
}
