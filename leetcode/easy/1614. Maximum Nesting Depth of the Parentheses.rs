impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut s_vec: Vec<char> = s.chars().collect();
        // let mut depth: i32 = 0;
        let mut stack: Vec<i32> = vec![];

        for i in 0..s_vec.len() {
            if s_vec[i] != '(' && s_vec[i] != ')' {
                continue;
            }
            // println!("{}--->{:?}", i, stack);
            // print!("{}", s_vec[i]);
            if s_vec[i] == '(' {
                stack.push(0);
                continue;
            }
            if stack[stack.len() - 1] == 0 {
                stack.pop();
                stack.push(1);
                continue;
            }
            let mut max_depth: i32 = 0;
            while stack[stack.len() - 1] != 0 {
                if max_depth < stack[stack.len() - 1] {
                    max_depth = stack[stack.len() - 1];
                }
                stack.pop();
            }
            stack.pop();
            stack.push(max_depth + 1);
        }
        // println!("final--->{:?}", stack);
        if stack.len() == 0 {
            return 0;
        }
        // depth
        *stack.iter().max().unwrap()
    }
}
