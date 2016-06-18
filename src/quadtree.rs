use std::fmt::Debug;
use std::vec::Vec;
use std::boxed::Box;

#[derive(Debug)]
pub struct QuadTree<T: Debug> {
    root: Node<T>
}

#[derive(Debug)]
enum Child<T: Sized + Debug> {
    Inner(Box<Node<T>>),
    Leaf(T)
}

impl<T: Debug> Child<T> {
    fn visit(&self) {
        match self {
            &Child::Leaf(ref t) => {
                println!("{:?}", t);
            },
            &Child::Inner(ref b) => {
                b.as_ref().visit();
            },
        }
    }
}

#[derive(Debug)]
struct Node<T: Sized + Debug> {
    children: Vec<Child<T>>
}

impl<T: Debug> Node<T> {

    fn new() -> Node<T> {
        return Node {
            children: Vec::with_capacity(4)
        };
    }

    fn visit(&self) {
        for i in 0..4 {
            match self.children.get(i) {
                Some(c) => c.visit(),
                None => println!("None")
            };
        };
    }

    fn add(&mut self, t: T) {
        if self.children.len() < 4 {
            self.children.push(Child::Leaf(t));
        } else {
            match self.children.pop().unwrap() {
                Child::Leaf(old_t) => {
                    let mut new_node = Node::new();
                    new_node.add(t);
                    new_node.add(old_t);
                    self.children.push(Child::Inner(Box::new(new_node)));
                },
                Child::Inner(b) => {

                }
            }
        }
    }
}

impl<T: Debug> QuadTree<T> {
    pub fn new(span: PosSpan) -> QuadTree<T> {
        return QuadTree { root: Node::new() };
    }

    pub fn visit(&self) {
        // println!("{:?}", self.root);
        self.root.visit();
    }

    pub fn add(&mut self, t: T) {
        self.root.add(t);
    }
}

pub enum Dir { N, S, E, W, NE, NW, SE, SW }

#[derive(Debug, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, PartialEq, Eq)]
pub struct PosSpan {
    pub nw: Position,
    pub width: i32,
    pub height: i32
}

impl Span<Position> for PosSpan {
    fn dir_of(&self, t: &Position) -> Option<Dir> {
        if t.x < self.nw.x {
            if t.y < self.nw.y {
                return Some(Dir::NW);
            } else if t.y < self.nw.y + self.height {
                return Some(Dir::W);
            } else {
                return Some(Dir::SW);
            }
        } else if t.x < self.nw.x + self.width {
            if t.y < self.nw.y {
                return Some(Dir::N);
            } else if t.y < self.nw.y + self.height {
                return None;
            } else {
                return Some(Dir::S);
            }
        } else {
            if t.y < self.nw.y {
                return Some(Dir::NE);
            } else if t.y < self.nw.y + self.height {
                return Some(Dir::E);
            } else {
                return Some(Dir::SE);
            }
        }
    }

    // fn split(self) -> Vec<PosSpan> {

    // }
}

pub trait Span<T> {
    fn dir_of(&self, t: &T) -> Option<Dir>;
    // fn north_span(&self) -> Span<T>;
    // fn south_span(&self) -> Span<T>;
    // fn east_span(&self) -> Span<T>;
    // fn west_span(&self) -> Span<T>;
}






