use std::collections::HashSet;
impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        // num appear either
        // once or twice
        let mut res: i32 = 0;
        let mut num_set: HashSet<i32> = HashSet::new();

        for i in 0..nums.len() {
            if num_set.contains(&nums[i]) {
                res ^= nums[i];
                continue;
            }
            num_set.insert(nums[i]);
        }

        res
    }
}
