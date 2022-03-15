mod ins;

// tree itself

#[derive(Clone, Debug)]
pub enum Tree23<T> {
    Empty,
    NonEmpty2(Box<TreeNode2<T>>),
    NonEmpty3(Box<TreeNode3<T>>),
}

#[derive(Clone, Debug)]
pub struct TreeNode2<T> {
    pub left: Tree23<T>,
    pub element: T,
    pub right: Tree23<T>,
}

#[derive(Clone, Debug)]
pub struct TreeNode3<T> {
    pub left: Tree23<T>,
    pub element1: T,
    pub middle: Tree23<T>,
    pub element2: T,
    pub right: Tree23<T>,
}

use Tree23::*;

impl<A> Tree23<A> {
    pub fn create2(left: Self, val: A, right: Self) -> Self {
        NonEmpty2(Box::new(TreeNode2 {
            left: left,
            element: val,
            right: right,
        }))
    }

    pub fn create3(left: Self, val1: A, mid: Self, val2: A, right: Self) -> Self {
        NonEmpty3(Box::new(TreeNode3 {
            left: left,
            element1: val1,
            middle: mid,
            element2: val2,
            right: right,
        }))
    }
}

// helper structure for insertion

#[derive(Clone, Debug)]
pub enum UpI<T> {
    SameI(Tree23<T>),
    Overflow(Tree23<T>, T, Tree23<T>),
}

use UpI::*;

impl<T: Ord> UpI<T> {
    pub fn collapseI(self) -> Tree23<T> {
        match self {
            SameI(t) => t,
            Overflow(l, a, r) => Tree23::create2(l, a, r),
        }
    }
}
