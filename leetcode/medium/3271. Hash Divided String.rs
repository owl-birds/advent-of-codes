impl Solution {
    pub fn string_hash(s: String, k: i32) -> String {
        let mut hashed_str: String = String::new();
        let mut temp_cs: Vec<char> = s.chars().collect();
        let b_len: usize = temp_cs.len();
        let a_len: usize = temp_cs.len() / k as usize;
        // if b_len == 1 {
        //     let ascii_val: u32 = temp_cs[0] as u32;
        //     let hashed_val = ascii_val - 97;
        //     let hashed_char = char::from_u32((hashed_val % 26) + 97);
        //     hashed_str.push(hashed_char.unwrap());
        //     // println!("hashed_val : {}\nhashed_char : {:?}", hashed_val, hashed_char);
        // }
        // else {
        let mut idx: usize = 0;
        while idx < temp_cs.len() {
            let mut hashed_val = 0;
            let start_idx = idx;
            while idx < temp_cs.len() && idx < start_idx + k as usize {
                hashed_val += (temp_cs[idx] as u32 - 97);
                idx += 1;
            }
            hashed_val %= 26;
            let hashed_char = char::from_u32(hashed_val + 97);
            match hashed_char {
                Some(c) => {
                    hashed_str.push(c);
                }
                None => {}
            }
        }
        // }

        // println!("{:?}\n{}\n{}\n{}", temp_cs, a_len, 'a' as usize, 97 as char);
        // println!("{}\n{}", 'A' as usize, 65 as char);
        hashed_str
    }
}
