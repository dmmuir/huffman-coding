pub type Codes = Vec<bool>;

pub fn bytes_from(bits: Codes) -> Vec<u8> {
    bits.chunks(8)
        .map(|chunk| {
            chunk
                .into_iter()
                .fold(0, |result, bit| (result << 1) ^ *bit as u8)
        })
        .collect()
}
