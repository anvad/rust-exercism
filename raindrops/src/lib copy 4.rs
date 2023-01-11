// copied erickpintor's solution
//  learnt about statically sized slices
static SOUNDS: [(u32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: u32) -> String {
    match SOUNDS
        .iter()
        .filter(|(divisor, _)| n % *divisor == 0)
        .map(|(_, sound)| *sound)
        .collect::<String>()
    {
        s if s.is_empty() => n.to_string(),
        s => s,
    }
}
