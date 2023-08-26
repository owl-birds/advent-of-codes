impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s_vec: Vec<char> = s.chars().collect();
        let mut count: i32 = 0;
        let mut length: usize = 1;

        while length <= s_vec.len() {
            let mut start: usize = 0;
            while start <= (s_vec.len() - length) {
                // println!("{}--{}", start, (start-1)+length);
                if Self::is_palindrom(&s_vec, start, (start - 1) + length) {
                    count += 1;
                }
                start += 1;
            }
            length += 1;
        }

        count
    }
    pub fn is_palindrom(s: &Vec<char>, start: usize, end: usize) -> bool {
        let mut l: i32 = start as i32;
        let mut r: i32 = end as i32;

        while r >= 0 && (l as usize) < s.len() && l <= r {
            if s[l as usize] != s[r as usize] {
                return false;
            }

            l += 1;
            r -= 1;
        }

        true
    }
}
