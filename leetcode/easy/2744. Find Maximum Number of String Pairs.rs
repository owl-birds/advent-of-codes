use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        // words have distinct strings, so each of the string has unique combination of chars
        // let mut reversed_string: HashSet<String> = HashSet::new();
        let mut reversed_string: HashMap<String, Vec<usize>> = HashMap::new();
        let mut count: i32 = 0;

        for i in 0..words.len() {
            // reversed_string.insert(Self::reverse_string(&words[i][0..]));
            let rev_word = Self::reverse_string(&words[i][0..]);
            if let Some(idxs) = reversed_string.get_mut(&rev_word) {
                idxs.push(i)
            } else {
                reversed_string.insert(rev_word, vec![i]);
            }
        }
        for i in 0..words.len() {
            if reversed_string.contains_key(&words[i]) {
                let idxs_match = reversed_string.get(&words[i]);
                match idxs_match {
                    Some(idxs) => {
                        // println!("{:?}", idxs);
                        if idxs[0] != i {
                            count += 1;
                        }
                    }
                    None => {}
                }
            }
        }

        // TEST
        // println!("{:?}", reversed_string);
        // let test: String = "testing a reverse function for a String type in rust".to_string();
        // println!("{:?}", test);
        // let test: String = Self::reverse_string(&test[0..test.len()]);
        // println!("{:?}", test);
        // TEST
        count / 2 // what a redundant things to do
    }
    pub fn reverse_string(word: &str) -> String {
        word.chars().rev().collect()
    }
}
