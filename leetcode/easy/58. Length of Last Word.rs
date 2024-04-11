impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut length: i32 = 0;
        let mut is_letter: bool = false;
        let mut idx: usize = s.len();
        let mut temp: Vec<char> = s.chars().collect();

        while idx >= 1 {
            idx -= 1;
            if temp[idx] != ' ' {
                is_letter = true;
            }
            if is_letter && temp[idx] == ' ' {
                break;
            }
        }
        if temp[idx] == ' ' {
            idx += 1;
        }
        // println!("{}", idx);
        while idx < temp.len() && temp[idx] != ' ' {
            length += 1;
            idx += 1;
        }

        length
    }
}
