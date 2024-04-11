use std::collections::HashSet;
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums_set: HashSet<i32> = HashSet::new();
        let mut ans: i32 = -1;
        let mut max: i32 = 0;
        for i in 0..nums.len() {
            nums_set.insert(nums[i]);
            if max < nums[i] {
                max = nums[i]
            }
        }

        let mut num: i32 = 1;
        while num <= max {
            if !nums_set.contains(&num) {
                ans = num;
                break;
            }
            num += 1;
        }
        if ans == -1 {
            return max + 1;
        }
        ans
    }
}
