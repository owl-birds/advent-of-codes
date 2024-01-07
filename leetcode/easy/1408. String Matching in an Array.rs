use std::collections::HashSet;
impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut temp: Vec<Vec<char>> = vec![];
        // let mut temp_result: Vec<usize> = vec![];
        for i in 0..words.len() {
            temp.push(words[i].chars().collect());
        }
        // println!("{:?}", temp);
        let mut result: Vec<String> = vec![];
        let mut result_set: HashSet<String> = HashSet::new();
        for i in 0..temp.len() {
            if result_set.contains(&words[i]) {
                continue;
            }
            let mut is_sub: bool = false;
            for j in 0..temp.len() {
                if i == j {
                    continue;
                }
                if is_sub {
                    break;
                }
                for idx in 0..temp[j].len() {
                    if Self::is_substring(&temp[j], &temp[i], idx) {
                        // println!("{:?} : {:?}", temp[i], temp[j]);
                        is_sub = true;
                        break;
                    }
                }
            }
            if is_sub {
                result.push(words[i].to_string());
                result_set.insert(words[i].to_string());
            }
        }

        result
    }
    pub fn is_substring(word: &Vec<char>, substr: &Vec<char>, word_idx: usize) -> bool {
        if substr.len() > word.len() {
            return false;
        }
        let mut w_idx: usize = word_idx;
        let mut sub_idx: usize = 0;
        while w_idx < word.len() && sub_idx < substr.len() {
            if word[w_idx] != substr[sub_idx] {
                break;
            }
            sub_idx += 1;
            w_idx += 1;
        }

        sub_idx == substr.len()
    }
}
