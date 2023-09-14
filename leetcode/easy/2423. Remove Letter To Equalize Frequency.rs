use std::collections::HashMap;
use std::collections::HashSet;
// TRASH SOLUTION IM LAZY
impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut freq_map: HashMap<char, i32> = HashMap::new();

        let word_c: Vec<char> = word.chars().collect();
        for c in word_c {
            if let Some(freq) = freq_map.get_mut(&c) {
                *freq += 1;
            } else {
                freq_map.insert(c, 1);
            }
        }
        // println!("{:?}", freq_map);
        // let mut freq_set: HashSet<i32> = HashSet::new();
        let mut freq_map2: HashMap<i32, i32> = HashMap::new();
        let mut freqs: Vec<i32> = vec![];
        for (letter, freq) in freq_map.iter() {
            if let Some(f) = freq_map2.get_mut(&freq) {
                *f += 1;
            } else {
                freq_map2.insert(*freq, 1);
                if freq_map2.len() > 2 {
                    return false;
                }
                freqs.push(*freq);
            }
        }
        println!("{:?}", freq_map2);
        println!("{:?}", freqs);
        if freqs.len() == 1 {
            if freqs[0] == 1 || freq_map.len() == 1 {
                return true;
            }
            return false;
        }
        let freq1: i32 = *freq_map2.get(&freqs[0]).unwrap();
        let freq2: i32 = *freq_map2.get(&freqs[1]).unwrap();
        if i32::abs(freqs[0] - freqs[1]) == 1 {
            if freqs[0] > freqs[1] && freq1 == 1 {
                return true;
            } else if freqs[0] < freqs[1] && freq2 == 1 {
                return true;
            }
        }
        if (freqs[0] == 1 && freq1 == 1) || (freqs[1] == 1 && freq2 == 1) {
            return true;
        }
        false
    }
}
