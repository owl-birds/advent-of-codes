impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut count: i32 = 0;

        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if Self::is_prefix_suffix(&words[i], &words[j]) {
                    count += 1;
                }
            }
        }

        count
    }
    pub fn is_prefix_suffix(str1: &String, str2: &String) -> bool {
        if str1.len() > str2.len() {
            return false;
        }
        let temp1: Vec<char> = str1.chars().collect();
        let temp2: Vec<char> = str2.chars().collect();
        // check prefix
        for i in 0..temp1.len() {
            if temp1[i] != temp2[i] {
                return false;
            }
        }
        // check suffix
        let mut i = temp1.len();
        let mut j = temp2.len();
        while i > 0 {
            i -= 1;
            j -= 1;
            if temp1[i] != temp2[j] {
                return false;
            }
        }
        true
    }
}
