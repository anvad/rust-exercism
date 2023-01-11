use std::cmp::min;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mine = '*' as u8; // "*".as_bytes()[0];
    let zero = '0' as u8; // "0".as_bytes()[0];

    let max_i = minefield.len();
    let max_j = minefield.get(0).unwrap_or(&"").len();

    let mut solved: Vec<Vec<u8>> = vec![vec![zero; max_j]; max_i];
    for (i, row) in minefield.iter().enumerate() {
        for (j, ch) in row.as_bytes().iter().enumerate() {
            if *ch == mine {
                solved[i][j] = mine;
                // now update the neighbouring non-mine cells
                let (imin, jmin) = match (i, j) {
                    (0, 0) => (0, 0),
                    (_, 0) => (i - 1, 0),
                    (0, _) => (0, j - 1),
                    _ => (i - 1, j - 1),
                };
                for ii in imin..min(max_i, i + 2) {
                    for jj in jmin..min(max_j, j + 2) {
                        if solved[ii][jj] != mine {
                            solved[ii][jj] += 1;
                        }
                    }
                }
            }
        }
    }

    // now convert solved to Vec<String>
    solved
        .iter()
        .map(|row| {
            row.iter()
                .map(|ch| if *ch == zero { ' ' } else { *ch as char })
                .collect::<String>()
        })
        .collect()
}
