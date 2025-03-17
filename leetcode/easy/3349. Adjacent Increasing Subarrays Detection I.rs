impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        for i in 0..nums.len() {
            let mut temp_idx: usize = i + 1;
            let mut is_increasing: bool = true;
            while temp_idx < nums.len() && temp_idx < i + k as usize {
                if nums[temp_idx] <= nums[temp_idx - 1] {
                    is_increasing = false;
                    break;
                }
                temp_idx += 1;
            }
            // println!("i:{}--temp_idx:{}--is_increasing:{}", i, temp_idx, is_increasing);
            if is_increasing {
                let mut is_increasing_2: bool = true;
                let starting_point: usize = temp_idx;
                temp_idx += 1;
                while temp_idx < nums.len() && temp_idx < starting_point + k as usize {
                    if nums[temp_idx] <= nums[temp_idx - 1] {
                        is_increasing_2 = false;
                        break;
                    }
                    temp_idx += 1;
                }
                if is_increasing_2 && (temp_idx - starting_point) as i32 == k {
                    return true;
                }
                // println!("starting_point:{}--temp_idx:{}--is_increasing_2:{}", starting_point, temp_idx, is_increasing_2);
            }
        }

        false
    }
}
