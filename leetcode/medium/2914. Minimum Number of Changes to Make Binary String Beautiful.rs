impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut str_chars: Vec<char> = s.chars().collect();
        let mut len: usize = s.len();
        let mut count: i32 = 0;
        let mut idx: usize = 1;

        while idx < len {
            if str_chars[idx] != str_chars[idx - 1] {
                count += 1;
            }

            idx += 2;
        }

        count
    }
}
