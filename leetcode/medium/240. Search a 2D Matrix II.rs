impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for row in matrix {
            let mut l: usize = 0;
            let mut r: usize = row.len();
            while l < r {
                let m: usize = (l + r) / 2;
                if row[m] == target {
                    return true;
                }
                if row[m] > target {
                    r = m;
                    continue;
                }
                l = m + 1;
            }
        }

        false
    }
}
