impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut ans: String = String::new();
        let mut curr_num: i32 = i32::MIN;
        let mut idx: usize = 0;
        let num_chars: Vec<char> = num.chars().collect();

        while idx < num_chars.len() - 2 {
            if num_chars[idx] == num_chars[idx + 1] {
                let mut count: i32 = 0;
                let c: char = num_chars[idx];
                // println!("num:{}", num_chars[idx]);
                // println!("before----c:{}", idx);
                // println!(">{}", idx);
                while idx < num_chars.len() && num_chars[idx] == c {
                    // println!("{}", num_chars[idx]);
                    count += 1;
                    idx += 1;
                }
                // println!("after----c:{}", idx);
                // println!("---");
                if count >= 3 {
                    let mut temp_ans: String = format!("{}{}{}", c, c, c);
                    let mut multiply: i32 = 100;
                    let mut temp_num: i32 = 0;
                    while multiply > 0 {
                        temp_num += (c.to_digit(10).unwrap() as i32 * multiply);
                        multiply /= 10;
                    }
                    // println!("{}", temp_num);
                    if temp_num > curr_num {
                        curr_num = temp_num;
                        ans = temp_ans;
                    }
                }
                continue;
            }
            idx += 1;
        }

        ans
    }
}
