impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let num_chars: Vec<char> = num.chars().collect();
        let mut ans: String = String::new();
        let mut idx: usize = num_chars.len();
        let mut is_exist: bool = false;

        while idx > 0 {
            idx -= 1;
            if num_chars[idx].to_digit(10).unwrap() % 2 != 0 {
                is_exist = true;
                break;
            }
        }
        if is_exist {
            // println!("idx:{}---{:?}", idx, num[..idx+1].to_string());
            ans = num[..idx + 1].to_string();
        }

        ans
    }
}
