use super::Tree23;
use super::Tree23::*;
use super::UpI;
use super::UpI::*;
// use super::Tree23::UpI::SameI;
// use super::Tree23::UpI::Overflow;
use std::cmp::Ordering::*;

impl<T: Ord + Copy> Tree23<T> {

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

    pub fn ins(&self, value: T) -> UpI<T> {
        match self {
            Empty => Overflow(Empty,value,Empty),
            NonEmpty2(node) => match value.cmp(&node.element) {
                Less => match node.left.ins(value) {
                          SameI(lnew) => SameI(Tree23::create2(lnew,node.element,node.right.clone())),
                          Overflow(l1,a,l2) => SameI(Tree23::create3(l1,a,l2,node.element,node.right.clone())),
                        }
                Equal => SameI(NonEmpty2(node.clone())),
                Greater => match node.right.ins(value) {
                    SameI(rnew) => SameI(Tree23::create2(node.left.clone(),node.element,rnew)),
                    Overflow(r1,a,r2) => SameI(Tree23::create3(node.left.clone(),node.element,r1,a,r2)),
                  },
            },
            NonEmpty3(node) => match value.cmp(&node.element1) {
                Less => match node.left.ins(value) {
                    SameI(lnew) => SameI(Tree23::create3(lnew,node.element1,node.middle.clone(),node.element2,node.right.clone())),
                    Overflow(l1,a,l2) => Overflow(Tree23::create2(l1,a,l2),
                                                  node.element1,
                                                  Tree23::create2(node.middle.clone(),node.element2,node.right.clone())),
                  },
                Equal => SameI(NonEmpty3(node.clone())),
                Greater => match value.cmp(&node.element2) {
                    Less => match node.middle.ins(value) {
                        SameI(mnew) => SameI(Tree23::create3(node.left.clone(),
                                                             node.element1,
                                                             mnew,
                                                             node.element2,
                                                             node.right.clone())),
                        Overflow(m1,a,m2) => Overflow(Tree23::create2(node.left.clone(),node.element1,m1),
                                                      a,
                                                      Tree23::create2(m2,node.element2,node.right.clone())),
                      },
                    Equal => SameI(NonEmpty3(node.clone())),
                    Greater => match node.right.ins(value) {
                        SameI(rnew) => SameI(Tree23::create3(node.left.clone(),
                                                             node.element1,
                                                             node.middle.clone(),
                                                             node.element2,
                                                             rnew)),
                        Overflow(r1,a,r2) => Overflow(Tree23::create2(node.left.clone(),node.element1,node.middle.clone()),
                                                      node.element2,
                                                      Tree23::create2(r1,a,r2)),
                      },
            },

        }
        }
    }

}