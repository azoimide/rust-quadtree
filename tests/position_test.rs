extern crate quadtree;

use quadtree::position::{Position, PosSpan};
use quadtree::quadtree::{Span, Dir};

fn test_span() -> PosSpan {
    return PosSpan {
        nw: Position { x: 0, y: 0 },
        width: 10,
        height: 10
    };
}

#[test]
fn pos_within_span() {
    assert_eq!(None, test_span().dir_of(&Position{ x: 4, y: 5 }));
}

#[test]
fn pos_nw_of_span() {
    assert_eq!(Some(Dir::NW), test_span().dir_of(&Position{ x: -2, y: -10 }));
}

#[test]
fn pos_n_of_span() {
    assert_eq!(Some(Dir::N), test_span().dir_of(&Position{ x: 4, y: -4 }));
}

#[test]
fn pos_ne_of_span() {
    assert_eq!(Some(Dir::NE), test_span().dir_of(&Position{ x: 11, y: -1 }));
}

#[test]
fn pos_w_of_span() {
    assert_eq!(Some(Dir::W), test_span().dir_of(&Position{ x: -1, y: 5 }));
}

#[test]
fn pos_e_of_span() {
    assert_eq!(Some(Dir::E), test_span().dir_of(&Position{ x: 10, y: 5 }));
}

#[test]
fn pos_sw_of_span() {
    assert_eq!(Some(Dir::SW), test_span().dir_of(&Position{ x: -1, y: 10 }));
}

#[test]
fn pos_s_of_span() {
    assert_eq!(Some(Dir::S), test_span().dir_of(&Position{ x: 4, y: 10 }));
}

#[test]
fn pos_se_of_span() {
    assert_eq!(Some(Dir::SE), test_span().dir_of(&Position{ x: 11, y: 12 }));
}
