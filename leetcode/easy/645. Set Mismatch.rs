use std::collections::HashMap;
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut real_nums: Vec<i32> = vec![];
        let mut num_freq: HashMap<i32, i32> = HashMap::new();
        let mut result: Vec<i32> = vec![];
        for n in 1..(nums.len() + 1) as i32 {
            real_nums.push(n);
            if let Some(freq) = num_freq.get_mut(&nums[(n - 1) as usize]) {
                *freq += 1;
                result.push(nums[(n - 1) as usize]);
            } else {
                num_freq.insert(nums[(n - 1) as usize], 1);
            }
        }
        // println!("{:?}", num_freq);
        for n in &real_nums {
            if num_freq.contains_key(&n) {
                continue;
            }
            result.push(*n);
        }

        result
    }
}
