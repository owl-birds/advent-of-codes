impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut longest_prefix: String = String::from("");
        let first_word: String = strs[0].clone();
        for length in 1..first_word.len()+1{
            let temp_prefix: &str = &first_word[0..length];
            // println!("prefix: {}", temp_prefix);
            for i in 1..strs.len(){
                if strs[i].len() < length{
                    return longest_prefix;
                }
                if temp_prefix != &strs[i][0..length]{
                    return longest_prefix;
                }
            }
            longest_prefix = String::from(temp_prefix);
        }
        return longest_prefix;
    }
}
