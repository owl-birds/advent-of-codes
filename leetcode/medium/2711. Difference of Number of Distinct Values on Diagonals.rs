use std::collections::HashSet;
impl Solution {
    pub fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        // println!("{:?}", Self::count_diag_distinct(&grid, &(0 as usize), &(0 as usize)));
        for r in 0..grid.len() {
            let mut temp: Vec<i32> = vec![];
            for c in 0..grid[0].len() {
                let distinct_diag: [i32; 2] = Self::count_diag_distinct(&grid, &r, &c);
                temp.push(i32::abs(distinct_diag[0] - distinct_diag[1]));
            }
            ans.push(temp);
        }
        ans
    }
    pub fn count_diag_distinct(mat: &Vec<Vec<i32>>, r: &usize, c: &usize) -> [i32; 2] {
        // let mut count: i32 = 0;
        let mut temp_r: i32 = *r as i32;
        let mut temp_c: i32 = *c as i32;
        let mut diag: [i32; 2] = [0, 0];
        let mut unique: HashSet<i32> = HashSet::new();
        while temp_r > 0 && temp_c > 0 {
            temp_r -= 1;
            temp_c -= 1;
            if temp_r < 0 || temp_c < 0 {
                break;
            }
            unique.insert(mat[temp_r as usize][temp_c as usize]);
        }
        diag[0] = unique.len() as i32;
        let mut unique: HashSet<i32> = HashSet::new();
        let mut temp_r: usize = *r;
        let mut temp_c: usize = *c;
        while temp_r < mat.len() && temp_c < mat[0].len() {
            temp_r += 1;
            temp_c += 1;
            if temp_r >= mat.len() || temp_c >= mat[0].len() {
                break;
            }
            unique.insert(mat[temp_r][temp_c]);
        }
        diag[1] = unique.len() as i32;
        diag
    }
}
