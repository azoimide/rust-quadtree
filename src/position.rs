use quadtree::{Span, Dir};

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
