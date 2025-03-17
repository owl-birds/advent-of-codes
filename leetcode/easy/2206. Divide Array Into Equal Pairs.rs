use std::collections::HashMap;
impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut num_freq: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            if let Some(freq) = num_freq.get_mut(&nums[i]) {
                *freq += 1;
            } else {
                num_freq.insert(nums[i], 1);
            }
        }
        let mut count: i32 = 0;
        for (_, freq) in num_freq.iter() {
            if freq % 2 == 0 {
                count += 1;
            }
        }
        count == num_freq.len() as i32
    }
}
