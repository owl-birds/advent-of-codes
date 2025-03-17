impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut temp_res: Vec<char> = vec![];
        let mut result: String = String::new();

        Self::recursion_rle(1, n, &mut temp_res);
        for c in temp_res.iter() {
            result.push(*c);
        }
        // println!("{:?}", temp_res);
        result
    }
    pub fn recursion_rle(curr_n: i32, nth: i32, compressed_str: &mut Vec<char>) {
        if curr_n > nth {
            return;
        }
        if curr_n == 1 {
            compressed_str.push('1');
            Self::recursion_rle(curr_n + 1, nth, compressed_str);
        } else if compressed_str.len() == 1 {
            compressed_str.push('1');
            Self::recursion_rle(curr_n + 1, nth, compressed_str);
        } else {
            let mut idx: usize = 0;
            let mut temp_comp: Vec<char> = vec![];
            while idx < compressed_str.len() {
                let mut count: i32 = 1;
                while idx < compressed_str.len() - 1
                    && compressed_str[idx] == compressed_str[idx + 1]
                {
                    count += 1;
                    idx += 1;
                }
                temp_comp.push(char::from_digit(count as u32, 10).unwrap());
                temp_comp.push(compressed_str[idx]);
                // println!("{}", count);
                idx += 1;
            }
            // println!("{:?}", temp_comp);
            *compressed_str = temp_comp;
            Self::recursion_rle(curr_n + 1, nth, compressed_str);
        }
    }
}
