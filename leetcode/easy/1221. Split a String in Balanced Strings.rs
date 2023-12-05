use std::collections::HashMap;

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let temp: Vec<char> = s.chars().collect();
        let mut count: i32 = 0;
        let mut char_freq: HashMap<char, i32> = HashMap::new();
        let mut all_chars: Vec<char> = vec!['R', 'L'];
        for c in &all_chars {
            char_freq.insert(*c, 0);
        }
        // println!("{:?}", char_freq);

        for i in 0..temp.len() {
            let mut curr_freq: i32 = 0;
            if let Some(freq) = char_freq.get_mut(&temp[i]) {
                *freq += 1;
                curr_freq = *freq;
            }
            let mut count_same_freq: u32 = 0;
            for c in &all_chars {
                if *char_freq.get(&c).unwrap() == curr_freq {
                    count_same_freq += 1;
                }
            }
            if count_same_freq as usize == char_freq.len() {
                count += 1;
            }
        }

        count
    }
}
