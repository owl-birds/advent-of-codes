use std::collections::HashMap;

#[derive(Debug)]
struct CharFreq {
    c: char,
    freq: i32,
}
impl CharFreq {
    pub fn new(c: char, freq: i32) -> Self {
        Self { c, freq }
    }
}

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut result: String = String::new();
        let mut chars_freq: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            if let Some(freq) = chars_freq.get_mut(&c) {
                *freq += 1;
                continue;
            }
            chars_freq.insert(c, 1);
        }
        // println!("{:?}", chars_freq);
        let mut cf_vec: Vec<CharFreq> = vec![];
        for (c, freq) in chars_freq.iter() {
            cf_vec.push(CharFreq::new(*c, *freq))
        }
        cf_vec.sort_by(|a, b| a.freq.cmp(&b.freq));
        // println!("{:?}", cf_vec);
        let mut curr_len: usize = 0;
        while curr_len < s.len() {
            let mut c_1: Option<CharFreq> = cf_vec.pop();
            let mut c_2: Option<CharFreq> = cf_vec.pop();
            let mut temp: Vec<char> = vec![];
            // println!("{:?}", cf_vec);
            match c_1 {
                Some(mut the_char) => {
                    if the_char.freq > 0 {
                        // result.push(the_char.c);
                        temp.push(the_char.c);
                        the_char.freq -= 1;
                        cf_vec.push(the_char);
                    }
                }
                None => {}
            }
            match c_2 {
                Some(mut the_char) => {
                    if the_char.freq > 0 {
                        // result.push(the_char.c);
                        temp.push(the_char.c);
                        the_char.freq -= 1;
                        cf_vec.push(the_char);
                    }
                }
                None => {}
            }

            if temp.len() == 0 {
                break;
            }
            cf_vec.sort_by(|a, b| a.freq.cmp(&b.freq));
            if result.len() == 0 {
                for i in 0..temp.len() {
                    result.push(temp[i]);
                }
            } else if temp.len() > 1 {
                let mut temp_last_char: char = result.pop().unwrap();
                if temp_last_char != temp[0] {
                    result.push(temp_last_char);
                    result.push(temp[0]);
                    result.push(temp[1]);
                } else if temp_last_char != temp[1] {
                    result.push(temp_last_char);
                    result.push(temp[1]);
                    result.push(temp[0]);
                }
            } else {
                let mut temp_last_char: char = result.pop().unwrap();
                if temp_last_char == temp[0] {
                    break;
                };
                result.push(temp_last_char);
                result.push(temp[0]);
            }
            // println!("{:?}", cf_vec);
            // println!("----");
            curr_len += 2;
        }
        if result.len() != s.len() {
            return "".to_string();
        }
        result
        // sort every time u take two chars (based on the freq)
        // String::from("zazazbzbabazabz")
    }
}
