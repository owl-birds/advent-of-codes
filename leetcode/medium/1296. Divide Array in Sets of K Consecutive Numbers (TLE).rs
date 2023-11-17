use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() as i32 % k != 0 {
            return false;
        }
        // let mut num_set: HashSet<i32> = HashSet::new();
        // let mut num_map: HashMap<i32, i32> = HashMap::new();
        // for num in &nums {
        //     // num_set.insert(*num);
        //     if let Some(freq) = num_map.get_mut(num) {
        //         *freq += 1;
        //     } else {
        //         num_map.insert(*num, 1);
        //     }
        // }

        // if (num_set.len() as i32) <= k {return false}
        // if (num_map.len() as i32) < k {return false}
        ///
        let mut temp: Vec<i32> = nums.to_vec();
        temp.sort_by(|a, b| a.cmp(&b));

        let mut num_set: HashSet<usize> = HashSet::new();
        let mut curr_num: i32 = temp[0];
        num_set.insert(0);
        let mut idx: usize = 1;
        let mut count: i32 = 1;
        // let mut len: i32 = temp.len() as i32 - 1;
        let mut last_idx: usize = usize::MAX;

        while idx < temp.len() {
            // println!("{:?}-last_idx:{}-idx:{}--curr_num:{}\ncount:{}--len:{}\n---", num_set, last_idx, idx, curr_num, count, len);
            if count == k {
                count = 1;
                idx = if last_idx == usize::MAX {
                    idx
                } else {
                    last_idx
                };
                curr_num = temp[idx];
                num_set.insert(idx);
                last_idx = usize::MAX;
                // len -= 1;
                continue;
            }
            if num_set.contains(&idx) {
                idx += 1;
                continue;
            }
            if curr_num == temp[idx] {
                last_idx = if last_idx == usize::MAX {
                    idx
                } else {
                    last_idx
                };
                idx += 1;
                continue;
            }
            if (curr_num + 1) != temp[idx] {
                return false;
            }
            num_set.insert(idx);
            curr_num = temp[idx];
            count += 1;
            idx += 1;
            // len -= 1;
        }
        // println!("{:?}-last_idx:{}-idx:{}--curr_num:{}\ncount:{}--len:{}\n---", num_set, last_idx, idx, curr_num, count, len);

        // len == 0
        num_set.len() == temp.len()
    }
}
