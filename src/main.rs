mod quadtree;

use quadtree::QuadTree;

fn main() {

    let mut tree: QuadTree<Position> = QuadTree::new();
    tree.visit();
    tree.add(Position { x: 1, y: 2 });
    tree.add(Position { x: 1, y: 2 });
    tree.add(Position { x: 1, y: 2 });
    tree.add(Position { x: 1, y: 2 });
    tree.add(Position { x: 1, y: 2 });
    // tree.add(Position { x: 1, y: 2 });
    tree.visit();


}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32
}
