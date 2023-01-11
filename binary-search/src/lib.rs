pub fn find<T>(array: &[T], key: T) -> Option<usize>
where
    T: PartialOrd,
{
    find_inner(&array, key, 0, array.len())
}

fn find_inner<T>(array: &[T], key: T, start: usize, end: usize) -> Option<usize>
where
    T: PartialOrd,
{
    if start == end {
        return None;
    }
    let mid = start + (end - start) / 2;
    if key == array[mid] {
        return Some(mid);
    } else if key < array[mid] {
        return find_inner(&array, key, start, mid);
    } else {
        return find_inner(&array, key, mid + 1, end);
    }
}
