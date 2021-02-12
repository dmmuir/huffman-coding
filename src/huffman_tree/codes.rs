pub type Codes = Vec<bool>;

pub fn bytes_from(bits: Codes) -> Vec<u8> {
    bits.chunks(8).map(bools_to_bits).collect()
}

fn bools_to_bits(chunk: &[bool]) -> u8 {
    let mut byte: u8 = 0;
    chunk.iter().enumerate().for_each(|(bit, value)| {
        if *value {
            byte |= 1 << bit;
        } else {
            byte &= !(1 << bit);
        }
    });

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
