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

impl<S: Span<S, T> + Debug, T: Debug> Node<S, T> {

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
        println!("add span: {:?}", self.span);
        println!("add sub spans: {:?}", sub_spans);
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
                &Child::Leaf(_, ref t) => println!("{:?}", t),
            };
        }
    }
}

impl<S: Span<S, T> + Debug, T: Debug> QuadTree<S, T> {
    pub fn new(span: S) -> QuadTree<S, T> {
        return QuadTree { root: Node::new(span) };
    }

    pub fn add(&mut self, t: T) {
        println!("add: {:?}", t);
        self.ensure_contains(&t);
        println!("---");
        self.root.add(t);
    }

    pub fn print(&self) {
        print!("root: ");
        self.root.print(1);
    }

    fn ensure_contains(&mut self, t: &T) {
        while !self.root.span.contains(&t) {
            let new_span = self.root.span.expand(&self.root.span.dir_of(&t).unwrap());
            println!("new span: {:?}", new_span);
            let mut new_root = Node::new(new_span);
            println!("split: {:?}", new_root.span.split());
            for (dir, span) in new_root.span.split() {
                println!("root: {:?} sub: {:?}", self.root.span, span);
                if span == self.root.span {
                    mem::swap(&mut self.root, &mut new_root);
                    self.root.children.insert(dir.clone(), Child::Inner(Box::new(new_root)));
                    break;
                }
                // unreachable!();
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Dir { N, S, E, W, NE, NW, SE, SW }

pub trait Span<S: Span<S, T>, T>: Clone + Eq + PartialEq {
    fn dir_of(&self, t: &T) -> Option<Dir>;
    fn north_span(&self) -> S;
    fn south_span(&self) -> S;
    fn east_span(&self) -> S;
    fn west_span(&self) -> S;
    
    fn expand(&self, dir: &Dir) -> S;
    fn split(&self) -> HashMap<Dir, S>;
    fn can_split(&self) -> bool;
    fn merge(spans: Values<Dir, S>) -> S;

    fn overlaps(&self, other: &S) -> bool;

    fn contains(&self, t: &T) -> bool {
        return self.dir_of(t) == None;
    }
}
