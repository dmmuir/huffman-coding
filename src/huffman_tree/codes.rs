pub type Codes = Vec<bool>;

pub fn bytes_from(bits: Codes) -> Vec<u8> {
    bits.chunks(8)
        .map(|chunk| {
            chunk
                .into_iter()
                .fold(0, |result, bit| (result << 1) ^ *bit as u8)
        })
        .map(|int| int ^ 255)
        .collect()
}

pub fn codes_from(bytes: Vec<u8>, size: usize) -> Vec<bool> {
    let mut codes = Vec::with_capacity(bytes.len() * 8);

    for mut byte in bytes {
        for _ in 0..8 {
            codes.push(byte.trailing_zeros() > 0);
            byte = byte.rotate_right(1);
        }
    }

    codes[..size].to_vec()
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn all_zeros() {
        let expected_bytes = vec![0; 100];
        let actual_bytes = bytes_from(codes_from(expected_bytes.clone(), 100*8));

        assert_eq!(expected_bytes, actual_bytes);
    }

    #[test]
    fn all_ones() {
        let expected_bytes = vec![255; 100];
        let actual_bytes = bytes_from(codes_from(expected_bytes.clone(), 100*8));

        assert_eq!(expected_bytes, actual_bytes);
    }

    #[test]
    fn all_true() {
        let expected_bytes = vec![true; 100];
        let actual_bytes = codes_from(bytes_from(expected_bytes.clone()), 100);

        assert_eq!(expected_bytes, actual_bytes);
    }

    #[test]
    fn all_false() {
        let expected_bytes = vec![false; 100];
        let actual_bytes = codes_from(bytes_from(expected_bytes.clone()), 100);

        assert_eq!(expected_bytes, actual_bytes);
    }

    #[test]
    fn five_bit_code_trues() {
        let expected_bits = 0b00000000;
        let actual_bits = bytes_from(vec![true; 5])[0]; 

        assert_eq!(expected_bits, actual_bits);
    }
    
    #[test]
    fn sixty_bit_code_trues() {
        let expected_bits = vec![0b00000000, 0]; 
        let actual_bits = bytes_from(vec![true; 12]); 

        assert_eq!(expected_bits, actual_bits);
    }
    
    #[test]
    fn five_bit_code() {
        let expected_bits = 0b00011111;
        let actual_bits = bytes_from(vec![false; 5])[0]; 

        assert_eq!(expected_bits, actual_bits);
    }
    
    #[test]
    fn sixty_bit_code() {
        let expected_bits = vec![0b00001111, 255]; 
        let actual_bits = bytes_from(vec![false; 12]); 

        assert_eq!(expected_bits, actual_bits);
    }
}

