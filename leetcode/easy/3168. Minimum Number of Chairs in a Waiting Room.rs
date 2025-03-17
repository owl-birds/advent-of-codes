impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let mut ans: i32 = 0;
        let mut temp: i32 = 0;
        for i in 0..s_chars.len() {
            if s_chars[i] == 'E' {
                temp += 1;
                if temp > ans {
                    ans = temp
                }
            } else {
                temp -= 1;
            }
        }
        ans
    }
}
