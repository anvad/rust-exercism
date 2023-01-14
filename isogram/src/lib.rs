// copied rsalmei's solution
//  learnt that bool has a `.then` method to convert bool to Option
//  and realized how i could have just used `is_some()` instead of matching on `Some`!
pub fn check(candidate: &str) -> bool {
    candidate
        .chars()
        .filter(|&ch| ch != '-' && ch != ' ')
        .flat_map(|ch| ch.to_lowercase())
        .map(|ch| 1u32 << (ch as u8) - b'a')
        .try_fold(0, |sum, n| (sum & n == 0).then(|| sum + n))
        .is_some()
}
