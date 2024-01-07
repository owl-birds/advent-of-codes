impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut dp: Vec<i32> = vec![];
        let mut longest: i32 = 1;

        for i in 0..nums.len() {
            let mut max_dp: i32 = 1;
            if i > 0 {
                for j in 0..i {
                    if nums[i] > nums[j] {
                        max_dp = if (dp[j] + 1) > max_dp {
                            dp[j] + 1
                        } else {
                            max_dp
                        };
                    }
                }
            }
            dp.push(max_dp);
            if max_dp > longest {
                longest = max_dp
            }
            // println!("{:?}", dp);
        }
        // println!("{:?}", dp);

        // idea for optimizations : Binary Search

        longest
    }
}
