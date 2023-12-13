impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut temp: Vec<i32> = nums.to_vec();
        temp.sort_by(|a, b| a.cmp(&b));

        (temp[temp.len() - 1] - 1) * (temp[temp.len() - 2] - 1)
    }
}
