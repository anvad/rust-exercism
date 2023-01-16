// iamhere2's solution. This is brilliant!
// my original solution was creating a vector of digits and then comparing it to the reverse.
// iamhere2's solution just stores the reverse as a number - so no need to allocate anything on the heap
// their solution is 10x faster than mine.
// i've copied their solution here, with one small mod that reduced their time in half-
//  on line 46, i am iterating `j` from `i` to `max`, rather than from `min` to `max`.

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);
impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if is_palindrome(value) {
            Some(Palindrome(value))
        } else {
            None
        }
    }
    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
fn is_palindrome(value: u64) -> bool {
    if value % 10 == 0 {
        return false;
    };
    let mut reverse = 0;
    let mut r = value;
    while r > 0 {
        reverse = reverse * 10 + r % 10;
        r /= 10;
    }
    value == reverse
}
pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_p = None;
    let mut max_p = None;
    for i in min..=max {
        if i % 10 == 0 {
            continue;
        };
        for j in i..=max {
            if j % 10 == 0 {
                continue;
            };
            let p = i * j;
            if is_palindrome(p) {
                if p < min_p.unwrap_or(u64::MAX) {
                    min_p = Some(p);
                }
                if p > max_p.unwrap_or(u64::MIN) {
                    max_p = Some(p);
                }
            }
        }
    }
    match (min_p, max_p) {
        (Some(min), Some(max)) => {
            Some((Palindrome::new(min).unwrap(), Palindrome::new(max).unwrap()))
        }
        _ => None,
    }
}
