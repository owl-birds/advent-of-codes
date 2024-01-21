impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return -1;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            if nums[0] > nums[1] {
                return nums[0];
            }
            return nums[1];
        }
        let mut dp: Vec<i32> = vec![nums[0], nums[1]];
        let mut max: i32 = -1;
        for i in 2..nums.len() {
            let mut curr_dp = dp[i - 1];
            for j in 0..i - 1 {
                if dp[j] + nums[i] > curr_dp {
                    curr_dp = dp[j] + nums[i];
                }
            }
            if curr_dp > max {
                max = curr_dp
            }
            dp.push(curr_dp);
        }
        // println!("{:?}", dp);
        max
    }
}
