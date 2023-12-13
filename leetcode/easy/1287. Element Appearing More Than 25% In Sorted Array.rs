use std::collections::HashMap;
impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut num_freq: HashMap<i32, i32> = HashMap::new();
        let mut ans: i32 = -1;
        for num in &arr {
            if let Some(freq) = num_freq.get_mut(&num) {
                *freq += 1;
                if *freq > (arr.len() / 4) as i32 {
                    ans = *num;
                    break;
                }
            } else {
                num_freq.insert(*num, 1);
                if 1 > (arr.len() / 4) {
                    ans = *num;
                    break;
                }
            }
        }
        ans
    }
}
