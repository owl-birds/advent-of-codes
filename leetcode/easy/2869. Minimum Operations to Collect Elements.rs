use std::collections::HashSet;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut count: i32 = 0;
        let mut set: HashSet<i32> = HashSet::new();
        let mut i: usize = nums.len();

        while i > 0 {
            i -= 1;
            if set.len() as i32 >= k {
                break;
            }
            if nums[i] <= k {
                set.insert(nums[i]);
            }
            count += 1;
        }

        count
    }
}
