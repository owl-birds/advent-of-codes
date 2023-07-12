impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() < 2 {
            return false;
        }

        // TLE TOO
        let mut prefix_sum: Vec<i32> = vec![];
        let mut temp_sum: i32 = 0;
        for i in 0..nums.len() {
            temp_sum += nums[i];
            prefix_sum.push(temp_sum);
        }
        // println!("{:?}", prefix_sum);
        for i in 0..prefix_sum.len() {
            let mut idx: usize = 0;
            if i > 0 && prefix_sum[i] % k == 0 {
                return true;
            }
            while idx < i {
                if (i - idx) >= 2 && (prefix_sum[i] - prefix_sum[idx]) % k == 0 {
                    return true;
                }
                idx += 1;
            }
        }
        false

        // // SLOW WAY | TLE
        // let mut starting_length: usize = 2;

        // while starting_length <= nums.len() {
        //     let mut sum: i32 = 0;
        //     for i in 0..starting_length as usize {
        //         sum += nums[i];
        //     }
        //     if sum % k == 0 {return true}
        //     let mut idx: usize = starting_length;
        //     while idx < nums.len() {
        //         sum += nums[idx];
        //         sum -= nums[idx-starting_length];
        //         if sum % k == 0 {return true}
        //         idx += 1;
        //     }
        //     // println!("{}", starting_length);
        //     starting_length += 1;
        // }
        // false
    }
}
