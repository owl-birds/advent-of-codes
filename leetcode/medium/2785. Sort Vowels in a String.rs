use std::collections::HashSet;
impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut vowels: Vec<char> = vec!['a', 'i', 'u', 'e', 'o', 'A', 'I', 'U', 'E', 'O'];
        let mut vowel_set: HashSet<char> = HashSet::new();
        for v in &vowels {
            vowel_set.insert(*v);
        }

        //
        let mut temp_s: Vec<char> = s.chars().collect();
        let mut vowels_s: Vec<char> = vec![];
        for c in &temp_s {
            if vowel_set.contains(&c) {
                vowels_s.push(*c);
            }
        }
        if vowels_s.len() == 0 {
            return s.clone();
        }
        // println!("{:?}", vowels_s);
        vowels_s.sort_by(|a, b| {
            let a_ascii = (*a as usize) as i32;
            let b_ascii = (*b as usize) as i32;
            a_ascii.cmp(&b_ascii)
        });
        // println!("{:?}", vowels_s);
        let mut result: String = String::new();
        let mut idx: usize = 0;
        for c in &temp_s {
            if vowel_set.contains(&c) {
                result.push(vowels_s[idx]);
                idx += 1;
                continue;
            }
            result.push(*c);
        }

        result
    }
}
