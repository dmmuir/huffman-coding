//! # Hoffman (en)Coding
//! 1. Build a Huffman Tree from input characters
//! 2. Traverse the Huffman Tree and assign codes to characters.
//!     1. Create leaf node for each unique character and build a min heap of all leaf nodes.

use std::{fmt::{Debug, Display}};
use crate::codes::Codes;

type BoxOption<T> = Option<Box<T>>;

#[derive(Debug)]
enum NodeType<T> {
    Leaf(T),
    Branch { left: BoxOption<MinHeapNode<T>>, right: BoxOption<MinHeapNode<T>> }
}

impl<T> NodeType<T> {
    fn set_leaf(&mut self, data: T) {
        if let Self::Leaf(ref mut t) = *self {
           *t = data;
        }
    }
    
    fn leaf(&self) -> Option<&T> {
        match self {
            Self::Leaf(t) => Some(t),
            _ => None,
        }
    }

    fn left(&mut self, node: Box<MinHeapNode<T>>) {
        if let Self::Branch { ref mut left, right: _ } = *self {
            *left = Some(node);
        }
    }

    fn right(&mut self, node: Box<MinHeapNode<T>>) {
        if let Self::Branch { left: _, ref mut right } = *self {
            *right = Some(node);
        }
    }

    fn left_as_mut(&mut self) -> Option<&mut MinHeapNode<T>> {
        if let Self::Branch { ref mut left , right: _ } = *self {
             if let Some(ref mut node ) = *left {
                let node = node.as_mut();
                return Some(node)
            }
        }

        None
    }

    fn right_as_mut(&mut self) -> Option<&mut MinHeapNode<T>> {
        if let Self::Branch { left: _ , ref mut right } = *self {
             if let Some(ref mut node ) = *right {
                let node = node.as_mut();
                return Some(node)
            }
        }

        None
    }
}

#[derive(Debug)]
struct MinHeapNode<T> {
    freq: usize,
    node: NodeType<T>,
}

impl<T> MinHeapNode<T>
where
    T: Display + Debug,
{
    fn new_branch(freq: usize) -> Self {
        Self {
            freq,
            node: NodeType::Branch{ 
                left: None,
                right: None,
            }
        }
    }

    fn new_leaf(data: T, freq: usize) -> Self {
        Self {
            freq,
            node: NodeType::Leaf(data), 
        }

    }

    fn is_leaf(&self) -> bool {
        match self.node {
            NodeType::Leaf(_) => true,
            _ => false,
        }
    }

    fn left(&mut self, node: Box<MinHeapNode<T>>) {
        self.node.left(node);
    }

    fn right(&mut self, node: Box<MinHeapNode<T>>) {
        self.node.right(node);
    }

    fn left_as_mut(&mut self) -> Option<&mut Self> {
        self.node.left_as_mut()
    }

    fn right_as_mut(&mut self) -> Option<&mut Self> {
        self.node.right_as_mut()
    }

    fn leaf(&mut self) -> Option<&T> {
        self.node.leaf() 
    }
}

#[derive(Debug)]
struct MinHeap<T> {
    array: Vec<MinHeapNode<T>>,
}

impl<T> MinHeap<T>
where
    T: Display + Debug,
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

    fn extract_min(&mut self) -> Option<MinHeapNode<T>> {
        if self.array.is_empty() { return None; }
        self.swap_nodes(0, self.size() - 1);
        let node = self.array.pop();
        self.min_heapify(0);

        node
    }

    fn insert_min_heap(&mut self, node: MinHeapNode<T>) {
        let node_freq = node.freq;
        self.array.push(node);
        if self.array.is_empty() { return; }
        let mut i = self.size() - 1;
        
        while i > 0 && node_freq < self.freq_at((i - 1) / 2) {
            self.swap_nodes(i, (i - 1) / 2);
            i = (i - 1) / 2;
        }
    }

    fn build_min_heap(&mut self) {
        let n = (self.size() - 1) / 2;

        for i in n..0 {
            self.min_heapify(i);
        }
    }

    fn from(data: Vec<T>, freqs: Vec<usize>, size: usize) -> Self {
        let mut heap = Self::with_capacity(size);

        data.into_iter().zip(freqs.into_iter()).for_each(|(d, f)| {
            heap.array.push(MinHeapNode::new_leaf(d, f));
        });

        heap
    }
}

fn from<T>(data: Vec<T>, frequencies: Vec<usize>, size: usize) -> Option<MinHeapNode<T>>
where
    T: Display + Debug
{
    let mut heap: MinHeap<T> = MinHeap::from(data, frequencies, size);

    while !heap.is_size_one() && !heap.array.is_empty() {
        let mut left = heap.extract_min();
        let mut right = heap.extract_min();

        let top_freq = left.as_ref().map(|l| l.freq).unwrap_or_default()
            + right.as_ref().map(|r| r.freq).unwrap_or_default();
        let mut top = MinHeapNode::new_branch(top_freq);

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

fn print_codes<T>(root: &mut MinHeapNode<T>, array: &mut Vec<bool>, top: usize) -> Vec<(String, Codes)> 
where
    T: Display + Debug
{
    let mut dict  = Vec::new();
    if let Some(left) = root.left_as_mut() {
        array[top] = false;
        dict.append(&mut print_codes(left, array, top + 1));
    }

    if let Some(right) = root.right_as_mut() {
        array[top] = true;
        dict.append(&mut print_codes(right, array, top + 1));
    }

    if let Some(data) = root.leaf() {
        dict.push((data.to_string(), Codes::from(array[..top].to_vec())))
    }

    dict
}

pub fn huffman_codes<T>(data: Vec<T>, freqs: Vec<usize>, size: usize) -> Vec<(String, Codes)>
where
    T: Display + Debug
{
    if let Some(mut root) = from(data, freqs, size) {
        let mut array = vec![false; size];
        return print_codes(&mut root, &mut array, 0)
    }

    Vec::new()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn _1() {
        let array = vec!['a', 'b', 'c', 'd', 'e', 'f'];
        let freq = vec![5, 9, 12, 13, 16, 45];

        let size = array.len();

        let (actual_array, codes): (Vec<String>, Vec<Codes>) = huffman_codes(array, freq, size).into_iter().unzip();

        let expected_codes: Vec<Codes> = vec![ vec![false], vec![true,false,false,], vec![true,false,true], vec![true,true,false,false], vec![true,true,false,true], vec![true,true,true]].into_iter().map(|c| Codes::from(c)).collect();
        let expected_array = vec![ "f", "c", "d", "a", "b", "e" ];
        assert_eq!(expected_array, actual_array);
        assert_eq!(expected_codes, codes);
    }
}
