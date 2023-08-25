// use std::collections::VecDeque; // better stack then Vec
impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        // the solution below need to be optimized, its shitty and smelly as hell

        let mut s_vec: Vec<char> = s.chars().collect();
        let mut bracket_stack: Vec<usize> = vec![];
        let mut subs_stack: Vec<Vec<char>> = vec![];

        // need to be filtered, the s_vec
        let mut first: Vec<char> = vec![];
        let mut last: Vec<char> = vec![];
        while s_vec.len() > 0 && s_vec[0] != '(' {
            first.push(s_vec[0]);
            s_vec.remove(0);
        }
        let mut idx: usize = s_vec.len() - 1;
        while idx < s_vec.len() && s_vec.len() > 0 && s_vec[idx] != ')' {
            last.push(s_vec[idx]);
            s_vec.pop();
        }
        last.reverse();
        // println!("{:?}", s_vec);
        // need to be filtered, the s_vec

        let mut idx: usize = 0;
        while idx < s_vec.len() {
            if s_vec[idx] == '(' {
                let mut temp_stack: Vec<char> = vec![];
                idx += 1;
                while idx < s_vec.len() && s_vec[idx] != ')' && s_vec[idx] != '(' {
                    temp_stack.push(s_vec[idx]);
                    idx += 1;
                }
                subs_stack.push(temp_stack);
                bracket_stack.push(subs_stack.len() - 1);
                continue;
            }
            let mut temp_stack: Vec<char> = vec![];
            let mut bracket: usize = bracket_stack.pop().unwrap();
            // println!("idx:{}::s_length:{}", bracket, subs_stack.len());
            for i in bracket..subs_stack.len() {
                for j in 0..subs_stack[i].len() {
                    temp_stack.push(subs_stack[i][j]);
                }
            }
            let stack_length: usize = subs_stack.len();
            while bracket < stack_length {
                subs_stack.pop();
                bracket += 1;
            }
            // println!("before;;;{:?}", subs_stack);
            // println!("before reverse: {:?}", temp_stack);
            temp_stack.reverse();
            // println!("reverse: {:?}", temp_stack);
            idx += 1;
            while idx < s_vec.len() && s_vec[idx] != ')' && s_vec[idx] != '(' {
                temp_stack.push(s_vec[idx]);
                idx += 1;
            }
            // println!("reverse and plus-->{:?}", temp_stack);
            subs_stack.push(temp_stack);
            // println!("after;;;{:?}", subs_stack);
        }
        // println!("{:?}", subs_stack);
        // println!("{:?}", bracket_stack);
        let mut result: String = String::from("");
        for i in 0..first.len() {
            result.push(first[i]);
        }
        if subs_stack.len() > 0 {
            for i in 0..subs_stack.len() {
                for j in 0..subs_stack[i].len() {
                    result.push(subs_stack[i][j]);
                }
            }
        }
        for i in 0..last.len() {
            result.push(last[i]);
        }

        result
    }
}
