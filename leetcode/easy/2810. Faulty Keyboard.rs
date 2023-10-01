impl Solution {
    pub fn final_string(s: String) -> String {
        let mut temp: Vec<char> = vec![];

        for c in s.chars() {
            if c == 'i' {
                temp.reverse();
                continue;
            }
            temp.push(c);
        }

        let mut result: String = String::new();
        for c in temp {
            result.push(c);
        }

        result
    }
}
