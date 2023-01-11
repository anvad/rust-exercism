use std::{
    collections::HashSet,
    fmt::{format, Debug},
    slice::Iter,
};

fn main() {
    // let mut arr = vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1];
    let mut arr = vec![1, 2, 3, 4, 5];
    // let mut arr = vec!['A', 'B', 'C', 'D', 'E'];
    println!("arr[0]= {}", arr[0]);
    let max = factorial(arr.len() as u32);
    let mut hs = HashSet::<Vec<i32>>::new();
    let mut p = Permute::new(&arr[0..3]);
    let mut i = 0;
    while p.nxt() {
        println!("{i}: {:?}", p.arr);
        i += 1;
    }
    p.reset();
    println!("i={i}");
    println!("{}", Permute::new(&arr).count());
    i = 0;
    while p.nxt() {
        // println!("{i}: {:?}", p.arr);
        i += 1;
    }
    println!("i={i}");

    let mut c = Combine::new(&arr, 2);
    i = 0;
    while c.nxt() {
        println!("{i}: {:?}", c.get());
        // println!("{:?}", c.get());
        i += 1;
    }
    println!("comb i={i}");
    // combinations(3, &arr);
}

fn factorial(num: u32) -> u32 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}

struct Combine<T: Copy + Debug> {
    r: usize,
    arr: Vec<T>,
    sel_ind: Vec<usize>,
    selected: Vec<T>,
}

// see https://cp-algorithms.com/combinatorics/generating_combinations.html
impl<T: Copy + Debug> Combine<T> {
    pub fn new(arr: &Vec<T>, r: usize) -> Self {
        let mut selected: Vec<T> = vec![];
        for item in &arr[0..r] {
            selected.push(item.clone());
        }
        Self {
            r,
            arr: arr.clone(),
            sel_ind: vec![0; r],
            selected,
        }
    }

    pub fn get(&mut self) -> &[T] {
        for i in 0..self.r {
            self.selected[i] = self.arr[self.sel_ind[i] - 1];
        }
        &self.selected[..]
    }

    pub fn nxt(&mut self) -> bool {
        let n = self.arr.len();
        let k = self.r;
        for i in (0..k).rev() {
            if self.sel_ind[i] < n - k + i + 1 {
                self.sel_ind[i] += 1;
                for j in (i + 1)..k {
                    self.sel_ind[j] = self.sel_ind[j - 1] + 1;
                }
                if self.sel_ind[0] > 0 {
                    return true;
                }
            }
        }
        return false;
    }
}

// working implementation of iterator that clones
struct Permute<T: Copy + Debug> {
    c: Vec<usize>, // store internal stack state
    i: usize,      // store internal index into stack
    arr: Vec<T>,   // array of values that we are permuting
}
impl<T: Copy + Debug> Permute<T> {
    pub fn new(arr: &[T]) -> Self {
        let n = arr.len();
        Self {
            c: vec![0; n],
            i: 0,
            // done: false,
            arr: arr.iter().copied().collect(),
        }
    }

    pub fn reset(&mut self) -> () {
        self.c.fill(0);
        self.i = 0;
    }

    // returns true if arr got reshuffled
    pub fn nxt(&mut self) -> bool {
        if self.i == 0 {
            self.i = 1;
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
                return true;
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }
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
