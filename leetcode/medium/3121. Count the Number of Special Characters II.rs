use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut count: i32 = 0;
        let mut uppercase: HashMap<char, Vec<usize>> = HashMap::new();
        let mut lowercase: HashMap<char, Vec<usize>> = HashMap::new();
        let mut temp: Vec<char> = word.chars().collect();
        for i in 0..temp.len() {
            if Self::is_uppercase_u32(temp[i]) {
                if let Some(idxs) = uppercase.get_mut(&temp[i]) {
                    idxs.push(i);
                } else {
                    uppercase.insert(temp[i], vec![i]);
                }
            } else {
                if let Some(idxs) = lowercase.get_mut(&temp[i]) {
                    idxs.push(i);
                } else {
                    lowercase.insert(temp[i], vec![i]);
                }
            }
        }
        for (low_let, low_idxs) in lowercase.iter() {
            let upp_let = low_let.to_ascii_uppercase();
            let opt_upp_idxs = uppercase.get(&upp_let);
            let mut l_i: usize = 0;
            let mut u_i: usize = 0;
            match opt_upp_idxs {
                Some(upp_idxs) => {
                    if low_idxs[low_idxs.len() - 1] < upp_idxs[0] {
                        count += 1;
                    }
                }
                None => {}
            }
            // println!("{}-{}", low_let, upp_let);
            // println!("{}-{}-{:?}", low_let, upp_let, upp_idxs);
        }

        // println!("{:?}", uppercase);
        // println!("{:?}", lowercase);
        count
    }
    pub fn is_uppercase_u32(letter: char) -> bool {
        let ascii: u32 = u32::from(letter);
        ascii < 97 && ascii >= 65
    }
}
