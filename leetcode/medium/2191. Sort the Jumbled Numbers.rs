use std::cmp::Ordering;
impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let mut temp_nums: Vec<Vec<i32>> = vec![];
        let mut result: Vec<i32> = vec![];
        for i in 0..nums.len() {
            let mut temp: i32 = nums[i];
            let mut multiply: i32 = 1;
            let mut ord_value: i32 = 0;
            while temp > 0 {
                let digit_i: usize = (temp % 10) as usize;
                ord_value += mapping[digit_i] * multiply;
                temp /= 10;
                multiply *= 10;
            }
            if nums[i] == 0 {
                temp_nums.push(vec![nums[i], mapping[nums[i] as usize]]);
                continue;
            }
            temp_nums.push(vec![nums[i], ord_value]);
        }
        temp_nums.sort_by(|a, b| {
            if a[1] != b[1] {
                return a[1].cmp(&b[1]);
            }
            return Ordering::Equal;
        });
        for i in 0..temp_nums.len() {
            result.push(temp_nums[i][0]);
        }
        // println!("{:?}", temp_nums);
        result
    }
}
