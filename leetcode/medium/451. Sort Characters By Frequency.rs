use std::collections::HashMap;
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        #[derive(Debug)]
        struct CharFreq {
            c: char,
            freq: i32
        }
        impl CharFreq {
            pub fn new(c: char, freq: i32) -> Self {
                Self {
                    c,
                    freq
                }
            }
        }
        let mut char_freq: HashMap<char, i32> = HashMap::new();
        let mut result: String = String::new();
        for c in s.chars() {
            if let Some(freq) = char_freq.get_mut(&c) {
                *freq += 1;        
                continue;
            }
            char_freq.insert(c, 1);
        }
        // println!("{:?}", char_freq);
        let mut temp: Vec<CharFreq> = vec![];
        for (c, f) in char_freq.iter() {
            temp.push(CharFreq::new(*c, *f));
        }
        temp.sort_by(|a,b| b.freq.cmp(&a.freq));
        // println!("{:?}", temp);
        for i in 0..temp.len() {
            for _ in 0..temp[i].freq {
                result.push(temp[i].c);
            }
        }

        result
    }
}