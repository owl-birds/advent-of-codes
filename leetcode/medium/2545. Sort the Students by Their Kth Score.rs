impl Solution {
    pub fn sort_the_students(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        for i in 0..score.len() {
            result.push(score[i].to_vec());
        }
        result.sort_by(|a, b| b[k as usize].cmp(&a[k as usize]));
        result
    }
}
