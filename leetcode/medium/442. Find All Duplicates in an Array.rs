use std::collections::HashSet;
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut nums_set: HashSet<i32> = HashSet::new();

        for i in 0..nums.len() {
            if nums_set.contains(&nums[i]) {
                result.push(nums[i]);
            }
            nums_set.insert(nums[i]);
        }

        result
    }
}
