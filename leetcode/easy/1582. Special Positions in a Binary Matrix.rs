impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut count: i32 = 0;

        for r in 0..mat.len() {
            for c in 0..mat[0].len() {
                if mat[r][c] == 0 {
                    continue;
                }
                if Self::is_special(&mat, r, c) {
                    count += 1;
                }
            }
        }

        count
    }
    pub fn is_special(mat: &Vec<Vec<i32>>, r: usize, c: usize) -> bool {
        for i in 0..mat[0].len() {
            if i == c {
                continue;
            }
            if mat[r][i] != 0 {
                return false;
            }
        }
        for i in 0..mat.len() {
            if i == r {
                continue;
            }
            if mat[i][c] != 0 {
                return false;
            }
        }

        true
    }
}
