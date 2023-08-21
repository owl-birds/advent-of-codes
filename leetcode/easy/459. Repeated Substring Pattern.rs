impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s_vec: Vec<char> = s.chars().collect();

        for length in 1..(s_vec.len() / 2) + 1 {
            if s_vec.len() % length != 0 {
                continue;
            }
            let mut subs: Vec<char> = vec![];
            for i in 0..length {
                subs.push(s_vec[i]);
            }
            // println!("length: {}-->{:?}", length, subs);
            let mut idx: usize = 0;
            // let mut count: usize = 0;
            while idx < s_vec.len() {
                if !Self::is_the_same(&s_vec, &subs, idx) {
                    break;
                }
                // count += 1;
                idx += length;
            }
            // if count == (s_vec.len()/length) {return true}
            if idx == s_vec.len() {
                return true;
            }
        }

        false
    }
    pub fn is_the_same(s_vec: &Vec<char>, subs: &Vec<char>, start_idx: usize) -> bool {
        let mut s_v_idx: usize = start_idx;
        for i in 0..subs.len() {
            if s_vec[s_v_idx] != subs[i] {
                return false;
            }
            s_v_idx += 1;
        }
        true
    }
}
