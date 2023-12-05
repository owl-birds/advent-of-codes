impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        let mut ans: Vec<String> = vec![];

        for word in &words {
            let temp_vec: Vec<String> = Self::str_split_by(&word, &separator);
            for w in temp_vec {
                ans.push(w);
            }
        }

        // TEST
        // println!("{:?}", Self::str_split_by(&String::from("one.two.three"), &separator));
        // println!("{:?}", Self::str_split_by(&words[0], &separator));
        // println!("{:?}", Self::str_split_by(&String::from("two"), &separator));
        // TEST

        ans
    }
    pub fn str_split_by(word: &String, separator: &char) -> Vec<String> {
        let mut str_vec: Vec<String> = vec![];
        let mut sep_idxs: Vec<usize> = vec![];

        let mut idx: usize = 0;
        for c in word.chars() {
            if c == *separator {
                sep_idxs.push(idx);
            }
            idx += 1;
        }
        sep_idxs.push(idx);
        // println!("{:?}", sep_idxs);
        let mut idx: usize = 0;
        let mut word_temp: Vec<char> = word.chars().collect();
        for i in 0..sep_idxs.len() {
            let mut temp_str: String = String::new();
            for j in idx..sep_idxs[i] {
                if word_temp[j] == *separator {
                    continue;
                }
                temp_str.push(word_temp[j]);
            }
            if temp_str.len() > 0 {
                str_vec.push(temp_str);
            }
            idx = sep_idxs[i];
        }

        str_vec
    }
}
