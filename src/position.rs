use std::collections::HashMap;
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

impl PosSpan {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> PosSpan {
        return PosSpan {
            nw: Position { x: x, y: y },
            width: w,
            height: h
        };
    }
}

impl Span<PosSpan, Position> for PosSpan {
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

    fn north_span(&self) -> PosSpan {
        return PosSpan::new(self.nw.x, self.nw.y - self.height, self.width, self.height);
    }

    fn south_span(&self) -> PosSpan {
        return PosSpan::new(self.nw.x, self.nw.y + self.height, self.width, self.height);
    }

    fn west_span(&self) -> PosSpan {
        return PosSpan::new(self.nw.x - self.width, self.nw.y, self.width, self.height);
    }

    fn east_span(&self) -> PosSpan {
        return PosSpan::new(self.nw.x + self.width, self.nw.y, self.width, self.height);
    }

    fn can_split(&self) -> bool {
        return self.width > 1 && self.height > 1;
    }

    fn split(&self) -> HashMap<Dir, PosSpan> {
        let left_x = self.nw.y;
        let left_y = self.nw.y;
        let width_mid = self.width / 2;
        let height_mid = self.height / 2;
        let mut result = HashMap::new();
        result.insert(Dir::NW, PosSpan::new(left_x, left_y, width_mid, height_mid));
        result.insert(Dir::NE, PosSpan::new(left_x + width_mid, left_y, self.width - width_mid, height_mid));
        result.insert(Dir::SW, PosSpan::new(left_x, left_y + height_mid, width_mid, self.height - height_mid));
        result.insert(Dir::SE, PosSpan::new(left_x + width_mid, left_y + height_mid, self.width - width_mid, self.height - height_mid));
        return result;
    }
}
