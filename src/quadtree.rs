use std::fmt::Debug;
use std::vec::Vec;

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
        println!("{:?}", self);
        // match self {

        // }
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


        // if self.children.len() < 1 {
            
        // } else {
        //     match self.children.pop().unwrap() {
        //         Child::LeafNode(t) => {
        //         },
        //         None => expr,
        //     }






        //     let tmp_t = self.children.pop().unwrap();
        //     let mut new_node = Node::new();
        //     new_node.add(tmp_t);
        //     self.children.push(Child::InnerNode(Box::new(new_node)));
        //     self.children.push(t);
        // }
    }
}

// #[derive(Debug)]
// struct Leaf<T: Sized + Debug> {
//     t: T
// }

impl<T: Debug> QuadTree<T> {
    pub fn new() -> QuadTree<T> {
        return QuadTree { root: Node::new() };
    }

    pub fn visit(&self) {
        println!("{:?}", self.root);
    }

    pub fn add(&mut self, t: T) {
        self.root.add(t);
    }
}






