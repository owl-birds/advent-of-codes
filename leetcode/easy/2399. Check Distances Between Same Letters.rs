use std::collections::HashMap;

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut char_idxs: HashMap<char, usize> = HashMap::new();
        let mut idx2: usize = 0;
        for c in s.chars() {
            if let Some(idx1) = char_idxs.get(&c) {
                if (idx2 - idx1 - 1) != distance[c as usize - 97] as usize {
                    return false;
                }
            } else {
                char_idxs.insert(c, idx2);
            }
            idx2 += 1;
        }

        // 'a' 97
        // println!("{}", 'a' as usize);
        // println!("{}", 'z' as usize);
        // 'z' 122

        true
    }
}
