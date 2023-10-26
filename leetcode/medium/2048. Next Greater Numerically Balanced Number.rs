use std::collections::HashMap;
impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        let mut result: i32 = n + 1;
        while !Self::is_balanced(result) {
            result += 1;
        }
        result        
    }
    pub fn is_balanced(num: i32) -> bool {
        let mut digit_freq: HashMap<i32, i32> = HashMap::new();
        let mut temp: i32 = num;
        while temp > 0 {
            let digit: i32 = temp % 10;
            if let Some(freq) = digit_freq.get_mut(&digit) {
                *freq += 1;
            } else {
                digit_freq.insert(digit, 1);
            }
            temp /= 10;
        }
        for (digit, freq) in digit_freq.iter() {
            if digit != freq {return false}
        }
        true
    }
}