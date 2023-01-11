#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct WordCount(char, u32);
fn main() {
    let a = vec![1, 2, 3];
    let mut b = vec![2, 1, 3];
    b.sort_unstable();
    assert_eq!(a, b);
    println!("a == b ? {}", a == b);

    let a = vec![WordCount('a', 2), WordCount('b', 3)];
    let mut b = vec![WordCount('b', 3), WordCount('a', 2)];
    b.sort_unstable();
    assert_eq!(a, b);

    let mut a = "heLlo".to_lowercase().chars().collect::<Vec<char>>();
    a.sort_unstable();
    let mut b = "holel".to_lowercase().chars().collect::<Vec<char>>();
    b.sort_unstable();
    println!("chars: {:?}", a);
    assert_ne!(a, b);
}
