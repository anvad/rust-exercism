use std::{collections::HashSet, fmt::Debug};

fn main() {
    let mut arr = vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1];
    // let mut arr = vec![1, 2, 3, 4];
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
    // p.nxt();
    // println!("first: {:?}", p.arr);
    // p.nxt();
    // println!("secnd: {:?}", p.arr);
    // println!("first: {:?}", p.next());
    let a = ["1", "2"].iter();
    let mut i = 0;
    while p.nxt() {
        // println!("{i}: {:?}", p.arr);
        i += 1;
    }
    p.reset();
    println!("i={i}");
    // for (i, _) in p.enumerate() {
    //     println!("{i}-- {:?}", hs);
    //     // hs.insert(v);
    // }
    // println!("hs len= {}, {}", hs.len(), Permute::new(&arr).count());
    println!("{}", Permute::new(&arr).count());
    i = 0;
    while p.nxt() {
        // println!("{i}: {:?}", p.arr);
        i += 1;
    }
    println!("i={i}");
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
    i: usize,      // store internal index into stack
    // done: bool,    // tells whether iteration has reached end
    arr: Vec<T>, // array of values that we are permuting
}
impl<T: Copy + Debug> Permute<T> {
    pub fn new(arr: &Vec<T>) -> Self {
        let n = arr.len();
        Self {
            c: vec![0; n],
            i: 0,
            // done: false,
            arr: arr.clone(),
        }
    }

    pub fn reset(&mut self) -> () {
        self.c.fill(0);
        self.i = 0;
        // self.done = false;
    }

    // returns true if arr got reshuffled
    pub fn nxt(&mut self) -> bool {
        if self.i == 0 {
            self.i = 1;
            // return Some(());
            return true;
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
                // return Some(());
                return true;
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }
        // self.done = true;
        return false;
    }
}
impl<T: Copy + Debug> Iterator for Permute<T> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            self.i = 1;
            return Some(());
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
                return Some(());
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

// pub fn nxt(&mut self) -> bool {
//     let num = format!("{:b}", self.current);
//     let n = self.arr.len();
//     let final_num: String = "0"
//         .repeat(n - num.len())
//         .chars()
//         .chain(num.chars())
//         .collect();
//     println!("final_num: {final_num}, diff: {:b}", self.diff);

//     if self.current == self.max {
//         return false;
//     }

//     self.current += self.diff;
//     if self.diff > 1 {
//         self.diff = self.diff >> 1;
//     } else {
//         self.diff = 1 << (num.len() - 1);
//     }
//     return true;
// }
