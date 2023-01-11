pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => 2u64.pow(s - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    // u64::MAX is the same as 2u64.pow(64) - 1
    //  i could not do the calc as that overflows
    //  i could do 2u64.pow(63) - 1 + 2u64.pow(63)
    //  but that is verbose and slow
    u64::MAX
}
