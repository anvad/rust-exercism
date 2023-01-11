use std::io::Read;

fn main() {
    let a = "hello";
    let b = a.get(1..1 + 1); //.unwrap_or("err");
    println!("b={:?}", b);
    // let mut s: String = "0".repeat(5);
    // println!("s (before): {s}");
    // // s.as_bytes_mut()[1] = b'1';
    let v: Vec<u8> = vec![];
    // v.iter().take(5).skip(n)
}
