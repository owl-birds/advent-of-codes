impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut temp: Vec<Vec<i32>> = vec![];
        for i in 0..nums.len() {
            temp.push(vec![nums[i], i as i32]);
        }
        temp.sort_by(|a, b| b[0].cmp(&a[0]));
        let mut temp_result: Vec<Vec<i32>> = vec![];

        for i in 0..k as usize {
            temp_result.push(temp[i].to_vec());
        }
        temp_result.sort_by(|a, b| a[1].cmp(&b[1]));

        let mut result: Vec<i32> = vec![];
        for i in 0..temp_result.len() {
            result.push(temp_result[i][0]);
        }

        result
    }
}
