impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut result: i32 = 0;
        let mut temp: Vec<char> = s.chars().collect();
        let mut left: usize = 0;
        let mut right: usize = temp.len() - 1;

        while left < temp.len() && right >= 0 && left != right {
            if temp[left] != temp[right] {
                break;
            }
            let curr_char: char = temp[left];
            while left < temp.len() && temp[left] == curr_char {
                left += 1;
            }
            if left >= right {
                break;
            }
            while right > 0 && temp[right] == curr_char {
                if right == 0 {
                    break;
                }
                right -= 1;
            }
        }
        // println!("left:{}\nright:{}", left, right);

        if left > right {
            return 0;
        }
        result = (right - left + 1) as i32;
        result
    }
}
