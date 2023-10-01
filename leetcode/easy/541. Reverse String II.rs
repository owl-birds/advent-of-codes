impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut temp: Vec<char> = s.chars().collect();
        let mut result: String = String::new();
        let mut idx: usize = if s.len() < k as usize {
            s.len()
        } else {
            k as usize
        };
        // let mut idx: usize = 0;

        if idx == temp.len() {
            while idx > 0 {
                idx -= 1;
                result.push(temp[idx]);
            }
        } else {
            while idx < temp.len() {
                // println!("idx:{}",idx);
                let mut temp_i: usize = idx;
                while temp_i > idx - k as usize {
                    temp_i -= 1;
                    result.push(temp[temp_i]);
                }
                let next_group: usize = idx + k as usize;
                while idx < temp.len() && idx < next_group {
                    result.push(temp[idx]);
                    idx += 1;
                }
                // println!("after_idx:{}",idx);
                if (idx + k as usize) >= temp.len() {
                    temp_i = temp.len();
                    while temp_i > idx {
                        temp_i -= 1;
                        result.push(temp[temp_i]);
                    }
                }
                idx += k as usize;
            }
            // println!("idx:{}",idx);
        }

        result
    }
}
