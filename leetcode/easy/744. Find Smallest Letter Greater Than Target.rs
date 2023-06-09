impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let target_unicode: u32 = target as u32;

        // println!("{}", 'a' as u32);
        // println!("{}", 'b' as u32);

        for i in 0..letters.len() {
            if letters[i] as u32 > target_unicode {
                return letters[i];
            }
        }

        letters[0]
    }
}
