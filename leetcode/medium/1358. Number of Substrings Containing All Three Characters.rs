use std::collections::HashMap;
impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut temp: Vec<char> = s.chars().collect();
        let mut count: i32 = 0;
        let mut idx: usize = 0;
        // let mut include: Vec<char
        while idx <= temp.len() - 2 {
            let mut f_a: i32 = -1;
            let mut f_b: i32 = -1;
            let mut f_c: i32 = -1;
            let mut temp_i: usize = idx;
            // println!("{}", idx);
            while temp_i < temp.len() {
                if f_a >= 0 && f_b >= 0 && f_c >= 0 {
                    break;
                }
                if f_a == -1 && temp[temp_i] == 'a' {
                    f_a = temp_i as i32;
                }
                if f_b == -1 && temp[temp_i] == 'b' {
                    f_b = temp_i as i32;
                }
                if f_c == -1 && temp[temp_i] == 'c' {
                    f_c = temp_i as i32;
                }
                temp_i += 1;
            }
            if f_a == -1 && f_b == -1 && f_c == -1 {
                break;
            }
            if f_a >= 0 && f_b >= 0 && f_c >= 0 {
                // println!("idx:{} a:{} b:{} c:{}", idx, f_a, f_b, f_c);
                let mut furthest_idx: i32 = -1;
                if f_a >= f_b && f_a >= f_c {
                    furthest_idx = f_a;
                }
                if f_b >= f_a && f_b >= f_c {
                    furthest_idx = f_b;
                }
                if f_c >= f_b && f_c >= f_a {
                    furthest_idx = f_c;
                }
                count += (temp.len() as i32 - furthest_idx);
            }
            idx += 1;
        }
        count
    }
}
