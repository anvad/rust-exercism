pub fn raindrops(n: u32) -> String {
    let mut raindrops = Vec::<&str>::new();
    if n % 3 == 0 {
        raindrops.push("Pling")
    }
    if n % 5 == 0 {
        raindrops.push("Plang")
    }
    if n % 7 == 0 {
        raindrops.push("Plong")
    }
    if raindrops.len() == 0 {
        return n.to_string();
    }
    raindrops.join("")
}
