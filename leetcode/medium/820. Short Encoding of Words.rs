use std::collections::HashSet;

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut temp: Vec<Vec<char>> = vec![];
        for word in &words {
            temp.push(word.chars().collect());
        }
        temp.sort_by(|a, b| {
            let a_len = a.len();
            let b_len = b.len();

            b_len.cmp(&a_len)
        });
        // println!("{:?}", temp);
        let mut decoded: Vec<char> = vec![];
        let mut word_idxs: Vec<usize> = vec![];

        let mut idx: usize = 0;
        for word in &temp {
            let mut is_exist: bool = false;
            for i in 0..word_idxs.len() {
                if word.len() > temp[word_idxs[i]].len() {
                    continue;
                }
                if Self::is_word1_in(&word, &temp[word_idxs[i]]) {
                    is_exist = true;
                    idx += 1;
                    break;
                }
            }
            if is_exist {
                continue;
            }
            word_idxs.push(idx);
            idx += 1;
        }
        // println!("{:?}", word_idxs);
        let mut result: i32 = 0;
        for i in 0..word_idxs.len() {
            result += (temp[word_idxs[i]].len()) as i32 + 1;
        }

        result
    }
    pub fn is_word1_in(word1: &Vec<char>, word2: &Vec<char>) -> bool {
        let mut idx1: usize = word1.len();
        let mut idx2: usize = word2.len();

        while idx1 >= 1 {
            idx1 -= 1;
            idx2 -= 1;
            if word1[idx1] != word2[idx2] {
                return false;
            }
        }

        true
    }
}
