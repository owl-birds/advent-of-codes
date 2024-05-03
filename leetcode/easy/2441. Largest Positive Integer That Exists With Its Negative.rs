use std::collections::HashSet;
impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut nums_set: HashSet<i32> = HashSet::new();
        let mut largest: i32 = -1;

        for i in 0..nums.len() {
            nums_set.insert(nums[i]);
            if nums[i] < 0 {
                let temp = nums[i] * -1;
                if nums_set.contains(&temp) && temp > largest {
                    largest = temp;
                }
            } else if nums[i] > 0 {
                let temp = nums[i] * -1;
                if nums_set.contains(&temp) && nums[i] > largest {
                    largest = nums[i];
                }
            }
        }

        largest
    }
}
