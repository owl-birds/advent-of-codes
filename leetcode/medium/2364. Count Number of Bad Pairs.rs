use std::collections::HashMap;
impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let mut diff_freq: HashMap<i32, i64> = HashMap::new();
        let mut count: i64 = 0;

        for i in 0..nums.len() {
            let diff: i32 = i as i32 - nums[i];
            if let Some(freq) = diff_freq.get_mut(&diff) {
                count += *freq;
                *freq += 1;
            } else {
                diff_freq.insert(diff, 1);
            }
        }
        let mut max_pair: i64 = 0;

        for i in 1..nums.len() as i64 {
            max_pair += i;
        }

        max_pair - count
    }
}
