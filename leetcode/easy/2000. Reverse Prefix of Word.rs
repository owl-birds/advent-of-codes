impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let ch_idx = word.find(ch).unwrap_or(word.len());
        // println!("{}", ch_idx);
        let mut word_temp: String = word.clone();
        if ch_idx == word.len() {
            return word_temp;
        }
        let before_char: Vec<char> = word_temp[..ch_idx + 1].to_string().chars().collect();
        let mut before_ch: String = String::new();
        let mut idx: usize = before_char.len();
        while idx >= 1 {
            idx -= 1;
            before_ch.push(before_char[idx]);
        }
        // println!("{}:{}:{:?}",ch, ch_idx, before_ch);
        word_temp.replace_range(..ch_idx + 1, &before_ch[..]);
        word_temp
    }
}
