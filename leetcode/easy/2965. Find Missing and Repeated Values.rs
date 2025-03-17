use std::collections::HashSet;
impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut min: i32 = 1;
        let mut max: i32 = (grid[0].len() * grid[1].len()) as i32;
        let mut ans: Vec<i32> = vec![-1, -1];
        let mut num_set: HashSet<i32> = HashSet::new();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if num_set.contains(&grid[i][j]) {
                    ans[0] = grid[i][j];
                } else {
                    num_set.insert(grid[i][j]);
                }
            }
        }
        while min <= max {
            if !num_set.contains(&min) {
                ans[1] = min;
                break;
            }
            min += 1;
        }
        ans
    }
}
