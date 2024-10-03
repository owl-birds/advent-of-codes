use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut count: i32 = 0;
        let mut letters_of_word: Vec<char> = word.chars().collect();
        // let mut map_letters: HashMap<char, i32> = HashMap::new();
        // let mut lowercase: HashMap<char, i32> = HashMap::new();
        // let mut uppercase: HashMap<char, i32> = HashMap::new();
        let mut lowercase: HashSet<char> = HashSet::new();
        let mut uppercase: HashSet<char> = HashSet::new();
        for i in 0..letters_of_word.len() {
            if Self::is_uppercase_u32(letters_of_word[i]) {
                // if let Some(f) = uppercase.get_mut(&letters_of_word[i]) {
                //     *f += 1;
                // } else {
                //     uppercase.insert(letters_of_word[i], 1);
                // }
                uppercase.insert(letters_of_word[i]);
            } else {
                // if let Some(f) = lowercase.get_mut(&letters_of_word[i]) {
                //     *f += 1;
                // } else {
                //     lowercase.insert(letters_of_word[i], 1);
                // }
                lowercase.insert(letters_of_word[i]);
            }
        }
        // for (letter, lf) in lowercase.iter() {
        //     let upp: char = letter.to_ascii_uppercase();
        //     if let Some(uf) = uppercase.get(&upp) {
        //         count += if lf > uf {uf} else {lf};
        //     }
        // }
        for letter in lowercase.iter() {
            let upp: char = letter.to_ascii_uppercase();
            if uppercase.contains(&upp) {
                count += 1;
            }
        }

        // // TEST
        // println!("{:?}", '9'.to_digit(10));
        // println!("{:?}", u128::from('a'));
        // println!("A:{:?}", u32::from('A'));
        // println!("Z:{:?}", u32::from('Z'));
        // println!("{:?}", char::from_digit(9, 10));
        // println!("{:?}", char::from_digit(0, 2));
        // println!("{:?}", char::from_digit(1, 2));
        // println!("{:?}", char::from_u32(65));
        // println!("{:?}", char::from_u32(65+25));
        // println!("{:?}", char::from_u32(97));
        // println!("{:?}", char::from_u32(97+25));
        // println!("{:?}", char::from_u32(91));
        // println!("{:?}", char::from_u32(90));
        // println!("{:?}", letters_of_word);
        // println!("A:{:?}", Self::is_uppercase_u32('A'));
        // println!("Z:{:?}", Self::is_uppercase_u32('Z'));
        // println!("a:{:?}", Self::is_uppercase_u32('a'));
        // println!("z:{:?}", Self::is_uppercase_u32('z'));
        // println!("{:?}", uppercase);
        // println!("{:?}", lowercase);
        count
    }
    pub fn is_uppercase_u32(letter: char) -> bool {
        let ascii_num: u32 = u32::from(letter);
        ascii_num <= 90 && ascii_num >= 65
    }
}
