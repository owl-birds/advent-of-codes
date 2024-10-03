impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut temp: Vec<i32> = nums.to_vec();
        let mut count: i64 = 0;
        temp.sort_by(|a, b| a.cmp(&b));
        let mut u_idx: usize = 1;

        while u_idx < temp.len() && temp[0] + temp[u_idx] <= upper {
            u_idx += 1;
        }
        u_idx -= 1;
        // if u_idx == 0 {return count}
        // while l_idx < temp.len() && temp[0] + temp[l_idx] < lower {
        //     l_idx += 1;
        // }
        // if l_idx == temp.len() {l_idx -= 1;}
        // println!("{:?}\nu:{}\nl:{}", temp, u_idx, l_idx);
        // println!("{:?}", temp);
        let mut idx: usize = 0;
        while idx < temp.len() - 1 && u_idx > idx {
            while u_idx > idx && temp[idx] + temp[u_idx] > upper {
                u_idx -= 1;
            }
            let mut l_idx: usize = idx + 1;
            while l_idx < temp.len() && temp[idx] + temp[l_idx] < lower {
                l_idx += 1;
            }
            if l_idx == temp.len() {
                l_idx -= 1;
            }
            // if u_idx < l_idx {break} // well the window is walking
            if u_idx < l_idx {
                idx += 1;
                continue;
            } // well the window is walking
            if temp[idx] + temp[u_idx] < lower {
                idx += 1;
                continue;
            }
            // println!("idx:{}-l:{}-u:{}", idx, l_idx, u_idx);
            // if u_idx == l_idx {count += 1}
            // else {count += (u_idx-l_idx+1) as i64;}
            count += (u_idx - l_idx + 1) as i64;
            idx += 1;
        }
        // println!("{:?}\nu:{}\nl:{}", temp, u_idx, l_idx);
        count
    }
}
