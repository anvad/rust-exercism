use std::{collections::HashSet, fmt::Debug};

fn main() {
    let mut arr = vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1];
    let max = factorial(arr.len() as u32);
    let mut hs = HashSet::<Vec<i32>>::new();
    // for i in 0..max {
    //     generate(arr.len(), &mut arr);
    //     println!("{i} -  {:?}", arr);
    //     hs.insert(arr.clone());
    // }
    // generate(arr.len(), &mut arr);
    // println!("hs={} {:?}", hs.len(), hs);
    let mut p = Permute::new(&arr);
    // println!("first: {:?}", p.next());
    // for (i, v) in p.enumerate() {
    //     // println!("{i}-- {:?}", v);
    //     hs.insert(v);
    // }
    // println!("hs len= {}, {}", hs.len(), Permute::new(&arr).count());
    println!("{}", Permute::new(&arr).count());
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

// working implementation of iterator that clones
struct Permute<T: Copy + Debug> {
    c: Vec<usize>, // store internal stack state
    i: usize,      // index into stack
    arr: Vec<T>,   // array of values that we are permuting
}
impl<T: Copy + Debug> Permute<T> {
    pub fn new(arr: &Vec<T>) -> Self {
        let n = arr.len();
        Self {
            c: vec![0; n],
            i: 0,
            arr: arr.clone(),
        }
    }
}
impl<T: Copy + Debug> Iterator for Permute<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            self.i = 1;
            return Some(self.arr.clone());
        }
        let n = self.arr.len();
        while self.i < n {
            if self.c[self.i] < self.i {
                if self.i % 2 == 0 {
                    // swap arr_0 and arr_i
                    (self.arr[0], self.arr[self.i]) = (self.arr[self.i], self.arr[0]);
                } else {
                    // swap arr_ci and arr_0
                    (self.arr[self.c[self.i]], self.arr[self.i]) =
                        (self.arr[self.i], self.arr[self.c[self.i]]);
                }
                self.c[self.i] += 1;
                self.i = 1;
                return Some(self.arr.clone());
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }
        return None;
    }
}

fn generate(n: usize, arr: &mut Vec<u8>) {
    // c is an encoding of the stack state. c[k] encodes the for-loop counter for when generate(k - 1, A) is called
    let mut c = vec![0; n];

    println!("1: {:?}", arr);

    // i acts similarly to a stack pointer
    let mut i = 1;
    while i < n {
        if c[i] < i {
            if i % 2 == 0 {
                // swap(A[0], A[i])
                (arr[0], arr[i]) = (arr[i], arr[0]);
            } else {
                // swap(A[c[i]], A[i])
                (arr[c[i]], arr[i]) = (arr[i], arr[c[i]]);
            }
            println!("2: {:?}", arr);
            // Swap has occurred ending the for-loop. Simulate the increment of the for-loop counter
            c[i] += 1;
            // Simulate recursive call reaching the base case by bringing the pointer to the base case analog in the array
            i = 1;
        } else {
            // Calling generate(i+1, A) has ended as the for-loop terminated. Reset the state and simulate popping the stack by incrementing the pointer.
            c[i] = 0;
            i += 1;
        }
    }
}
