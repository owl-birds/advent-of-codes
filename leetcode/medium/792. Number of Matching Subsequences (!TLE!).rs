use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut count: i32 = 0;
        let mut char_idxs: HashMap<char, Vec<usize>> = HashMap::new();
        let mut memo: HashSet<String> = HashSet::new();
        // [
        // {char: [pointer_idx, num_idx, num_idx ...]}
        // ]
        let mut i: usize = 0;
        for c in s.chars() {
            if let Some(idxs) = char_idxs.get_mut(&c) {
                idxs.push(i);
            } else {
                char_idxs.insert(c, vec![1, i]);
            }
            i += 1;
        }

        for word in words {
            if word.len() > s.len() {
                continue;
            }
            if memo.contains(&word) {
                count += 1;
                continue;
            }
            if Self::is_subseq(&char_idxs, &word) {
                // if Self::is_subseq(&s, &word) {
                memo.insert(word);
                count += 1;
            }
        }

        count
    }
    // pub fn is_subseq(the_str: &String, the_subseq: &String) -> bool {
    pub fn is_subseq(char_idxs: &HashMap<char, Vec<usize>>, s: &String) -> bool {
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
        // println!("->{:?}", char_idxs);
        // let mut s_idx_in_t: Vec<usize> = vec![];
        let mut s_idx_in_t: i32 = -1;
        let mut char_idxs = char_idxs.clone();
        for c in s.chars() {
            if let Some(point_idxs) = char_idxs.get_mut(&c) {
                if point_idxs[0] == point_idxs.len() {
                    return false;
                }
                let mut c_idx: i32 = -1;
                if s_idx_in_t == -1 {
                    // the first char
                    s_idx_in_t = point_idxs[point_idxs[0]] as i32;
                    point_idxs[0] += 1;
                    continue;
                }
                for i in point_idxs[0]..point_idxs.len() {
                    // we need to find the idx in t that have the same char as in t, but the idx need to be bigger then the prev char idx in s
                    if point_idxs[i] > s_idx_in_t as usize {
                        c_idx = point_idxs[i] as i32;
                        point_idxs[0] = i + 1;
                        break;
                    }
                }
                if c_idx != -1 {
                    s_idx_in_t = c_idx;
                    continue;
                }
            }
            return false;
        }

        true

        // //
        // let mut i_main: usize = 0;
        // let mut i_sub: usize = 0;

        // while i_main < the_str.len() && i_sub < the_subseq.len() {
        //     if the_str.as_bytes()[i_main] == the_subseq.as_bytes()[i_sub] {
        //         i_sub += 1;
        //     }
        //     i_main += 1;
        // }

        // i_sub == the_subseq.len()
    }
}
