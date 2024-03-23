impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let s_chars: Vec<char> = s.chars().collect();
        let mut max_oddnum: String = String::new();
        let mut count_one: i32 = 0;
        for i in 0..s_chars.len() {
            if s_chars[i] == '1' {
                count_one += 1
            }
        }
        let mut zero_count: i32 = s.len() as i32 - count_one;
        // println!("{}", count_one);
        while count_one > 1 {
            max_oddnum.push('1');
            count_one -= 1;
        }
        // println!("{}", count_one);
        while zero_count > 0 {
            zero_count -= 1;
            max_oddnum.push('0');
        }
        max_oddnum.push('1');
        max_oddnum
    }
}
