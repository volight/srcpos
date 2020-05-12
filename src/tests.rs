use crate::*;

#[test]
fn test_macro() {
    let p = pos!();
    dbg!(p);
    assert_eq!(p, pos(5, 13));
}
