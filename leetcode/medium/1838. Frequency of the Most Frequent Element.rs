impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut temp_nums: Vec<i32> = nums.to_vec();
        temp_nums.sort_by(|a, b| a.cmp(&b));
        let mut max_freq: i32 = 0;
        let mut curr_sum: i32 = 0;
        let mut left: usize = 0;
        let mut right: usize = 0;

        while right < temp_nums.len() {
            curr_sum += temp_nums[right];
            let mut sum_needed: i32 = temp_nums[right] * (right - left + 1) as i32;

            // decrease the window size : so the k operations can cover all the needed increase needed to make the num in the window the same as the num in the idx right
            while (sum_needed - curr_sum) > k {
                sum_needed -= temp_nums[right];
                curr_sum -= temp_nums[left];
                left += 1;
            }
            // increase the window size ? we dont need to increase the window size because there is k constraint

            max_freq = if ((right - left + 1) as i32) > max_freq {
                (right - left + 1) as i32
            } else {
                max_freq
            };
            right += 1;
        }

        // for i in 0..temp_nums.len() {
        //     let mut temp_k: i32 = k;
        //     let mut idx: usize = i-1;
        //     let mut count_freq: i32 = 1;
        //     while i > 0 && idx >= 0 &&  (temp_nums[i] - temp_nums[idx]) <= temp_k {
        //         temp_k -= (temp_nums[i] - temp_nums[idx]);
        //         count_freq += 1;
        //         if idx == 0 {break}
        //         idx -= 1;
        //     }
        //     // println!("num:{}--freq:{}--temp_k:{}", temp_nums[i], count_freq, temp_k);
        //     if count_freq > max_freq {
        //         max_freq = count_freq;
        //     }
        // }

        max_freq
    }
}
