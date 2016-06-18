mod quadtree;
mod position;

use quadtree::{QuadTree};
use position::{Position, PosSpan};

fn main() {

    let mut tree = QuadTree::new(PosSpan::new(0,0,10,10));
    println!("{:?}", tree);
    tree.add(Position { x: 1, y: 2 });
    tree.add(Position { x: 1, y: 7 });
    tree.add(Position { x: 1, y: 2 });
    tree.add(Position { x: 1, y: 2 });
    tree.add(Position { x: 1, y: 2 });
    tree.print();


}


// impl<Position> Span<Position> for PosSpan {
//     fn dir_of(&self, t: &Position) -> Option<Dir> {
//         return self.dir_of(t);
//     }
// }
