use std::collections::HashMap;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut num_freq: HashMap<i32, i32> = HashMap::new();
        let mut min_oper: i32 = 0;
        for i in 0..nums.len() {
            if let Some(freq) = num_freq.get_mut(&nums[i]) {
                *freq += 1;
                continue;
            }
            num_freq.insert(nums[i], 1);
        }
        // println!("{:?}", num_freq);

        for (_, freq) in num_freq.iter() {
            if *freq <= 1 {
                return -1;
            }
            // println!("freq: {}", freq);
            let mut temp_freq: i32 = *freq;
            // while temp_freq >= 5 {
            //     min_oper += 1;
            //     temp_freq -= 3;
            // }
            if temp_freq >= 5 {
                let operations: i32 = temp_freq / 3;
                if temp_freq - (operations * 3) == 1 {
                    min_oper += (operations - 1);
                    temp_freq -= (operations - 1) * 3;
                } else {
                    min_oper += operations;
                    temp_freq -= operations * 3;
                }
            }
            // println!("{}", temp_freq);
            if temp_freq == 0 {
                continue;
            }
            if temp_freq < 2 {
                return -1;
            }
            if temp_freq % 2 == 0 {
                min_oper += (temp_freq / 2);
            }
            if temp_freq == 3 {
                min_oper += 1;
            }
        }

        min_oper
    }
}
