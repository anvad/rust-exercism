use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna).and_then(|hm| hm.get(&nucleotide).cloned().ok_or(nucleotide))
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    dna.chars().try_fold(
        HashMap::from([('A', 0), ('G', 0), ('C', 0), ('T', 0)]),
        |mut hm, n| {
            if !"AGCT".contains(n) {
                return Err(n);
            }
            hm.entry(n).and_modify(|count| *count += 1);
            Ok(hm)
        },
    )
}
