impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut count: i32 = 0;

        for i in 0..grid.len() {
            let mut idx: i32 = (grid[i].len() - 1) as i32;
            while idx >= 0 && grid[i][idx as usize] < 0 {
                count += 1;
                idx -= 1;
            }
        }

        count
    }
}
