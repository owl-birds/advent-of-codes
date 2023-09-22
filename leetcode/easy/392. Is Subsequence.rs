use std::collections::HashMap;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        // more simple solution
        let mut idx_s: usize = 0;
        let mut idx_t: usize = 0;

        while idx_s < s.len() && idx_t < t.len() {
            if s.as_bytes()[idx_s] == t.as_bytes()[idx_t] {
                idx_s += 1;
            }
            idx_t += 1;
        }

        idx_s == s.len()
        // more simple solution

        // // OVERCOMPLICATED SOLUTION
        // let mut char_idxs: HashMap<char, Vec<usize>> = HashMap::new();
        // // [
        //     // {char: [pointer_idx, num_idx, num_idx ...]}
        // // ]
        // let mut i: usize = 0;
        // for c in t.chars() {
        //     if let Some(idxs) = char_idxs.get_mut(&c) {
        //         idxs.push(i);
        //     }else {
        //         char_idxs.insert(c, vec![1, i]);
        //     }
        //     i += 1;
        // }
        // // println!("->{:?}", char_idxs);
        // // let mut s_idx_in_t: Vec<usize> = vec![];
        // let mut s_idx_in_t: i32 = -1;
        // for c in s.chars() {
        //     if let Some(point_idxs) = char_idxs.get_mut(&c) {
        //         if point_idxs[0] == point_idxs.len() {return false}
        //         let mut c_idx: i32 = -1;
        //         if s_idx_in_t == -1 { // the first char
        //             s_idx_in_t = point_idxs[point_idxs[0]] as i32;
        //             point_idxs[0] += 1;
        //             continue;
        //         }
        //         for i in point_idxs[0]..point_idxs.len() { // we need to find the idx in t that have the same char as in t, but the idx need to be bigger then the prev char idx in s
        //             if point_idxs[i] > s_idx_in_t as usize {
        //                 c_idx = point_idxs[i] as i32;
        //                 point_idxs[0] = i + 1;
        //                 break;
        //             }
        //         }
        //         if c_idx != -1 {
        //             s_idx_in_t = c_idx;
        //             continue;
        //         }
        //     }
        //     return false
        // }

        // true
        // // OVERCOMPLICATED SOLUTION
    }
}
