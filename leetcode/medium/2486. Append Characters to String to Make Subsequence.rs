impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let mut s_idx: usize = 0;
        let mut t_idx: usize = 0;

        while s_idx < s.len() && t_idx < t.len() {
            if s.as_bytes()[s_idx] == t.as_bytes()[t_idx] {
                t_idx += 1;
            }

            s_idx += 1;
        }
        // println!("s_idx:{}\nt_idx:{}", s_idx, t_idx);

        (t.len() - t_idx) as i32
    }
}
