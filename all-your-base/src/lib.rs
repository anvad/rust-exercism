#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///

// here's dangets's solution copied below with slight mods
//  love how they abstracted the tasks into well named functions
//  and also moved the error check into each function.
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    convert_from_base(number, from_base).and_then(|num| convert_to_base(num, to_base))
}
fn convert_from_base(number: &[u32], base: u32) -> Result<u32, Error> {
    require(base >= 2, Error::InvalidInputBase)?;
    number.iter().try_fold(0, |num, &digit| {
        require(digit < base, Error::InvalidDigit(digit))?;
        Ok(num * base + digit)
    })
}
fn convert_to_base(mut num: u32, base: u32) -> Result<Vec<u32>, Error> {
    require(base >= 2, Error::InvalidOutputBase)?;
    let mut ret = vec![];
    while num >= base {
        ret.push(num % base);
        num /= base;
    }
    ret.push(num);
    ret.reverse();
    Ok(ret)
}
#[rustfmt::skip]
fn require<T>(predicate: bool, error: T) -> Result<(), T> {
    if predicate { Ok(()) } else { Err(error) }
}
