pub mod codes;
mod node;
mod vecdeque;

use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use codes::Codes;
use node::Node;

pub fn with_vecdeque<T>(data: &[T], freqs: Vec<usize>, size: usize) -> Vec<(T, Codes)>
where
    T: Debug + Hash + Clone,
{
    if let Some(tree) = vecdeque::from(data, freqs, size) {
        return HuffmanTree::from(tree, size).stream_codes();
    }

    Vec::new()
}

struct HuffmanTree<T> {
    id: u64,
    tree: Node<T>,
    size: usize,
}

impl<T> HuffmanTree<T>
where
    T: Hash,
{
    fn from(tree: Node<T>, size: usize) -> Self {
        let id = hash(&tree, size);
        Self { id, tree, size }
    }

    fn stream_codes<'a>(self) -> Vec<(T, Codes)>
    where
        T: Clone + Debug,
    {
        let mut array = vec![false; self.size];
        codes_from(&self.tree, &mut array, 0)
    }
}

fn hash<T: Hash>(tree: &Node<T>, size: usize) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write_usize(size);
    tree.hash(&mut hasher);
    hasher.finish()
}

fn codes_from<T>(root: &Node<T>, array: &mut Vec<bool>, top: usize) -> Vec<(T, Codes)>
where
    T: Debug + Clone,
{
    let mut dict = Vec::new();
    if let Some(left) = root.left_as_ref() {
        array[top] = false;
        dict.append(&mut codes_from(left, array, top + 1));
    }

    if let Some(right) = root.right_as_ref() {
        array[top] = true;
        dict.append(&mut codes_from(right, array, top + 1));
    }

    if let Some(data) = root.leaf().take() {
        dict.push((data.clone(), (array[..top].to_owned())))
    }

    dict
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn _vecdeque() {
        let array = vec![b'a', b'b', b'c', b'd', b'e', b'f'];
        let freq = vec![5, 9, 12, 13, 16, 45];

        let size = array.len();

        let (actual_array, codes): (Vec<u8>, Vec<Codes>) =
            with_vecdeque(&array, freq, size).into_iter().unzip();

        let expected_codes: Vec<Codes> = vec![
            vec![false],
            vec![true, false, false],
            vec![true, false, true],
            vec![true, true, false, false],
            vec![true, true, false, true],
            vec![true, true, true],
        ]
        .into_iter()
        .map(|c| Codes::from(c))
        .collect();
        let expected_array = vec![b'f', b'c', b'd', b'a', b'b', b'e'];
        assert_eq!(expected_array, actual_array);
        assert_eq!(expected_codes, codes);
    }
}
