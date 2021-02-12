pub mod codes;
mod node;
mod vecdeque;

use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use codes::Codes;
use node::Node;

pub fn with_vecdeque<T>(data: &[T], freqs: &[usize], size: usize) -> Option<HuffmanTree<T>>
where
    T: Debug + Hash + Clone,
{
    if let Some(tree) = vecdeque::from(data, freqs, size) {
        return Some(HuffmanTree::from(tree, size));
    }

    None
}

pub struct HuffmanTree<T> {
    _id: u64,
    tree: Node<T>,
    size: usize,
}

impl<T> HuffmanTree<T>
where
    T: Debug + Hash + Clone,
{
    pub fn from(tree: Node<T>, size: usize) -> Self {
        let _id = hash(&tree, size);
        Self { _id, tree, size }
    }

    pub fn read(&self, codes: Codes) -> Vec<T> {
        let mut file = Vec::with_capacity(self.size);
        let mut curr = &self.tree;

        for code in codes {
            if code {
                if let Some(left) = curr.left_as_ref() {
                    curr = left;
                }
            } else {
                if let Some(right) = curr.right_as_ref() {
                    curr = right;
                }
            }

            if let Some(token) = curr.leaf() {
                file.push(token.clone());
                curr = &self.tree;
            }
        }

        file
    }

    pub fn stream_codes<'a>(self) -> Vec<(T, Codes)>
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
        array[top] = true;
        dict.append(&mut codes_from(left, array, top + 1));
    }

    if let Some(right) = root.right_as_ref() {
        array[top] = false;
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

        let (actual_array, codes): (Vec<u8>, Vec<Codes>) = with_vecdeque(&array, &freq, size)
            .unwrap()
            .stream_codes()
            .into_iter()
            .unzip();

        let expected_codes: Vec<Codes> = vec![
            vec![true],
            vec![false, true, true],
            vec![false, true, false],
            vec![false, false, true, true],
            vec![false, false, true, false],
            vec![false, false, false],
        ];

        let expected_array = vec![b'f', b'c', b'd', b'a', b'b', b'e'];
        assert_eq!(expected_array, actual_array);
        assert_eq!(expected_codes, codes);
    }
}
