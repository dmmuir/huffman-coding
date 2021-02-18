use std::collections::HashMap;

use super::huffman_tree::tree;
use crate::bytes::{bytes_to_usize, read_be_usize};

use prettytable::Table;

pub fn print(source: &[u8]) {
    let mut source = source;
    let tokens_len = read_be_usize(&mut source);
    let hits_len = tokens_len * 8;
    let compression_size = read_be_usize(&mut source);

    let tokens = &source[..tokens_len];
    let hits = bytes_to_usize(&source[tokens_len..tokens_len + hits_len]);
    let hits_map: HashMap<&u8, &usize> = tokens.iter().zip(hits.iter()).collect();
    let size = hits.iter().sum::<usize>();
    let tree = tree::with_vecdeque(tokens, &hits, size).unwrap();
    let key_pairs = tree.stream_codes();

    let compression_total = compression_size / 8 + tokens_len + hits_len;
    let compression_percent = (1.0 - compression_total as f64 / size as f64) * 100.0;
    println!(
        "Compression ratio: {}/{}; {:.2}%",
        compression_total, size, compression_percent
    );
    println!("Dictionary stats:");
    println!("Tokens:\t{}", tokens_len);
    println!("Hits size:\t{}", hits_len);
    println!("Total bytes:\t{}", tokens_len + hits_len);

    println!("Dictionary contents:");
    let mut rows = Vec::with_capacity(tokens_len);
    for (t, codes) in key_pairs {
        let binary_codes: String = codes
            .into_iter()
            .map(|c| if c { '0' } else { '1' })
            .collect();
        let hit = hits_map.get(&t).unwrap();
        rows.push((t, **hit, binary_codes));
    }

    rows.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    print_table(rows);
}

fn print_table(row: Vec<(u8, usize, String)>) {
    let mut table = Table::new();
    table.add_row(row!["#", "Character", "Count", "Code", "Bits"]);

    for (i, (t, x, c)) in row.into_iter().enumerate() {
        table.add_row(row![
            i,
            format!(
                "{:?}",
                String::from_utf8(vec![t]).unwrap_or_else(|t| format!("{}", t))
            ),
            x,
            c,
            c.len(),
        ]);
    }

    table.printstd();
}
