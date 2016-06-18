extern crate quadtree;

use quadtree::position::{Position, PosSpan};
use quadtree::quadtree::{Span, Dir};

fn test_span() -> PosSpan {
    return PosSpan::new(0, 0, 10, 10);
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

#[test]
fn can_split() {
    assert_eq!(true, test_span().can_split());
}

#[test]
fn cannot_split_x() {
    assert_eq!(false, PosSpan::new(0, 0, 1, 2).can_split());
}

#[test]
fn cannot_split_y() {
    assert_eq!(false, PosSpan::new(0, 0, 2, 1).can_split());
}

#[test]
fn split_even() {
    assert_eq!(
        vec![
            PosSpan::new(1,1,2,3), PosSpan::new(3,1,2,3), 
            PosSpan::new(1,4,2,3), PosSpan::new(3,4,2,3)],
        PosSpan::new(1,1,4,6).split());
} 

#[test]
fn north_span() {
    assert_eq!(PosSpan::new(0,0,5,5), PosSpan::new(0,5,5,5).north_span());
}

#[test]
fn south_span() {
    assert_eq!(PosSpan::new(0,10,5,5), PosSpan::new(0,5,5,5).south_span());
}

#[test]
fn west_span() {
    assert_eq!(PosSpan::new(-5,5,5,5), PosSpan::new(0,5,5,5).west_span());
}

#[test]
fn east_span() {
    assert_eq!(PosSpan::new(5,5,5,5), PosSpan::new(0,5,5,5).east_span());
}
