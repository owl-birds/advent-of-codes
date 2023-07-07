impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max: i32 = 0;

        let mut idx: usize = 0;

        while idx < nums.len() {
            let mut temp_count: i32 = 0;
            while idx < nums.len() && nums[idx] == 1 {
                temp_count += 1;
                idx += 1;
            }
            if temp_count == 0 {
                idx += 1;
                continue;
            }
            if temp_count > max {
                max = temp_count;
            }
        }

        max
    }
}
