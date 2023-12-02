impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut new_mat: Vec<Vec<i32>> = vec![];
        for i in 0..matrix.len() {
            new_mat.push(vec![0; matrix[i].len()])
        }
        // println!("{:?}", new_mat);
        let mut c: usize = matrix[0].len()-1;
        let mut r: usize = 0;
        while r < matrix.len() {
            let mut temp_r: usize = 0;
            let mut old_c: usize = 0;
            while old_c < matrix[r].len() {
                new_mat[temp_r][c] = matrix[r][old_c];
                temp_r += 1;
                old_c += 1;
            }
            c -= 1;
            r += 1;
        }
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                matrix[i][j] = new_mat[i][j];
            }
        }
        // println!("{:?}", new_mat);
    }
}