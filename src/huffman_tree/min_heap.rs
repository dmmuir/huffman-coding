//! # Hoffman (en)Coding
//! 1. Build a Huffman Tree from input characters
//! 2. Traverse the Huffman Tree and assign codes to characters.
//!     1. Create leaf node for each unique character and build a min heap of all leaf nodes.

use super::node::{Node, T};
use std::fmt::{Debug, Display};

#[derive(Debug)]
struct MinHeap {
    array: Vec<Node<T>>,
}

impl MinHeap
where
    T: Display + Debug
{
    fn with_capacity(size: usize) -> Self {
        Self {
            array: Vec::with_capacity(size),
        }
    }

    fn swap_nodes(&mut self, a: usize, b: usize) {
        self.array.swap(a, b);
    }

    fn min_heapify(&mut self, index: usize) {
        let mut smallest = index;
        let left = 2 * index + 1;
        let right = 2 * index + 2;

        if left < self.size() && self.array[left].freq < self.array[smallest].freq {
            smallest = left;
        }

        if right < self.size() && self.array[right].freq < self.array[smallest].freq {
            smallest = right;
        }

        if smallest != index {
            self.swap_nodes(smallest, index);
            self.min_heapify(smallest);
        }
    }

    fn size(&self) -> usize {
        self.array.len()
    }

    fn freq_at(&self, index: usize) -> usize {
        self.array[index].freq
    }

    fn is_size_one(&self) -> bool {
        self.size() == 1
    }

    fn extract_min(&mut self) -> Option<Node<T>> {
        if self.array.is_empty() {
            return None;
        }
        self.swap_nodes(0, self.size() - 1);
        let node = self.array.pop();
        self.min_heapify(0);

        node
    }

    fn insert_min_heap(&mut self, node: Node<T>) {
        let node_freq = node.freq;
        self.array.push(node);
        if self.array.is_empty() {
            return;
        }
        let mut i = self.size() - 1;

        while i > 0 && node_freq < self.freq_at((i - 1) / 2) {
            self.swap_nodes(i, (i - 1) / 2);
            i = (i - 1) / 2;
        }
    }
}

pub fn from(data: &[T], freqs: Vec<usize>, size: usize) -> Option<Node<T>>
where
    T: Display + Debug,
{
    let mut heap = MinHeap::with_capacity(size);

    data.into_iter().zip(freqs.iter()).for_each(|(d, f)| {
        heap.array.push(Node::new_leaf(*d, *f));
    });

    while !heap.is_size_one() && !heap.array.is_empty() {
        let mut left = heap.extract_min();
        let mut right = heap.extract_min();

        let top_freq = left.as_ref().map(|l| l.freq).unwrap_or_default()
            + right.as_ref().map(|r| r.freq).unwrap_or_default();
        let mut top = Node::new_branch(top_freq);

        if let Some(node) = left.take() {
            top.left(Box::new(node));
        }

        if let Some(node) = right.take() {
            top.right(Box::new(node));
        }

        heap.insert_min_heap(top);
    }

    heap.extract_min()
}

#[cfg(foo)]
mod test {
    use super::super::codes::Codes;
    use super::*;

    //#[test]
    fn _1() {
        let array = vec!['a', 'b', 'c', 'd', 'e', 'f'];
        let freq = vec![5, 9, 12, 13, 16, 45];

        let size = array.len();

        let (actual_array, codes): (Vec<String>, Vec<Codes>) =
            from(array, freq, size).into_iter().unzip();

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
        let expected_array = vec!["f", "c", "d", "a", "b", "e"];
        assert_eq!(expected_array, actual_array);
        assert_eq!(expected_codes, codes);
    }
}
