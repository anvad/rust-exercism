// copying kstep's solution. Brilliant!

// although, this has more iterations than my solution
//  i.e. we look at all the neighbours for each cell
//  so, same cell is hit multiple times
// my solution started with all zeros and only updates the neighbours
//  of cells that contain mines

#[rustfmt::skip]
static NEIGHBOURHOOD_OFFSETS: &[(i32,i32)] = &[
    (-1,-1), (-1,0), (-1,1),
    ( 0,-1),         ( 0,1),
    ( 1,-1), ( 1,0), ( 1,1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len() as i32;
    (0..height)
        .map(|y| {
            let width = minefield[y as usize].len() as i32;
            (0..width)
                .map(|x| {
                    if minefield[y as usize].as_bytes()[x as usize] == b'*' {
                        '*'
                    } else {
                        match NEIGHBOURHOOD_OFFSETS
                            .iter()
                            .map(|&(oy, ox)| (y + oy, x + ox))
                            .filter(|&(y, x)| y >= 0 && y < height && x >= 0 && x < width)
                            .filter(|&(y, x)| minefield[y as usize].as_bytes()[x as usize] == b'*')
                            .count()
                        {
                            0 => ' ',
                            c => (c as u8 + '0' as u8) as char,
                        }
                    }
                })
                .collect()
        })
        .collect()
}
