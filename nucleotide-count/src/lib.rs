// i was already using `ok_or` in the `count()` fn, but did not think to apply
//  the same pattern in `nucleotide_counts()` fn till i saw jtmueller's solution.
// i need a lot more practice before these idioms come naturally.

use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna).and_then(|hm| hm.get(&nucleotide).cloned().ok_or(nucleotide))
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    dna.chars().try_fold(
        HashMap::from([('A', 0), ('G', 0), ('C', 0), ('T', 0)]),
        |mut hm, n| {
            hm.get_mut(&n).map(|c| *c += 1).ok_or(n)?;
            Ok(hm)
        },
    )
}
