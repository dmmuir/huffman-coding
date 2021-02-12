use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

type BoxOption<T> = Option<Box<T>>;

#[derive(Clone, Debug)]
enum NodeType<T> {
    Leaf(T),
    Branch {
        left: BoxOption<Node<T>>,
        right: BoxOption<Node<T>>,
    },
}

impl<T: Hash> Hash for NodeType<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Leaf(t) => t.hash(state),
            Self::Branch { left, right } => {
                left.as_ref().hash(state);
                right.as_ref().hash(state);
            }
        }
    }
}

impl<T> NodeType<T> {
    fn leaf(&self) -> Option<&T> {
        match self {
            Self::Leaf(t) => Some(t),
            _ => None,
        }
    }

    fn left(&mut self, node: Box<Node<T>>) {
        if let Self::Branch {
            ref mut left,
            right: _,
        } = *self
        {
            *left = Some(node);
        }
    }

    fn right(&mut self, node: Box<Node<T>>) {
        if let Self::Branch {
            left: _,
            ref mut right,
        } = *self
        {
            *right = Some(node);
        }
    }

    fn left_as_ref(&self) -> Option<&Node<T>> {
        if let Self::Branch { ref left, right: _ } = *self {
            if let Some(ref node) = *left {
                return Some(node.as_ref());
            }
        }

        None
    }

    fn right_as_ref(&self) -> Option<&Node<T>> {
        if let Self::Branch { left: _, ref right } = *self {
            if let Some(ref node) = *right {
                return Some(node.as_ref());
            }
        }

        None
    }
}

#[derive(Clone, Debug, Hash)]
pub struct Node<T> {
    pub freq: usize,
    node: NodeType<T>,
}

impl<T> Node<T>
where
    T: Debug,
{
    pub fn new_branch(freq: usize) -> Self {
        Self {
            freq,
            node: NodeType::Branch {
                left: None,
                right: None,
            },
        }
    }

    pub fn new_leaf(data: T, freq: usize) -> Self {
        Self {
            freq,
            node: NodeType::Leaf(data),
        }
    }

    pub fn left(&mut self, node: Box<Node<T>>) {
        self.node.left(node);
    }

    pub fn right(&mut self, node: Box<Node<T>>) {
        self.node.right(node);
    }

    pub fn left_as_ref(&self) -> Option<&Self> {
        self.node.left_as_ref()
    }

    pub fn right_as_ref(&self) -> Option<&Self> {
        self.node.right_as_ref()
    }

    pub fn leaf(&self) -> Option<&T> {
        self.node.leaf()
    }
}

impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.freq.cmp(&other.freq))
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}
