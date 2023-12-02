impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let mut w_1: String = String::new();
        let mut w_2: String = String::new();

        for w in &word1 {
            w_1.push_str(&w[..]);
        }
        for w in &word2 {
            w_2.push_str(&w[..]);
        }

        w_1 == w_2
    }
}
