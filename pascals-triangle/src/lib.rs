// bupknar's solution is recursive (new triangle is old triangle + 1 more row) with zip
// jan-schreib's solution directly calculates nth row, so it can be used to generate a particular row without calculating all the previous rows

pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        // we hardcode the first two rows to avoid checking for underflow of u32 in each loop
        match self.0 {
            0 => vec![],
            1 => vec![vec![1]],
            2 => vec![vec![1], vec![1, 1]],
            _ => {
                let mut pascals: Vec<Vec<u32>> = vec![vec![1], vec![1, 1]];
                pascals.reserve(self.0 as usize - 2); // minimize re-sizing
                let max_i = self.0 as usize;
                for i in 2..max_i {
                    let prev_row = &pascals[i - 1];
                    let mut row = vec![1; i + 1];

                    // since triangle is symmetric, we only need to loop till half the row
                    let max_j = i / 2;
                    for j in 1..=max_j {
                        row[j] = prev_row[j - 1] + prev_row[j];
                        row[i - j] = row[j];
                    }
                    pascals.push(row);
                }
                pascals
            }
        }
    }
}
