use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut temp: Vec<char> = s.chars().collect();
        let mut subs_remove: HashSet<String> =
            HashSet::from([String::from("AB"), String::from("CD")]);
        let mut s_stack: VecDeque<char> = VecDeque::from([temp[0]]);
        let mut idx: usize = 1;
        while idx < temp.len() {
            if s_stack.len() == 0 {
                s_stack.push_back(temp[idx]);
                idx += 1;
                continue;
            }
            let mut temp_string: String = String::new();
            let mut back: char = *s_stack.back().unwrap();
            temp_string.push(back);
            temp_string.push(temp[idx]);
            // println!("{:?}", temp_string);
            if subs_remove.contains(&temp_string) {
                s_stack.pop_back();
            } else {
                s_stack.push_back(temp[idx]);
            }
            idx += 1;
        }
        // println!("{:?}", s_stack);

        s_stack.len() as i32
    }
}
