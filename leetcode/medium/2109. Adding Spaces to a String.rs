impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut result: String = String::new();

        let mut count: i32 = 0;
        let mut space_idx: usize = 0;

        for c in s.chars() {
            if space_idx < spaces.len() && count == spaces[space_idx] {
                result.push(' ');
                space_idx += 1;
            }
            result.push(c);
            count += 1;
        }

        result
    }
}
