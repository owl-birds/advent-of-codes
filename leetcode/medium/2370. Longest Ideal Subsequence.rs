use std::collections::HashMap;
impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let mut dp: Vec<i32> = vec![0];
        let mut len: i32 = 0;
        //
        // let mut let_order: HashMap<char, i32> = HashMap::new();
        // let mut order: i32 = 1;
        // for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        //     let_order.insert(c, order);
        //     order += 1;
        // }
        //

        let mut char_len: HashMap<char, i32> = HashMap::new();
        // for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        //     char_len.insert(c, 0);
        // }
        let mut temp: Vec<char> = s.chars().collect();

        for i in 0..temp.len() {
            let mut max: i32 = 0;
            let curr_ord: i32 = Self::find_order(temp[i]);
            for (c, l) in char_len.iter() {
                let prev_ord: i32 = Self::find_order(*c);
                let abs_diff: i32 = curr_ord.abs_diff(prev_ord) as i32;
                if abs_diff <= k && *l > max {
                    max = *l;
                }
            }
            // dp.push(max + 1);
            char_len.insert(temp[i], max + 1);
            if max + 1 > len {
                len = max + 1;
            }
        }

        // println!("{:?}", let_order);
        // println!("{:?}", char_len);
        // println!("{:?}", dp);
        // println!("a:{}", 'a' as usize);
        // println!("a:{}", Self::find_order('a'));
        // println!("z:{}", 'z' as usize);
        // println!("z:{}", Self::find_order('z'));
        // println!("z-a:{}", 'z' as usize - 'a' as usize);
        // println!("b-a:{}", 'b' as usize - 'a' as usize);

        len
    }
    pub fn find_order(c: char) -> i32 {
        (c as usize - 'a' as usize + 1) as i32
    }
}
