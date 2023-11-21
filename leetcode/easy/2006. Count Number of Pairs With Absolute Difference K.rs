use std::collections::HashMap;
impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut diff_freq: HashMap<i32, i32> = HashMap::new();
        let mut count: i32 = 0;

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if i32::abs(nums[i] - nums[j]) == k {
                    count += 1;
                }
            }
        }

        count
    }
}
