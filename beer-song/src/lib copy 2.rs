pub fn verse(n: u32) -> String {
    let verse = (
        "of beer on the wall",
        "of beer.\nTake",
        "down and pass it around,",
    );
    match n {
        0 => format!("No more bottles {}, no more bottles of beer.\nGo to the store and buy some more, 99 bottles {}.\n", verse.0, verse.0),
        1 => format!("1 bottle {}, 1 bottle {} it {} no more bottles of beer on the wall.\n", verse.0, verse.1, verse.2),
        2 => format!("{n} bottles {}, {n} bottles {} one {} {} bottle {}.\n", verse.0, verse.1, verse.2, 1, verse.0),
        _ => format!("{n} bottles {}, {n} bottles {} one {} {} bottles {}.\n", verse.0, verse.1, verse.2, n-1, verse.0),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..start + 1)
        .into_iter()
        .rev()
        .map(|n| verse(n))
        .collect::<Vec<_>>()
        .join("\n")
}
