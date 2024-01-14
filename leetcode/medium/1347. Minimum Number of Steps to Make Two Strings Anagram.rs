use std::collections::HashMap;
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        // same length char
        if s.len() != t.len() {
            return -1;
        }
        let t_temp: Vec<char> = t.chars().collect();
        let s_temp: Vec<char> = s.chars().collect();
        let mut t_count: HashMap<char, i32> = HashMap::new();
        let mut s_count: HashMap<char, i32> = HashMap::new();
        for i in 0..s.len() {
            if let Some(freq) = t_count.get_mut(&t_temp[i]) {
                *freq += 1;
            } else {
                t_count.insert(t_temp[i], 1);
            }
            if let Some(freq) = s_count.get_mut(&s_temp[i]) {
                *freq += 1;
            } else {
                s_count.insert(s_temp[i], 1);
            }
        }
        // println!("{:?}\n{:?}", t_count, s_count);
        let mut count: i32 = 0;

        for (c, f) in s_count.iter() {
            if let Some(freq) = t_count.get(&c) {
                if f <= freq {
                    count += *f;
                } else {
                    count += *freq;
                }
            }
        }

        (s.len() as i32) - count
    }
}
