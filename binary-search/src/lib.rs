pub fn find<S, T>(array: S, key: T) -> Option<usize>
where
    T: PartialOrd,
    S: AsRef<[T]>,
{
    find_inner(&array, key, 0, array.as_ref().len())
}

fn find_inner<S, T>(array: S, key: T, start: usize, end: usize) -> Option<usize>
where
    T: PartialOrd,
    S: AsRef<[T]>,
{
    if start == end {
        return None;
    }
    let array = array.as_ref();
    let mid = start + (end - start) / 2;
    if key == array[mid] {
        return Some(mid);
    } else if key < array[mid] {
        return find_inner(array, key, start, mid);
    } else {
        return find_inner(array, key, mid + 1, end);
    }
}
