// denenr's solution is good. Replicating it below
// my copy of his solution initially had a `array.len() == 0` check
//  then, looking back at his solution, i saw how `.get()` returns an Option
//  and we can use `?` to short circuit return
pub fn find<S: AsRef<[T]>, T: Ord>(array: S, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mid = array.len() / 2;
    match key.cmp(array.get(mid)?) {
        std::cmp::Ordering::Equal => Some(mid),
        std::cmp::Ordering::Less => find(&array[..mid], key),
        std::cmp::Ordering::Greater => find(&array[mid + 1..], key).map(|i| mid + 1 + i),
    }
}
