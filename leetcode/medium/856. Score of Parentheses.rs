impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        // let mut idxs: Vec<Vec<usize>> = vec![];

        // TY : https://leetcode.com/problems/score-of-parentheses/solutions/1856519/java-c-visually-explained/
        //     The score of a balanced parentheses string is based on the following rule:

        // "()" has score 1.
        // AB has score A + B, where A and B are balanced parentheses strings.
        // (A) has score 2 * A, where A is a balanced parentheses string.

        let mut s_vec: Vec<char> = s.chars().collect();
        let mut stack: Vec<i32> = vec![];
        let mut score: i32 = 0;

        for i in 0..s_vec.len() {
            // println!("{}-->{:?}", i, stack);
            if s_vec[i] == '(' {
                stack.push(0);
                continue;
            }
            if stack[stack.len() - 1] == 0 {
                stack.pop();
                stack.push(1);
                continue;
            } // ---> ()
            let mut poped: i32 = 0;
            while stack[stack.len() - 1] != 0 {
                poped += stack[stack.len() - 1];
                stack.pop();
            } // ---> A + B
            stack.pop();
            stack.push(2 * poped); // ---> 2 * A
        }
        // println!("-->{:?}", stack);
        for s in stack {
            score += s;
        }
        score
    }
}
