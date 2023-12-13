impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut row_one: Vec<i32> = vec![];
        let mut row_zero: Vec<i32> = vec![];
        let mut col_one: Vec<i32> = vec![];
        let mut col_zero: Vec<i32> = vec![];

        for r in 0..grid.len() {
            row_one.push(0);
            row_zero.push(0);
            let mut last_idx: usize = row_one.len() - 1;
            for c in 0..grid[0].len() {
                if grid[r][c] == 1 {
                    row_one[last_idx] += 1;
                }
                if grid[r][c] == 0 {
                    row_zero[last_idx] += 1;
                }
            }
        }
        for c in 0..grid[0].len() {
            col_one.push(0);
            col_zero.push(0);
            let mut last_idx: usize = col_one.len() - 1;
            for r in 0..grid.len() {
                if grid[r][c] == 1 {
                    col_one[last_idx] += 1;
                }
                if grid[r][c] == 0 {
                    col_zero[last_idx] += 1;
                }
            }
        }
        for r in 0..grid.len() {
            ans.push(vec![]);
            for c in 0..grid[0].len() {
                ans[r].push((row_one[r] + col_one[c]) - (row_zero[r] + col_zero[c]));
            }
        }
        // // TEST
        // println!("row_one:{:?}", row_one);
        // println!("row_zero:{:?}", row_zero);
        // println!("col_one:{:?}", col_one);
        // println!("col_zero:{:?}", col_zero);
        // // TEST

        ans
    }
}
