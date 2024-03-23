use std::collections::HashMap;
impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut num_freq: HashMap<i32, i32> = HashMap::new();
        let mut count: i32 = 0;
        let mut max_freq: i32 = 1;
        for i in 0..nums.len() {
            if let Some(freq) = num_freq.get_mut(&nums[i]) {
                *freq += 1;
                if *freq > max_freq {
                    max_freq = *freq;
                }
            } else {
                num_freq.insert(nums[i], 1);
            }
        }
        for (_, freq) in num_freq.iter() {
            if *freq == max_freq {
                count += 1
            }
        }
        count * max_freq
    }
}
