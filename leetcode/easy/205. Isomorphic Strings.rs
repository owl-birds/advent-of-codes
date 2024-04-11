use std::collections::HashMap;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut temp_s: Vec<char> = s.chars().collect();
        let mut temp_map: HashMap<char, i32> = HashMap::new();
        let mut new_s: Vec<i32> = vec![];
        let mut count: i32 = 0;
        for i in 0..temp_s.len() {
            if let Some(change) = temp_map.get(&temp_s[i]) {
                new_s.push(*change);
                continue;
            }
            count += 1;
            temp_map.insert(temp_s[i], count);
            new_s.push(count);
        }

        let mut temp_t: Vec<char> = t.chars().collect();
        let mut temp_map: HashMap<char, i32> = HashMap::new();
        let mut new_t: Vec<i32> = vec![];
        let mut count: i32 = 0;
        for i in 0..temp_t.len() {
            if let Some(change) = temp_map.get(&temp_t[i]) {
                new_t.push(*change);
                continue;
            }
            count += 1;
            temp_map.insert(temp_t[i], count);
            new_t.push(count);
        }

        for i in 0..new_s.len() {
            if new_s[i] != new_t[i] {
                return false;
            }
        }

        true
    }
}
