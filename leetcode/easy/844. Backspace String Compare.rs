impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s_vec: Vec<char> = vec![];
        let mut t_vec: Vec<char> = vec![];
        for c in s.chars() {
            if c == '#' {
                s_vec.pop();
                continue;
            }
            s_vec.push(c);
        }
        for c in t.chars() {
            if c == '#' {
                t_vec.pop();
                continue;
            }
            t_vec.push(c);
        }
        if t_vec.len() != s_vec.len() {
            return false;
        }
        for i in 0..t_vec.len() {
            if t_vec[i] != s_vec[i] {
                return false;
            }
        }
        true
    }
}
