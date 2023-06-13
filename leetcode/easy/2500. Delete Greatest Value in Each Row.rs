impl Solution {
    pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
        let mut sum_max: i32 = 0;
        let mut temp_grid: Vec<Vec<i32>> = vec![];

        for row in grid {
            let mut temp_row = row.clone();
            temp_row.sort_by(|a, b| b.cmp(a));
            temp_grid.push(temp_row);
        }
        println!("{:?}", temp_grid);
        for c in 0..temp_grid[0].len() {
            let mut max_num_row: Vec<i32> = vec![];
            for r in 0..temp_grid.len() {
                max_num_row.push(temp_grid[r][c]);
            }
            sum_max += (*max_num_row.iter().max().unwrap());
        }
        sum_max
    }
}
