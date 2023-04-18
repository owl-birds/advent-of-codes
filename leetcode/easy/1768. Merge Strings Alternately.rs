impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let longer_str: &String = if (word1.len() > word2.len()) {&word1} else {&word2};
        let shorter_str: &String = if (word1.len() > word2.len()) {&word2} else {&word1};
        let mut result_vec: Vec<char> = vec!['i'; shorter_str.len()*2+(longer_str.len()-shorter_str.len())];


        let mut idx = 0;
        let mut count = 0;

        for c in word1.chars(){
            if count == shorter_str.len(){
                break;
            }
            result_vec[idx] = c;
            count += 1;
            idx += 2;
        }
        idx = 1;
        count = 0;
        for c in word2.chars(){
            if count == shorter_str.len(){
                break;
            }
            result_vec[idx] = c;
            count += 1;
            idx += 2;
        }
        if longer_str.len() - shorter_str.len() > 0{
            idx -= 1;
            for c in longer_str[shorter_str.len()..].chars(){
                result_vec[idx] = c;
                idx += 1;
            }  
        }
        let mut result: String = String::from("");    

        for c in result_vec{
            result.push(c);
        }
        // u can push in String rust
        // println!("{} {}", longer_str, shorter_str);
        // println!("{:?}", result_vec);
        result
    }
}