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
            if contains(a, b) {
                Superlist
            } else {
                Unequal
            }
        }
        (m, n) if m < n => {
            if contains(b, a) {
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

fn contains<T: PartialEq>(big: &[T], small: &[T]) -> bool {
    if big.len() < small.len() {
        return false;
    }
    // we can later implement a more efficient implementation
    //  since this solution will check every sliding window
    //  knowing something about the pattern, we should be able to skip
    //  a few of the windows
    big.windows(small.len()).any(|a_slice| a_slice == small)
}

/**
given a pattern (slice), returns a vector of the same size as
the slice, where each index position contains the size
of the longest longest border of a prefix of the pattern of that
size
e.g.
given a pattern `abababcaab`
for a prefix of size 1 (i.e. `a`), longest border is 0
for a prefix of size 2 (i.e. `ab`), longest border is 0
for a prefix of size 3 (i.e. `aba`), longest border is 1 (i.e. `a`)
for a prefix of size 4 (i.e. `abab`), longest border is 2 (i.e. `ab`)
for a prefix of size 5 (i.e. `ababa`), longest border is 3 (i.e. `aba`)
for a prefix of size 6 (i.e. `ababab`), longest border is 4 (i.e. `abab`)
for a prefix of size 7 (i.e. `abababc`), longest border is 0
for a prefix of size 8 (i.e. `abababca`), longest border is 1 (i.e. `a`)
for a prefix of size 9 (i.e. `abababcaa`), longest border is 1 (i.e. `a`)
for a prefix of size 9 (i.e. `abababcaab`), longest border is 2 (i.e. `ab`)
so, `calc_prefix` will return the vector [0,0,1,2,3,4,0,1,1,2]

another example:
//    i = 0 1 2 3 4 5 6 7 8 9 10
//  pat = a b a b c a b a b a c
// s(i) = 0 0 1 2 0 1 2 3 4 3 0  // i.e. result of calc_prefix
*/
fn calc_prefix<T: PartialEq>(pat: &[T]) -> Vec<usize> {
    let mut s: Vec<usize> = vec![0; pat.len()];
    let mut border: usize = 0;
    for i in 1..pat.len() {
        while border > 0 && pat[i] != pat[border] {
            border = s[border - 1]
        }
        if pat[i] == pat[border] {
            border = border + 1;
        } else {
            border = 0;
        }
        s[i] = border;
    }
    s
}
