extern crate core;

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
        // println!("{:?}", sub_spans);
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

    // fn get_child(&self, i: i32) -> &mut Child<S, T> {
    //     match i {
    //         0 => return self.nw_child,
    //         1 => return self.ne_child,
    //         2 => return self.sw_child,
    //         3 => return self.se_child,
    //         _ => unreachable!()
    //     }
    // }

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
    // pub fn new(span: PosSpan) -> QuadTree<T> {
    //     return QuadTree { root: Node::new() };
    // }

    // pub fn visit(&self) {
    //     // println!("{:?}", self.root);
    //     self.root.visit();
    // }

    pub fn add(&mut self, t: T) {
        if !self.root.span.contains(&t) {
            let new_spans = expand_span(&self.root.span, &t);
            println!("{:?}", new_spans);
            let new_root = Node::new(S::merge(new_spans.values()));
            println!("{:?}", new_root);
        }
        // self.root.add(t);
    }

    pub fn print(&self) {
        print!("root: ");
        self.root.print(1);
    }
}

pub fn expand_span<S: Span<S, T>, T>(span: &S, t: &T) -> HashMap<Dir, S> {
    let dir = span.dir_of(&t).unwrap();
    let mut new_spans = HashMap::new();
    match dir {
        Dir::N => {
            // new_spans.insert(Dir::NE, span.north_span().east_span());
            new_spans.insert(Dir::NW, span.north_span());
            // new_spans.insert(Dir::SE, span.east_span());
            new_spans.insert(Dir::SW, span.clone());
        }, 
        Dir::S => {
            // new_spans.insert(Dir::NE, span.east_span());
            new_spans.insert(Dir::NW, span.clone());
            // new_spans.insert(Dir::SE, span.south_span().east_span());
            new_spans.insert(Dir::SW, span.south_span());
        }, 
        Dir::E => {
            new_spans.insert(Dir::NE, span.east_span());
            new_spans.insert(Dir::NW, span.clone());
            // new_spans.insert(Dir::SE, span.south_span().east_span());
            // new_spans.insert(Dir::SW, span.south_span());
        }, 
        Dir::W => {
            // new_spans.insert(Dir::NE, span.north_span());
            // new_spans.insert(Dir::NW, span.north_span().west_span());
            new_spans.insert(Dir::SE, span.clone());
            new_spans.insert(Dir::SW, span.west_span());
        }, 
        Dir::NE => {
            new_spans.insert(Dir::NE, span.north_span().east_span());
            // new_spans.insert(Dir::NW, span.north_span());
            // new_spans.insert(Dir::SE, span.east_span());
            new_spans.insert(Dir::SW, span.clone());
        }, 
        Dir::NW => {
            // new_spans.insert(Dir::NE, span.north_span());
            new_spans.insert(Dir::NW, span.north_span().west_span());
            new_spans.insert(Dir::SE, span.clone());
            // new_spans.insert(Dir::SW, span.west_span());
        }, 
        Dir::SE => {
            // new_spans.insert(Dir::NE, span.east_span());
            new_spans.insert(Dir::NW, span.clone());
            new_spans.insert(Dir::SE, span.east_span().south_span());
            // new_spans.insert(Dir::SW, span.south_span());
        }, 
        Dir::SW => {
            new_spans.insert(Dir::NE, span.clone());
            // new_spans.insert(Dir::NW, span.west_span());
            // new_spans.insert(Dir::SE, span.south_span());
            new_spans.insert(Dir::SW, span.south_span().west_span());
        }
    }
    return new_spans;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Dir { N, S, E, W, NE, NW, SE, SW }

pub trait Span<S: Span<S, T>, T>: Clone + Eq + PartialEq {
    fn dir_of(&self, t: &T) -> Option<Dir>;
    fn can_split(&self) -> bool;
    fn split(&self) -> HashMap<Dir, S>;
    fn north_span(&self) -> S;
    fn south_span(&self) -> S;
    fn east_span(&self) -> S;
    fn west_span(&self) -> S;
    fn contains(&self, t: &T) -> bool {
        return self.dir_of(t) == None;
    }
    // fn expand(&self, t: &T) -> HashMap<Dir, S> {
    //     let dir = self.dir_of(&t).unwrap();
    //     let mut new_spans = HashMap::new();
    //     match dir {
    //         Dir::N => {
    //             // new_spans.insert(Dir::NE, self.north_span().east_span());
    //             new_spans.insert(Dir::NW, self.north_span());
    //             // new_spans.insert(Dir::SE, self.east_span());
    //             new_spans.insert(Dir::SW, self.clone());
    //         }, 
    //         Dir::S => {
    //             // new_spans.insert(Dir::NE, self.east_span());
    //             new_spans.insert(Dir::NW, self.clone());
    //             // new_spans.insert(Dir::SE, self.south_span().east_span());
    //             new_spans.insert(Dir::SW, self.south_span());
    //         }, 
    //         Dir::E => {
    //             new_spans.insert(Dir::NE, self.east_span());
    //             new_spans.insert(Dir::NW, self.clone());
    //             // new_spans.insert(Dir::SE, self.south_span().east_span());
    //             // new_spans.insert(Dir::SW, self.south_span());
    //         }, 
    //         Dir::W => {
    //             // new_spans.insert(Dir::NE, self.north_span());
    //             // new_spans.insert(Dir::NW, self.north_span().west_span());
    //             new_spans.insert(Dir::SE, self.clone());
    //             new_spans.insert(Dir::SW, self.west_span());
    //         }, 
    //         Dir::NE => {
    //             new_spans.insert(Dir::NE, self.north_span().east_span());
    //             // new_spans.insert(Dir::NW, self.north_span());
    //             // new_spans.insert(Dir::SE, self.east_span());
    //             new_spans.insert(Dir::SW, self.clone());
    //         }, 
    //         Dir::NW => {
    //             // new_spans.insert(Dir::NE, self.north_span());
    //             new_spans.insert(Dir::NW, self.north_span().west_span());
    //             new_spans.insert(Dir::SE, self.clone());
    //             // new_spans.insert(Dir::SW, self.west_span());
    //         }, 
    //         Dir::SE => {
    //             // new_spans.insert(Dir::NE, self.east_span());
    //             new_spans.insert(Dir::NW, self.clone());
    //             new_spans.insert(Dir::SE, self.east_span().south_span());
    //             // new_spans.insert(Dir::SW, self.south_span());
    //         }, 
    //         Dir::SW => {
    //             new_spans.insert(Dir::NE, self.clone());
    //             // new_spans.insert(Dir::NW, self.west_span());
    //             // new_spans.insert(Dir::SE, self.south_span());
    //             new_spans.insert(Dir::SW, self.south_span().west_span());
    //         }
    //     }
    //     return new_spans;
    // }
    fn merge(spans: Values<Dir, S>) -> S;
}







