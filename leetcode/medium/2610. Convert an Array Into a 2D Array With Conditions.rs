use std::collections::HashMap;
impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums_freq: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            if let Some(freq) = nums_freq.get_mut(&nums[i]) {
                *freq += 1;
                continue;
            }
            nums_freq.insert(nums[i], 1);
        }
        let mut result: Vec<Vec<i32>> = vec![];
        while nums_freq.len() != 0 {
            let mut num_soon_deleted: Vec<i32> = vec![];
            let mut temp_vec: Vec<i32> = vec![];
            for (n, f) in nums_freq.iter_mut() {
                if *f == 0 {
                    num_soon_deleted.push(*n);
                    continue;
                }
                temp_vec.push(*n);
                *f -= 1;
            }
            for i in 0..num_soon_deleted.len() {
                nums_freq.remove(&num_soon_deleted[i]);
            }
            if nums_freq.len() == 0 {
                break;
            }
            result.push(temp_vec);
        }

        result
    }
}
