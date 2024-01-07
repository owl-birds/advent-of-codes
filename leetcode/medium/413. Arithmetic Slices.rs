impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }
        let mut count: i32 = 0;

        for i in 1..(nums.len() - 1) {
            let diff: i32 = nums[i] - nums[i - 1];
            let mut seq_count: i32 = 0;
            for j in (i + 1)..nums.len() {
                let curr_diff: i32 = nums[j] - nums[j - 1];
                if curr_diff != diff {
                    break;
                }
                seq_count += 1;
                if seq_count > 0 {
                    count += 1
                }
            }
        }

        count
    }
}
