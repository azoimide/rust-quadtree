use std::fmt::Debug;
use std::vec::Vec;
use std::boxed::Box;
use std::collections::HashMap;

#[derive(Debug)]
pub struct QuadTree<S, T: Debug> {
    root: Node<S, T>
}

#[derive(Debug)]
enum Child<S, T: Sized + Debug> {
    Inner(Box<Node<S, T>>),
    Leaf(S, T)
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
        for (key, span) in sub_spans.drain() {
            if span.contains(&t) {
                if self.children.contains_key(&key) {
                    println!("{:?}", self.children.get(&key).unwrap());
                } else {
                    self.children.insert(key.clone(), Child::Leaf(span, t));
                }
                return;
            }
        }
        // for i in 0..4 {
        //     if sub_spans[i].dir_of(&t) == None {
        //         // match self.get_child(i) {
        //         //     Some(c) => c,
        //         //     None => expr,
        //         // };
        //         return;
        //     }
        // }
        unreachable!("Tried to add {:?} to node with span {:?}", t, self.span);


        // if self.children.len() < 4 {
        //     self.children.push(Child::Leaf(t));
        // } else {
        //     match self.children.pop().unwrap() {
        //         Child::Leaf(old_t) => {
        //             let mut new_node = Node::new();
        //             new_node.add(t);
        //             new_node.add(old_t);
        //             self.children.push(Child::Inner(Box::new(new_node)));
        //         },
        //         Child::Inner(b) => {

        //         }
        //     }
        // }
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
        assert_eq!(None, self.root.span.dir_of(&t));
        self.root.add(t);
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Dir { N, S, E, W, NE, NW, SE, SW }

pub trait Span<S: Span<S, T>, T> {
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
}







