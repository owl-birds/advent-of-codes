use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() || s.len() == 1 {
            return false;
        }
        let s_chars: Vec<char> = s.chars().collect();
        let goal_chars: Vec<char> = goal.chars().collect();
        let mut s_set: HashSet<char> = HashSet::new();
        let mut goal_set: HashSet<char> = HashSet::new();
        let mut count: i32 = 0;
        for i in 0..s_chars.len() {
            s_set.insert(s_chars[i]);
            goal_set.insert(goal_chars[i]);
            if s_chars[i] == goal_chars[i] {
                count += 1
            }
        }
        if count as usize == s.len()
            && s.len() > 2
            && (s_set.len() > (s.len() - 2) || goal_set.len() > (goal.len() - 2))
        {
            return false;
        }
        if count as usize == s_chars.len() && count > 2 {
            return true;
        }
        // println!("{:?}", s_chars);
        // println!("{:?}", goal_chars);
        let mut s_map: HashMap<char, i32> = HashMap::new();
        for i in 0..s_chars.len() {
            if let Some(count) = s_map.get_mut(&s_chars[i]) {
                *count += 1;
                continue;
            }
            s_map.insert(s_chars[i], 1);
        }
        let mut goal_map: HashMap<char, i32> = HashMap::new();
        for i in 0..goal_chars.len() {
            if let Some(count) = goal_map.get_mut(&goal_chars[i]) {
                *count += 1;
                continue;
            }
            goal_map.insert(goal_chars[i], 1);
        }
        // println!("{:?}", s_map);
        // println!("{:?}", goal_map);

        if s_map.len() == 1 && goal_map.len() == 1 {
            for (c, _) in s_map.iter() {
                if goal_map.contains_key(c) {
                    return true;
                } else {
                    return false;
                }
            }
        }
        for (c, v) in s_map.iter() {
            // if !goal_map.contains_key(c) {return false}
            if let Some(count) = goal_map.get(c) {
                if count != v {
                    return false;
                }
                continue;
            }
            return false;
        }

        let mut count: i32 = 0;
        for i in 0..s_chars.len() {
            if s_chars[i] != goal_chars[i] {
                count += 1
            }
        }

        if count == 2 {
            return true;
        }
        false
    }
}
