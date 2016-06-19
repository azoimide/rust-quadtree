extern crate core;

use std::mem;
use std::fmt::Debug;
use std::vec::Vec;
use std::boxed::Box;
use std::collections::HashMap;
use std::collections::hash_map::Values;
use self::core::ops::RangeFull;

#[derive(Debug)]
pub struct QuadTree<S, T: Debug> {
    root: Node<S, T>
}

#[derive(Debug)]
enum Child<S, T: Sized + Debug> {
    Inner(Box<Node<S, T>>),
    Leaf(S, Vec<T>)
}

impl<S, T: Debug> Child<S, T> {
    // fn visit(&self) {
    //     match self {
    //         &Child::Leaf(_, ref t) => {
    //             println!("{:?}", t);
    //         },
    //         &Child::Inner(ref b) => {
    //             b.as_ref().visit();
    //         }
    //     }
    // }
}

#[derive(Debug)]
struct Node<S, T: Sized + Debug> {
    span: S, 
    children: HashMap<Dir, Child<S, T>>
}

impl<S: Span<S, T> + Debug, T: Debug + Clone + PartialEq> Node<S, T> {

    fn new(span: S) -> Node<S, T> {
        return Node {
            span: span,
            children: HashMap::new()
        };
    }

    // fn visit(&self) {
    //     // for i in 0..4 {
    //     //     match self.children.get(i) {
    //     //         Some(c) => c.visit(),
    //     //         None => println!("None")
    //     //     };
    //     // };
    // }

    fn add(&mut self, t: T) {
        let mut sub_spans = self.span.split();
        for (key, span) in sub_spans.drain() {
            if span.contains(&t) {
                if self.children.contains_key(&key) {
                    let child = self.children.remove(&key).unwrap();
                    let new_child: Child<S, T> = match child {
                        Child::Leaf(span, mut v) => {
                            if v.len() > 1 && self.span.can_split() {
                                let mut new_node = Node::new(span);
                                for old_t in v.drain(RangeFull) {
                                    new_node.add(old_t);
                                }
                                new_node.add(t);
                                Child::Inner(Box::new(new_node))
                            } else {
                                v.push(t);
                                Child::Leaf(span, v)
                            }
                        },
                        Child::Inner(mut b) => {
                            b.as_mut().add(t);
                            Child::Inner(b)
                        }
                    };
                    self.children.insert(key.clone(), new_child);
                } else {
                    self.children.insert(key.clone(), Child::Leaf(span, vec![t]));
                }
                return;
            }
        }
        unreachable!("Tried to add {:?} to node with span {:?}", t, self.span);
    }

    fn print(&self, depth: usize) {
        println!("Node {:?}", self.span);
        let indent = String::from_utf8(vec![b' '; 4 * depth]).unwrap();
        for (key, child) in self.children.iter() {
            print!("{}{:?}: ", indent, key);
            match child {
                &Child::Inner(ref b) => b.as_ref().print(depth + 1),
                &Child::Leaf(_, ref v) => println!("{:?}", v),
            };
        }
    }

    fn scan(&self, span: &S) -> Vec<T> {
        if !self.span.overlaps(span) {
            return Vec::new();
        }
        let mut result: Vec<T> = Vec::new();
        for (_, child) in &self.children {
            match child {
                &Child::Leaf(_, ref v) => {
                    for t in v.iter().filter(|t| span.contains(&t)) {
                        result.push(t.clone());
                    }
                },
                &Child::Inner(ref b) => {
                    for t in b.as_ref().scan(span) {
                        result.push(t.clone());
                    }
                }
            };
        }
        return result;
    }

    fn contains(&self, t: &T) -> bool {
        if !self.span.contains(t) {
            return false;
        }
        for (_, child) in &self.children {
            match child {
                &Child::Leaf(_, ref v) => {
                    if v.contains(t) {
                        return true;
                    }
                },
                &Child::Inner(ref b) => {
                    if b.as_ref().contains(t) {
                        return true;
                    }
                }
            };
        }
        return false;
    }
}

impl<S: Span<S, T> + Debug, T: Debug + Clone + PartialEq> QuadTree<S, T> {
    pub fn new(span: S) -> QuadTree<S, T> {
        return QuadTree { root: Node::new(span) };
    }

    pub fn add(&mut self, t: T) {
        self.ensure_contains(&t);
        self.root.add(t);
    }

    pub fn print(&self) {
        print!("root: ");
        self.root.print(1);
    }

    fn ensure_contains(&mut self, t: &T) {
        while !self.root.span.contains(&t) {
            let new_span = self.root.span.expand(&self.root.span.dir_of(&t).unwrap());
            let mut new_root = Node::new(new_span);
            for (dir, span) in new_root.span.split() {
                if span == self.root.span {
                    mem::swap(&mut self.root, &mut new_root);
                    self.root.children.insert(dir.clone(), Child::Inner(Box::new(new_root)));
                    break;
                }
                // unreachable!();
            }
        }
    }

    pub fn scan(&self, span: &S) -> Vec<T> {
        return self.root.scan(span);
    }

    pub fn contains(&self, t: &T) -> bool {
        return self.root.contains(t);
    }

    pub fn remove(&self, t: &T) -> bool {
        unimplemented!();
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Dir { N, S, E, W, NE, NW, SE, SW }

pub trait Span<S: Span<S, T>, T>: Clone + Eq + PartialEq {

    /// The direction of `t` in relation to `self`, or `None` if
    /// `self` `contains` `t`.
    fn dir_of(&self, t: &T) -> Option<Dir>;

    /// The span of equal size to the north of `self`.
    fn north_span(&self) -> S;

    /// The span of equal size to the south of `self`.
    fn south_span(&self) -> S;

    /// The span of equal size to the east of `self`.
    fn east_span(&self) -> S;

    /// The span of equal size to the west of `self`.
    fn west_span(&self) -> S;
    
    /// Returns a new `S` with double the size of `self` expanded
    /// in the direction `dir`.
    fn expand(&self, dir: &Dir) -> S;

    /// Returns a partition of `self` with four blocks.
    /// The partition of the result of `expand` must contain the 
    /// original `S`.
    fn split(&self) -> HashMap<Dir, S>;

    /// Returns true if the span can be split, e.g., if the span is
    /// represented by integers and the width and height is larger than `1`.
    fn can_split(&self) -> bool;
    
    /// Not implemented!
    fn merge(spans: Values<Dir, S>) -> S;

    /// Returns `true` if `self` overlaps with `other`, otherwise `false`.
    /// Used for `QuadTree.scan`.
    fn overlaps(&self, other: &S) -> bool;

    /// Returns `true` if `self` contains `t`, otherwise `false`.
    fn contains(&self, t: &T) -> bool {
        return self.dir_of(t) == None;
    }
}
