static SOUNDS: [(u32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: u32) -> String {
    let raindrops = SOUNDS
        .iter()
        .fold(Vec::<&str>::new(), |mut sounds, &(divisor, sound)| {
            if n % divisor == 0 {
                sounds.push(sound);
            }
            sounds
        });
    match raindrops.len() {
        0 => n.to_string(),
        _ => raindrops.join(""),
    }
}
