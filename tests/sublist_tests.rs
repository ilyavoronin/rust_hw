use rust_hw::sublist::{Comparison, compare};

#[test]
fn test_sublist() {
    let res = compare(&[1, 2, 3], &[0, 1, 2, 3, 4, 5]);
    assert_eq!(res, Comparison::Sublist)
}

#[test]
fn test_sublist_last() {
    let res = compare(&[5], &[0, 1, 2, 3, 4, 5]);
    assert_eq!(res, Comparison::Sublist)
}

#[test]
fn test_sublist_first() {
    let res = compare(&[0], &[0, 1, 2, 3, 4, 5]);
    assert_eq!(res, Comparison::Sublist)
}

#[test]
fn test_superlist() {
    let res = compare(&[1, 2, 3, 4, 5], &[1, 2, 3]);
    assert_eq!(res, Comparison::Superlist)
}

#[test]
fn test_eq() {
    let res = compare(&[1, 2, 3, 4], &[1, 2, 3, 4]);
    assert_eq!(res, Comparison::Equal)
}

#[test]
fn test_other() {
    let res = compare(&[1, 2, 3, 4, 5], &[2, 3, 4, 5, 6]);
    assert_eq!(res, Comparison::Other)
}

#[test]
fn test_str() {
    let res = compare(&["aba", "caba", "aba"], &["aaa", "aba", "caba", "aba", "daba"]);
    assert_eq!(res, Comparison::Sublist)
}

#[test]
fn test_empty_sublist() {
    let res = compare(&[], &[1]);
    assert_eq!(res, Comparison::Sublist)
}

#[test]
fn test_empty_superlist() {
    let res = compare(&[1], &[]);
    assert_eq!(res, Comparison::Superlist)
}

#[test]
fn test_empty_eq() {
    let res = compare::<()>(&[], &[]);
    assert_eq!(res, Comparison::Equal)
}