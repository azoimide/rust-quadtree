extern crate rand;

mod quadtree;
mod position;

use quadtree::{QuadTree};
use position::{Position, PosSpan};

fn main() {

    // let mut tree = QuadTree::new(PosSpan::new(0,0,10,10));
    // // println!("{:?}", tree);
    // // tree.print();
    // tree.add(Position { x: 1, y: 2 });
    // // tree.print();
    // tree.add(Position { x: 6, y: 2 });
    // tree.add(Position { x: 1, y: 7 });
    // tree.add(Position { x: 8, y: 7 });
    // // tree.print();
    // tree.add(Position { x: 1, y: 2 });
    // tree.add(Position { x: 1, y: 2 });
    // // tree.add(Position { x: 1, y: 2 });
    // // tree.add(Position { x: 1, y: 2 });
    // tree.print();

    // let ps = tree.scan(&PosSpan::new(1, 2, 6, 5));
    // println!("{:?}", ps);

    // let mut tree = QuadTree::new(PosSpan::new(0,0,4,4));
    // tree.print();
    // tree.add(Position { x: 0, y: 0 });
    // tree.add(Position { x: 1, y: 0 });
    // tree.add(Position { x: 0, y: 1 });
    // tree.print();
    // tree.add(Position { x: 1, y: 1 });
    // tree.add(Position { x: 1, y: 1 });
    // tree.add(Position { x: 1, y: 1 });
    // tree.add(Position { x: 1, y: 2 });
    // tree.add(Position { x: 1, y: 3 });
    // tree.print();

    // let mut tree = QuadTree::new(PosSpan::new(0,0,2,2));
    // tree.print();
    // tree.add(Position { x: 18, y: 10 });
    // tree.print();

//    let mut tree = QuadTree::new(PosSpan::new(0,0,50,50));
//    for _ in 0..250 {
//        tree.add(Position::new(rand::random::<i32>() % 50, rand::random::<i32>() % 50));
//    }
//    tree.print();
//    // println!("{:?}", tree.scan(&PosSpan::new(10, 10, 10, 10)));
//    // let pos = Position::new(10, 12);
//    // println!("contains {:?}: {:?}", pos, tree.contains(&pos));
//    println!("size: {}", tree.size());
//    println!("actual size: {}", tree.size_actual());
//    let p = Position::new(1, 2);
//    println!("remove {:?}: {}", p, tree.remove(&p));

    // tree.add(Position { x: 18, y: 10 });
    // tree.print();

    let mut tree = QuadTree::new(PosSpan::new(0,0,1,1));
    tree.add(Position::new(10,10));
    tree.print();


}


// impl<Position> Span<Position> for PosSpan {
//     fn dir_of(&self, t: &Position) -> Option<Dir> {
//         return self.dir_of(t);
//     }
// }
