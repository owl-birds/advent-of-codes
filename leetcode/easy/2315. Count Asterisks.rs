impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut count: i32 = 0;
        let mut is_in_vertical: bool = false;
        // guaranteed even number of '|' vertical bar char
        for c in s.chars() {
            if c == '|' {
                is_in_vertical = !is_in_vertical;
            }
            if !is_in_vertical && c == '*' {
                count += 1
            }
        }

        count
    }
}
