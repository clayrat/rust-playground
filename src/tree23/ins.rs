use super::Tree23;
use super::Tree23::*;
use std::cmp::Ordering::*;

impl<T: Ord> Tree23<T> {

    pub fn contains(&self, value: T) -> bool {
        match self {
            Empty => false,
            NonEmpty2(node) => match value.cmp(&node.element) {
                Less => node.left.contains(value),
                Equal => true,
                Greater => node.right.contains(value),
            },
            NonEmpty3(node) => match value.cmp(&node.element1) {
                Less => node.left.contains(value),
                Equal => true,
                Greater => match value.cmp(&node.element2) {
                    Less => node.middle.contains(value),
                    Equal => true,
                    Greater => node.right.contains(value),
                },
            },
        }
    }
}