use std::collections::HashMap;
impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut count_map: HashMap<i32, i32> = HashMap::new();
        for i in 1..nums.len() {
            if nums[i - 1] == key {
                if let Some(count) = count_map.get_mut(&nums[i]) {
                    *count += 1;
                } else {
                    count_map.insert(nums[i], 1);
                }
            }
        }

        let mut target: i32 = -1;
        let mut max_count: i32 = 0;
        for (num, count) in count_map.iter() {
            if *count > max_count {
                max_count = *count;
                target = *num;
            }
        }

        target
    }
}
