// looked at hansrodtang's solution elegant solution.
// in there, i found rsalmei's comment about using u32 to map the 26 chars of ascii alphabet
//  i still think hansrodtang's solution is more readable and
//  more general since it can handle most non-ascii chars too,
//  whereas this solution (developed from rsalmei's hints) only works for ascii strings.
pub fn check(candidate: &str) -> bool {
    match candidate
        .chars()
        .filter(|&ch| ch != '-' && ch != ' ')
        .flat_map(|ch| ch.to_lowercase())
        .map(|ch| 2u32.pow(((ch as u8) - b'a') as u32))
        .try_fold(0, |sum, n| if sum & n == 0 { Some(sum + n) } else { None })
    {
        Some(_) => true,
        _ => false,
    }
}
