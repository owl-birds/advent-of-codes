impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        // IM LOST, IDK
        // inspiration : https://leetcode.com/problems/maximum-nesting-depth-of-two-valid-parentheses-strings/solutions/358419/confused-by-this-problem-i-was-too-here-is-how-it-became-crystal-clear/
        let mut result: Vec<i32> = vec![];
        let mut s_vec: Vec<char> = seq.chars().collect();
        let mut depth: i32 = 0;

        for i in 0..s_vec.len() {
            if s_vec[i] == '(' {
                depth += 1;
                // result.push(depth);
                // continue;
            }
            result.push(depth % 2);
            if s_vec[i] == ')' {
                depth -= 1;
            }
        }

        result
    }
}
