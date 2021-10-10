use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Other,
}

pub fn compare<T>(a: &[T], b: &[T]) -> Comparison
where T: Eq {

    fn is_sublist<T: Eq>(sub: &[T], arr: &[T]) -> bool {
        (0..arr.len()).any(|start| arr[start..].starts_with(sub))
    }
    match a.len().cmp(&b.len()) {
        Ordering::Equal => if a == b { Comparison::Equal } else { Comparison::Other }
        Ordering::Greater => if is_sublist(b, a) { Comparison::Superlist } else { Comparison::Other }
        Ordering::Less => if is_sublist(a, b) { Comparison::Sublist } else { Comparison::Other }
    }
}