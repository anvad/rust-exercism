// copying kstep's solution. Brilliant!

// although, this will have more iterations than my solution if the
//  field has few (sparse) mines
//  since kstep's solution looks at all the neighbours for non-mine cell
//  while my solution only updates the neighbours of cells that contain mines
// can i have best of both solutions?

// on the other hand, my solution needs twice the memory
//  since we store a 2-d array of chars and then create
//  an array of strings

#[rustfmt::skip]
static NEIGHBOURHOOD_OFFSETS: &[(i32,i32)] = &[
    (-1,-1), (-1,0), (-1,1),
    ( 0,-1),         ( 0,1),
    ( 1,-1), ( 1,0), ( 1,1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mine = '*' as u8;

    let max_i = minefield.len();
    let max_j = minefield.get(0).unwrap_or(&"").len();

    let mut solved: Vec<Vec<u8>> = vec![vec![b'0'; max_j]; max_i];
    for (i, row) in minefield.iter().enumerate() {
        for (j, ch) in row.as_bytes().iter().enumerate() {
            if *ch == mine {
                solved[i][j] = mine;
                // now update the neighbouring cells
                NEIGHBOURHOOD_OFFSETS
                    .iter()
                    .map(|&(oi, oj)| (i as i32 + oi, j as i32 + oj))
                    .filter(|&(ni, nj)| {
                        ni >= 0 && (ni as usize) < max_i && nj >= 0 && (nj as usize) < max_j
                    })
                    .filter(|&(ni, nj)| minefield[ni as usize].as_bytes()[nj as usize] != mine)
                    .for_each(|(ni, nj)| {
                        solved[ni as usize][nj as usize] += 1;
                    });
            }
        }
    }

    // now convert solved to Vec<String>
    solved
        .iter()
        .map(|row| {
            row.iter()
                .map(|ch| if *ch == b'0' { ' ' } else { *ch as char })
                .collect::<String>()
        })
        .collect()
}
