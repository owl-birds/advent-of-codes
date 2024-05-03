impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans: i32 = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    continue;
                }
                let pos: [usize; 2] = [i, j];
                let curr_par: i32 = Self::find_parameter(&pos, &grid);
                ans += curr_par;
            }
        }

        ans
    }
    pub fn find_parameter(pos: &[usize; 2], grid: &Vec<Vec<i32>>) -> i32 {
        let mut param: i32 = 0;

        // horizontal
        if pos[1] + 1 >= grid[0].len() || grid[pos[0]][pos[1] + 1] == 0 {
            param += 1;
        }
        if pos[1] == 0 || grid[pos[0]][pos[1] - 1] == 0 {
            param += 1;
        }
        // vertical
        if pos[0] + 1 >= grid.len() || grid[pos[0] + 1][pos[1]] == 0 {
            param += 1;
        }
        if pos[0] == 0 || grid[pos[0] - 1][pos[1]] == 0 {
            param += 1;
        }
        param
    }
}
