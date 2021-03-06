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
                    self.root.size = new_root.size;
                    if new_root.size > 0 {
                        self.root.children.insert(dir.clone(), Child::Inner(Box::new(new_root)));
                    }
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

    pub fn size(&self) -> usize {
        return self.root.size;
    }

    pub fn size_actual(&self) -> usize {
        return self.root.size();
    }

    pub fn remove(&mut self, t: &T) -> usize {
        return self.root.remove(t);
    }

    pub fn elements(&self) -> Vec<T> {
        return self.root.elements();
    }

    /// Not implemented!
    pub fn nearest<F>(&self, t: &T, norm: F) where F: Fn(&T, &T) -> f32 {
        unimplemented!();
    }

    /// Not implemented!
    pub fn nearest_n<F>(&self, t: &T, n: i32, norm: F) where F: Fn(&T, &T) -> f32 {
        unimplemented!();
    }

    /// Not implemented!
    pub fn smallest_span(&self) -> S {
        unimplemented!();
    }


    // removes all elements equal to 'old' and adds new element 'new'
    // returns number of elements removed
    // efficient on small changes in element  
    pub fn replace_with(&mut self, old: &T, new: T) -> usize {
        self.ensure_contains(&new);
        return self.root.replace_with(old, new);
    }

    /// Not implemented!
    pub fn rebalance(&self) {
        unimplemented!();
    }
}

#[derive(Debug)]
struct Node<S, T: Sized + Debug> {
    span: S, 
    children: HashMap<Dir, Child<S, T>>,
    size: usize     // number of elements contained in the span
}

impl<S: Span<S, T> + Debug, T: Debug + Clone + PartialEq> Node<S, T> {

    fn new(span: S) -> Node<S, T> {
        return Node {
            span: span,
            children: HashMap::new(),
            size: 0
        };
    }

    fn add(&mut self, t: T) {
        self.size += 1;
        let mut sub_spans = self.span.split();
        for (key, span) in sub_spans.drain() {
            if span.contains(&t) {
                let old_child = self.children.remove(&key);
                let new_child = self.new_child(span, old_child, t);
                self.children.insert(key, new_child);
                return;
            }
        }
        unreachable!("Tried to add {:?} to node with span {:?}", t, self.span);
    }

    fn new_child(&self, span: S, child: Option<Child<S, T>>, t: T) -> Child<S, T> {
        return match child {
            None => Child::Leaf(span, vec![t]),
            Some(c) => {
                match c {
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
                }
            }
        };
    }

    fn print(&self, depth: usize) {
        println!("Node ({}) {:?}", self.size, self.span);
        let mut indent = String::new();
        for _ in 0..depth {
            indent.push_str(".   ");
        }
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

    fn remove(&mut self, t: &T) -> usize {
        if !self.span.contains(t) {
            return 0;
        }

        for (_, child) in self.children.iter_mut() {
            let removed = match child {
                &mut Child::Leaf(_, ref mut v) => {
                    let mut i = 0;
                    let mut removed = 0;
                    while i < v.len() {
                        if v.get(i).unwrap() == t {
                            v.remove(i);
                            removed += 1;
                        } else {
                            i += 1;
                        }
                    }
                    removed
                },
                &mut Child::Inner(ref mut b) => {
                    b.as_mut().remove(t)
                }
            };
            if removed > 0 {
                return removed;
            }
        }
        return 0;
    }

    pub fn elements(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.size);
        for child in self.children.values() {
            match child {
                &Child::Leaf(_, ref v) => {
                    for elem in v {
                        result.push(elem.clone());
                    }
                },
                &Child::Inner(ref b) => {
                    for elem in b.as_ref().elements() {
                        result.push(elem);
                    }
                }
            };
        }
        return result;
    }

    fn size(&self) -> usize {
        let mut result = 0;
        for (_, child) in &self.children {
            result += match child {
                &Child::Leaf(_, ref v) => v.len(),
                &Child::Inner(ref b) => b.as_ref().size()
            }
        };
        return result;
    }


    fn replace_with(&mut self, old: &T, new: T) -> usize {

        for (_, child) in self.children.iter_mut() {
            match child {
                &mut Child::Inner(ref mut b) => {
                    if b.as_ref().span.contains(old) && b.as_ref().span.contains(&new) {
                        return b.as_mut().replace_with(old, new);
                    }
                },
                _ => {}
            };
        }

        self.add(new);
        return self.remove(old);
    }
}

#[derive(Debug)]
enum Child<S, T: Sized + Debug> {
    Inner(Box<Node<S, T>>),
    Leaf(S, Vec<T>)
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

    /// Returns the `S` in the given direction.
    fn span_at(&self, dir: &Dir) -> S {
        return match dir {
            &Dir::N => self.north_span(),
            &Dir::S => self.south_span(),
            &Dir::E => self.east_span(),
            &Dir::W => self.west_span(),
            &Dir::NE => self.north_span().east_span(),
            &Dir::NW => self.north_span().west_span(),
            &Dir::SE => self.south_span().east_span(),
            &Dir::SW => self.south_span().west_span()
        };
    }
    
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
