impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];

        for c in 0..matrix[0].len() {
            ans.push(vec![]);
            for r in 0..matrix.len() {
                ans[c].push(matrix[r][c]);
            }
        }

        ans
    }
}
