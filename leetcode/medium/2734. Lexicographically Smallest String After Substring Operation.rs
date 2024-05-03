use std::collections::HashMap;
impl Solution {
    pub fn smallest_string(s: String) -> String {
        let mut temp: Vec<char> = s.chars().collect();
        let mut ord_letter: HashMap<i32, char> = HashMap::new();
        let mut start: usize = 0;
        let mut end: usize = 0;
        let mut s_len: usize = s.len();

        for c in "abcdefghijklmnopqrstuvwxyz".chars() {
            ord_letter.insert(c as i32, c);
        }
        // println!("{:?}", ord_letter);

        while start < s_len {
            if temp[start] != 'a' {
                break;
            }
            start += 1;
        }
        if start == s_len {
            temp[s_len - 1] = 'z';
        } else {
            // println!("substring start at : {}", start);
            end = start;
            while end < s_len {
                if temp[end] == 'a' {
                    break;
                }
                end += 1;
            }
            // println!("substring end at : {}", end);
            while start < end {
                temp[start] = *ord_letter.get(&(temp[start] as i32 - 1)).unwrap();
                start += 1;
            }
        }

        let mut ans: String = "".to_string();
        for i in 0..temp.len() {
            ans.push(temp[i]);
        }
        ans
    }
}
