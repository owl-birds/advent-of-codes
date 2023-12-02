impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let mut temp_mat: Vec<Vec<i32>> = vec![];
        for row in &mat {
            temp_mat.push(row.to_vec());
        }
        for i in 0..4 {
            // println!("{}-{:?}", i, temp_mat);
            if Self::is_equal_matrixs(&temp_mat, &target) {
                return true;
            }
            temp_mat = Self::rotate_90(&temp_mat);
        }

        // TEST
        // println!("{}", Self::is_equal_matrixs(&mat, &mat));
        // println!("{}", Self::is_equal_matrixs(&mat, &target));
        // TEST

        false
    }
    pub fn is_equal_matrixs(mat1: &Vec<Vec<i32>>, mat2: &Vec<Vec<i32>>) -> bool {
        if mat1.len() != mat2.len() {
            return false;
        }
        for i in 0..mat1.len() {
            if mat1[i].len() != mat2[i].len() {
                return false;
            }
            for j in 0..mat1[i].len() {
                if mat1[i][j] != mat2[i][j] {
                    return false;
                }
            }
        }

        true
    }
    pub fn rotate_90(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut new_mat: Vec<Vec<i32>> = vec![];
        for i in 0..matrix.len() {
            new_mat.push(vec![0; matrix[i].len()])
        }
        // println!("{:?}", new_mat);
        let mut c: usize = matrix[0].len() - 1;
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
        // for i in 0..matrix.len() {
        //     for j in 0..matrix[i].len() {
        //         matrix[i][j] = new_mat[i][j];
        //     }
        // }
        new_mat
    }
}
