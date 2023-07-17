use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut window_map: HashMap<i32, i32> = HashMap::new();
        if nums.len() < k as usize {
            return 0;
        }
        let mut max_sum: i64 = 0;
        let mut temp_sum: i64 = 0;
        let mut idx: usize = 0;
        while idx < k as usize {
            if let Some(freq) = window_map.get_mut(&nums[idx]) {
                *freq += 1;
            } else {
                window_map.insert(nums[idx], 1);
            }
            temp_sum += nums[idx] as i64;
            idx += 1;
        }
        // println!("window_map:{:?},temp_sum:{}", window_map, temp_sum);
        if k as usize == window_map.len() && max_sum < temp_sum {
            max_sum = temp_sum
        }
        while idx < nums.len() {
            if let Some(freq) = window_map.get_mut(&nums[idx]) {
                *freq += 1;
            } else {
                window_map.insert(nums[idx], 1);
            }
            let mut is_zero: bool = false;
            if let Some(freq) = window_map.get_mut(&nums[idx - k as usize]) {
                *freq -= 1;
                if *freq == 0 {
                    is_zero = true;
                }
            }
            if is_zero {
                window_map.remove(&nums[idx - k as usize]);
            }
            temp_sum += nums[idx] as i64;
            temp_sum -= nums[idx - k as usize] as i64;
            if window_map.len() == k as usize && max_sum < temp_sum {
                max_sum = temp_sum
            }
            idx += 1;
        }

        max_sum
    }
}
