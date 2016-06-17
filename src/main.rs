mod quadtree;

use quadtree::{QuadTree, Position, PosSpan};

fn main() {

    let mut tree = QuadTree::new(PosSpan {
        nw: Position { x: 0, y: 0 },
        width: 10,
        height: 10
    });
    tree.visit();
    tree.add(Position { x: 1, y: 2 });
    tree.add(Position { x: 1, y: 2 });
    tree.add(Position { x: 1, y: 2 });
    tree.add(Position { x: 1, y: 2 });
    tree.add(Position { x: 1, y: 2 });
    // tree.add(Position { x: 1, y: 2 });
    tree.visit();


}


// impl<Position> Span<Position> for PosSpan {
//     fn dir_of(&self, t: &Position) -> Option<Dir> {
//         return self.dir_of(t);
//     }
// }
