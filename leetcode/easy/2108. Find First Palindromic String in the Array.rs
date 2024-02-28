impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for i in 0..words.len() {
            let temp: Vec<char> = words[i].chars().collect();
            if Self::is_pal(&temp) {
                return words[i].to_string();
            }
        }

        String::from("")
    }
    pub fn is_pal(word_chars: &Vec<char>) -> bool {
        let mut first: usize = 0;
        let mut last: usize = word_chars.len();

        while first < word_chars.len() {
            last -= 1;

            if word_chars[first] != word_chars[last] {
                return false;
            }

            first += 1;
        }

        true
    }
}
