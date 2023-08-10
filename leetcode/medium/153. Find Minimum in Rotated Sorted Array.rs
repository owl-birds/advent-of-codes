impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        // HAHAHHAHAHAHAHAHA
        // *nums.iter().min().unwrap()
        // HAHAHHAHAHAHAHAHA

        if nums[0] > nums[nums.len() - 1] {
            let mut sepr_idx: usize = 0;
            while sepr_idx < nums.len() {
                sepr_idx += 1;
                if sepr_idx < nums.len() && nums[sepr_idx] < nums[sepr_idx - 1] {
                    return nums[sepr_idx];
                }
            }
        }

        nums[0]
    }
}
