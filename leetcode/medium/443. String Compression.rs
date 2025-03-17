impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.len() <= 1 {
            return chars.len() as i32;
        }
        // let mut res: i32 = 0;
        let mut compressed: Vec<char> = vec![];
        let mut idx: usize = 0;

        while idx < chars.len() {
            if idx < chars.len() - 1 && chars[idx] == chars[idx + 1] {
                let mut count: u32 = 1;
                while idx < chars.len() - 1 && chars[idx] == chars[idx + 1] {
                    count += 1;
                    idx += 1;
                }
                if count < 10 {
                    compressed.push(chars[idx]);
                    compressed.push(char::from_digit(count, 10).unwrap());
                } else {
                    compressed.push(chars[idx]);
                    let mut temp: Vec<char> = vec![];
                    while count > 0 {
                        temp.push(char::from_digit(count % 10, 10).unwrap());
                        count /= 10;
                    }
                    let mut temp_idx: usize = temp.len();
                    while temp_idx > 0 {
                        temp_idx -= 1;
                        compressed.push(temp[temp_idx]);
                    }
                }
                idx += 1;
                continue;
            }
            // println!("{}", idx);
            compressed.push(chars[idx]);
            idx += 1;
        }
        // println!("{:?}", compressed);
        *chars = compressed.to_vec();
        compressed.len() as i32
    }
}
