#[derive(Clone, Debug)]
pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

#[derive(Clone, Debug)]
pub struct TreeNode<T> {
    pub left: BinaryTree<T>,
    pub element: T,
    pub right: BinaryTree<T>,
}

use std::cmp::Ordering::*;
use BinaryTree::*;

impl<T: Ord> BinaryTree<T> {
    pub fn create(left: Self, value: T, right: Self) -> Self {
        NonEmpty(Box::new(TreeNode {
            left: left,
            element: value,
            right: right,
        }))
    }

    pub fn add(&mut self, value: T) {
        match *self {
            Empty => *self = Self::create(Empty, value, Empty),
            NonEmpty(ref mut node) => match value.cmp(&node.element) {
                Equal => (),
                Less => node.left.add(value),
                Greater => node.right.add(value),
            },
        }
    }

    pub fn contains(&self, value: T) -> bool {
        match self {
            Empty => false,
            NonEmpty(node) => match value.cmp(&node.element) {
                Equal => true,
                Less => node.left.contains(value),
                Greater => node.right.contains(value),
            },
        }
    }

    fn inorder_sub(self, ret: &mut Vec<T>) {
        match self {
            Empty => (),
            NonEmpty(node) => {
                node.left.inorder_sub(ret);
                ret.push(node.element);
                node.right.inorder_sub(ret);
            }
        }
    }
    pub fn inorder(self) -> Vec<T> {
        let mut ret = Vec::new();
        self.inorder_sub(&mut ret);
        ret
    }

    // deletion by replacement

    fn split_min(self) -> (T, BinaryTree<T>) {
        match self {
            Empty => panic!(),
            NonEmpty(mut node) => {
                if let Empty = node.left {
                    (node.element, node.right)
                } else {
                    let (x, l1) = node.left.split_min();
                    node.left = l1;
                    (x, NonEmpty(node))
                }
            }
        }
    }

    pub fn delete(self, value: T) -> (Self, bool) {
        match self {
            Empty => (self, false),
            NonEmpty(node) => {
                let TreeNode {
                    left,
                    element,
                    right,
                } = *node;
                match value.cmp(&element) {
                    Equal => {
                        if let Empty = right {
                            (left, true)
                        } else {
                            let (x, r1) = right.split_min();
                            (Self::create(left, x, r1), true)
                        }
                    }
                    Less => {
                        let (left0, b) = left.delete(value);
                        (Self::create(left0, element, right), b)
                    }
                    Greater => {
                        let (right0, b) = right.delete(value);
                        (Self::create(left, element, right0), b)
                    }
                }
            }
        }
    }

    // deletion by joining

    fn join(t1: Self, t2: Self) -> Self {
        match t1 {
            Empty => t2,
            NonEmpty(node1) => match t2 {
                Empty => NonEmpty(node1),
                NonEmpty(node2) => {
                    let TreeNode {
                        left: l1,
                        element: a,
                        right: r1,
                    } = *node1;
                    let TreeNode {
                        left: l2,
                        element: b,
                        right: r2,
                    } = *node2;
                    if let NonEmpty(node3) = Self::join(r1, l2) {
                        Self::create(
                            Self::create(l1, a, node3.left),
                            node3.element,
                            Self::create(node3.right, b, r2),
                        )
                    } else {
                        Self::create(l1, a, Self::create(Empty, b, r2))
                    }
                }
            },
        }
    }

    pub fn delete2(self, value: T) -> (Self, bool) {
        match self {
            Empty => (self, false),
            NonEmpty(node) => {
                let TreeNode {
                    left,
                    element,
                    right,
                } = *node;
                match value.cmp(&element) {
                    Equal => (Self::join(left, right), true),
                    Less => {
                        let (t, b) = left.delete(value);
                        (Self::create(t, element, right), b)
                    }
                    Greater => {
                        let (t, b) = right.delete(value);
                        (Self::create(left, element, t), b)
                    }
                }
            }
        }
    }

}
