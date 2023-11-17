impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut temp_nums: Vec<i32> = nums.to_vec();
        let mut max: i32 = i32::MIN;

        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;

        temp_nums.sort_by(|a, b| a.cmp(&b));

        while left < right {
            if (temp_nums[left] + temp_nums[right]) > max {
                max = temp_nums[left] + temp_nums[right];
            }

            left += 1;
            right -= 1;
        }

        max
    }
}
