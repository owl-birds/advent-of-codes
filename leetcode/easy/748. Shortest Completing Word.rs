use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let lp_chars: Vec<char> = license_plate.to_lowercase().chars().collect();
        let letters_set: HashSet<char> = HashSet::from([
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ]);

        let lp_map: HashMap<char, i32> = Self::gen_freq_map_consist(lp_chars, &letters_set);
        // println!("{:?}", lp_map);

        let mut completing_word: String = String::from("");
        // let mut completing_word_score: i32 = i32::MAX;

        for word in words {
            let w_map: HashMap<char, i32> =
                Self::gen_freq_map_consist(word.chars().collect(), &letters_set);
            // let mut score: i32 = 0;
            let mut count: i32 = 0;
            for (num, freq) in lp_map.iter() {
                let mut w_freq: i32 = *freq;
                if let Some(w_f) = w_map.get(&num) {
                    // w_freq = (freq-*w_f);
                    // w_freq = w_freq.abs();
                    if *w_f >= *freq {
                        count += 1;
                    }
                }
                // score += w_freq;
            }

            // println!("word:{:?},map:{:?}", word, w_map);
            // println!("word:{:?},score:{}", word, score);
            // println!("word:{:?},count:{}", word, count);
            if (count == lp_map.len() as i32 && completing_word == "")
                || (count == lp_map.len() as i32 && word.len() < completing_word.len())
            {
                // completing_word_score = score;
                completing_word = word.clone();
            }
        }

        completing_word
    }
    pub fn gen_freq_map_consist(s_chars: Vec<char>, chars: &HashSet<char>) -> HashMap<char, i32> {
        let mut result: HashMap<char, i32> = HashMap::new();

        for c in s_chars {
            if let Some(freq) = result.get_mut(&c) {
                *freq += 1;
                continue;
            }
            if chars.contains(&c) {
                result.insert(c, 1);
            }
        }

        result
    }
}
