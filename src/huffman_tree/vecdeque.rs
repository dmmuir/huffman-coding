use super::node::Node;

use std::collections::VecDeque;
use std::fmt::Debug;

fn min<T>(
    first_queue: &mut VecDeque<Node<T>>,
    second_queue: &mut VecDeque<Node<T>>,
) -> Option<Node<T>> {
    if first_queue.is_empty() {
        return second_queue.pop_front();
    }

    if second_queue.is_empty() {
        return first_queue.pop_front();
    }

    if first_queue.front() < second_queue.front() {
        return first_queue.pop_front();
    }

    second_queue.pop_front()
}

pub fn from<T>(data: &[T], freq: &[usize], size: usize) -> Option<Node<T>>
where
    T: Debug + Clone,
{
    let mut first_queue = VecDeque::with_capacity(size);
    let mut second_queue = VecDeque::with_capacity(size);

    data.iter().zip(freq.iter()).for_each(|(d, f)| {
        first_queue.push_back(Node::new_leaf(d.clone(), *f));
    });

    while !(first_queue.is_empty() && second_queue.len() == 1) {
        let mut left = min(&mut first_queue, &mut second_queue);
        let mut right = min(&mut first_queue, &mut second_queue);

        let get_freq: fn(&Node<T>) -> usize = |n| n.freq;
        let top_freq = left.as_ref().map(get_freq).unwrap_or_default()
            + right.as_ref().map(get_freq).unwrap_or_default();
        let mut top = Node::new_branch(top_freq);

        if let Some(left) = left.take() {
            top.left(Box::new(left));
        }

        if let Some(right) = right.take() {
            top.right(Box::new(right));
        }

        second_queue.push_back(top);
    }

    second_queue.pop_front()
}
