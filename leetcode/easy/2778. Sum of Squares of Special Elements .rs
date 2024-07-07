impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let mut res: i32 = 0;
        for i in 0..nums.len() {
            if nums.len() % (i + 1) == 0 {
                res += (i32::pow(nums[i], 2));
            }
        }
        res
    }
}
