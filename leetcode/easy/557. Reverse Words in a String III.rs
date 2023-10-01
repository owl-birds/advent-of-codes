impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut result: String = String::new();

        let mut temp: Vec<char> = s.chars().collect();
        for i in 0..temp.len() {
            if temp[i] == ' ' || i == (temp.len() - 1) {
                let mut idx: usize = if i == (temp.len() - 1) { i } else { i - 1 };
                while idx >= 0 && temp[idx] != ' ' {
                    result.push(temp[idx]);
                    if idx == 0 {
                        break;
                    }
                    idx -= 1;
                }
                if i == (temp.len() - 1) {
                    continue;
                }
                result.push(' ');
            }
        }

        result
    }
}
