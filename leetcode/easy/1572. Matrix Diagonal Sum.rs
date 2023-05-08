impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut i: usize = 0;
        let mut j: usize = mat.len()-1;
        let mut sum: i32 = 0;

        while i < mat.len(){
            sum += mat[i][i] + if i == j {0} else {mat[i][j]};
            println!("{} {}", i, j);
            j -= 1;
            i += 1;
        }

        sum 
    }
}