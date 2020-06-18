use crate::*;

#[test]
fn test_macro() {
    let p = pos!();
    dbg!(p);
    assert_eq!(p, pos(5, 13));
}

#[test]
fn test_loc_macro() {
    let a = locof!(pos(1, 2), pos(3, 4));
    let b = locof!([1, 2], [3, 4]);
    let c = locof!(1, 2, 3, 4);
    let d = locof!((1, 2)..(3, 4));
    assert_eq!(a, b);
    assert_eq!(b, c);
    assert_eq!(c, d);
}

#[test]
fn test_pos_macro() {
    let a = posof!(1, 2);
    let b = posof!([1, 2]);
    assert_eq!(a, b);
}
