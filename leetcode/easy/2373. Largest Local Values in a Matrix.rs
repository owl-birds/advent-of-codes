impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if grid.len() < 3 || grid[0].len() < 3 {
            return vec![];
        }
        let mut new_mat: Vec<Vec<i32>> = vec![];

        for i in 0..grid.len() - 2 {
            let mut new_row: Vec<i32> = vec![];
            for j in 0..grid[0].len() - 2 {
                let mut max: i32 = 0;
                for k in i..i + 3 {
                    for l in j..j + 3 {
                        if grid[k][l] > max {
                            max = grid[k][l];
                        }
                    }
                }
                new_row.push(max);
            }
            new_mat.push(new_row);
        }
        new_mat
    }
}
