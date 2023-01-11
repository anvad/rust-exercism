// from bernardoamc's and gomesalexandre's solutions

// Edge cases are moved to their own array
const EDGE_CASES: [&'static str; 3] = [
	"No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n",
	"1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n",
	"2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"
];
// Returns a verse using pattern matching for edge cases, with a _ catch-all for the rest
pub fn verse(n: u32) -> String {
    match n {
		0 => EDGE_CASES[0].to_string(),
		1 => EDGE_CASES[1].to_string(),
		2 => EDGE_CASES[2].to_string(),
		_ => format!("{number_of_beers} bottles of beer on the wall, {number_of_beers} bottles of beer.\nTake one down and pass it around, {number_of_beers_minus_one} bottles of beer on the wall.\n", number_of_beers = n, number_of_beers_minus_one = n -1)
	}
}
pub fn sing(start: u32, end: u32) -> String {
    // Implicit return! :D
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<_>>() // Collecting object into a vector of reference `str`s so we can call join method
        .join("\n")
}
