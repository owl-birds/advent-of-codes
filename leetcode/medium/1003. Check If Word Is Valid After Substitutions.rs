impl Solution {
    pub fn is_valid(s: String) -> bool {
        // let mut stack_a: Vec<usize> = vec![];
        // let mut stack_b: Vec<usize> = vec![];
        // let mut stack_c: Vec<char> = vec![];
        let mut stack: Vec<char> = vec![];
        // use three stack or nah
        let mut count: i32 = 0;
        let valid_count: i32 = s.len() as i32 / 3;
        // println!("valid_count : {}", valid_count);

        let s_chars: Vec<char> = s.chars().collect();
        for i in 0..s_chars.len() {
            if s_chars[i] == 'c' {
                if stack.len() < 2 {
                    return false;
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                if b != 'b' {
                    return false;
                }
                if a != 'a' {
                    return false;
                }
                count += 1;
                continue;
            }
            stack.push(s_chars[i]);
        }
        // println!("{:?}", stack_a);
        // println!("{:?}", stack_b);
        // println!("count : {}", count);

        // if stack_a.len() != 0 || stack_b.len() != 0 {
        //     return false
        // }

        if stack.len() != 0 {
            return false;
        }

        count == valid_count
    }
}
