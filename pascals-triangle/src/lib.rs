pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut pascals: Vec<Vec<u32>> = vec![];
        if self.0 == 0 {
            return pascals;
        }
        pascals.push(vec![1]);
        if self.0 == 1 {
            return pascals;
        }
        pascals.push(vec![1, 1]);
        if self.0 == 2 {
            return pascals;
        }
        let max_i = self.0 as usize;
        for i in 2..max_i {
            let prev_row = &pascals[i - 1];
            let mut row = vec![1; i + 1];
            let max = i / 2;
            for j in 1..=max {
                row[j] = prev_row[j - 1] + prev_row[j];
                row[i - j] = row[j];
            }
            pascals.push(row);
        }
        pascals
    }
}
