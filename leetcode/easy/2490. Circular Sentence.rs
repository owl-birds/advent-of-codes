impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let mut sen_chars: Vec<char> = sentence.chars().collect();
        let mut words: Vec<Vec<char>> = vec![];
        let mut temp: Vec<char> = vec![];

        for i in 0..sen_chars.len() {
            if sen_chars[i] == ' ' {
                words.push(temp.to_vec());
                temp = vec![];
                continue;
            }
            temp.push(sen_chars[i]);
        }
        words.push(temp.to_vec());

        // first check
        for i in 1..words.len() {
            if words[i][0] != words[i - 1][words[i - 1].len() - 1] {
                return false;
            }
        }
        // second check
        if words.len() > 0
            && words[0][0] != words[words.len() - 1][words[words.len() - 1].len() - 1]
        {
            return false;
        }
        // println!("{:?}", words);
        // println!("{:?}\n{:?}",words[words.len()-1][words[words.len()-1].len()-1],words[0][words[0].len()-1]);
        true
    }
}
