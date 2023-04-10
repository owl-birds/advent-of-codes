impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut open_brackets: Vec<char> = vec![];
        for p in s.chars(){
            if p == '}' || p == ')' || p == ']'{
                if open_brackets.is_empty(){
                    return false;
                }
                let temp_bracket: char = open_brackets[open_brackets.len()-1].clone();
                if p == ']' && temp_bracket != '['{return false}
                if p == '}' && temp_bracket != '{'{return false}
                if p == ')' && temp_bracket != '('{return false}
                open_brackets.pop();
                continue;
            }
            open_brackets.push(p);
        }
        open_brackets.is_empty()
    }
}