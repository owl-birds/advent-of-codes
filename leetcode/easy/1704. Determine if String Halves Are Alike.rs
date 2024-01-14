use std::collections::HashSet;
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let vowels: HashSet<char> =
            HashSet::from(['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']);
        let mut first_half: i32 = 0;
        let mut second_half: i32 = 0;

        let temp: Vec<char> = s.chars().collect();
        let mut end: usize = temp.len();

        for start in 0..temp.len() / 2 {
            end -= 1;
            if vowels.contains(&temp[start]) {
                first_half += 1;
            }
            if vowels.contains(&temp[end]) {
                second_half += 1;
            }
        }

        first_half == second_half
    }
}
