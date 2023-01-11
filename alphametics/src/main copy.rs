struct Permute<'a, T>
where
    T: Copy + 'a,
{
    max: u32,
    current: u32,
    arr: Vec<&'a T>,
}

fn main() {
    let mut arr = vec![1, 2, 3, 4];
    println!("{:?}", arr);
    println!(" 4! = {}", factorial(4));
    println!(" 7! = {}", factorial(7));
    println!("10! = {}", factorial(10));
    for i in 0..24 {
        generate(arr.len(), &mut arr);
        println!("{i} -  {:?}", arr);
    }

    let a = vec![&1, &2, &3, &4];
    let pa = Permute::<i32>::new(&a);
}

impl<'a, T: Copy> Permute<'a, T> {
    pub fn new(arr: &Vec<&'a T>) -> Self {
        Self {
            max: factorial(arr.len() as u32),
            current: 0,
            arr: arr.clone(),
        }
    }
}

impl<'a, T: Copy> Iterator for Permute<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.max {
            return None;
        }
        self.current += 1;
        generate(self.arr.len(), &mut self.arr);
        Some(self.arr.clone())
    }
}
fn factorial(num: u32) -> u32 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}
// https://en.wikipedia.org/wiki/Heap%27s_algorithm
fn generate<T: Copy>(k: usize, arr: &mut Vec<T>) {
    if k == 1 {
        return;
    }
    // Generate permutations with k-th unaltered
    // Initially k = length(arr)
    generate(k - 1, arr);

    // Generate permutations for k-th swapped with each k-1 initial
    for i in 0..(k - 1) {
        // Swap choice dependent on parity of k (even or odd)
        if k % 2 == 0 {
            // swap(A[i], A[k-1]) // zero-indexed, the k-th is at k-1
            (arr[i], arr[k - 1]) = (arr[k - 1], arr[i]);
        } else {
            // swap(A[0], A[k-1])
            (arr[0], arr[k - 1]) = (arr[k - 1], arr[0]);
        }
        generate(k - 1, arr);
    }
}
