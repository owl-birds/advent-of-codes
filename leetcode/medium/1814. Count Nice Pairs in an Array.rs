use std::collections::HashMap;

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut count: i32 = 0;
        let mut diff_reverse: Vec<i32> = vec![];
        let mut diff_freq: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            let diff: i32 = nums[i] - Self::rev_num(nums[i]);
            if let Some(freq) = diff_freq.get_mut(&diff) {
                count = (count + *freq) % (i32::pow(10, 9) + 7);
                *freq += 1;
            } else {
                diff_freq.insert(diff, 1);
            }
        }

        // // TEST
        // println!("{:?}", diff_freq);
        // println!("90--{}", Self::rev_num(90));
        // // TEST

        count
    }
    pub fn rev_num(num: i32) -> i32 {
        let mut temp: i32 = num;
        let mut len: i32 = 0;
        while temp > 0 {
            len += 1;
            temp /= 10;
        }
        let mut result: i32 = 0;
        let mut temp: i32 = num;
        let mut degree: i32 = i32::pow(10, (len - 1) as u32);
        while temp > 0 {
            let digit: i32 = (temp % 10) * degree;

            result += digit;

            degree /= 10;
            temp /= 10;
        }

        result
    }
}
