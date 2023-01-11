#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

// copied alireza4050's and DuBistKomisch's solutions

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;
    match (a.len(), b.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m > n => {
            if a.windows(b.len()).any(|a_slice| a_slice == b) {
                Superlist
            } else {
                Unequal
            }
        }
        (m, n) if m < n => {
            if a.windows(b.len()).any(|a_slice| a_slice == b) {
                Sublist
            } else {
                Unequal
            }
        }
        _ => {
            if a == b {
                Equal
            } else {
                Unequal
            }
        }
    }
}
