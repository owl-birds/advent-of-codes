use std::collections::HashMap;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // not constant space, but linear complexity
        let mut num_freq: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            if let Some(freq) = num_freq.get_mut(&nums[i]) {
                *freq += 1;
                continue;
            }
            num_freq.insert(nums[i], 1);
        }
        for (num, freq) in num_freq.iter() {
            if *freq == 1 {
                return *num;
            }
        }
        -1

        // constant space but not linear runtime complexity (o(n^2))
        // let mut the_num: i32 = 0;
        // for i in 0..nums.len() {
        //     let mut idx: usize = 0;
        //     the_num = nums[i];
        //     if i == (nums.len()-1) {return the_num}
        //     while idx < nums.len() {
        //         if i == idx {
        //             idx += 1;
        //             continue
        //         }
        //         if the_num == nums[idx] {break}
        //         idx += 1;
        //     }
        //     if idx == nums.len() {
        //         break;
        //     }
        // }
        // the_num
    }
}
