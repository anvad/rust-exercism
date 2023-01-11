use std::{collections::HashSet, fmt::Debug};

fn main() {
    let mut arr = vec![1, 2, 3];
    let max = factorial(arr.len() as u32);
    let mut hs = HashSet::<Vec<i32>>::new();
    // for i in 0..max {
    //     generate(arr.len(), &mut arr);
    //     println!("{i} -  {:?}", arr);
    //     hs.insert(arr.clone());
    // }
    generate(arr.len(), &mut arr);
    println!("hs={} {:?}", hs.len(), hs);
    combinations(3, &arr);
}

fn combinations<T: Copy>(r: u32, arr: &Vec<T>) {
    let mut init = (1 << r) - 1u32;
    let n = arr.len() as u32;
    let max = init << (n - r);
    println!("init is: {init}/{}, max is: {max}", (1 << r) - 1);
}

fn factorial(num: u32) -> u32 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}
fn generate<T: Copy + Debug>(k: usize, arr: &mut Vec<T>) {
    if k == 1 {
        println!("{:?}", arr);
        return;
    }
    // Generate permutations with k-th unaltered
    // Initially k = length(arr)
    println!("calling generate {k}");
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
        // println!("{:?}", arr);
    }
}
