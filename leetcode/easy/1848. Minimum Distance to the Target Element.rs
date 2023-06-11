impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        if nums[start as usize] == target {
            return 0;
        }
        let mut left: i32 = start - 1;
        let mut right: i32 = start + 1;
        let mut left_found: i32 = -1;
        let mut right_found: i32 = -1;
        while left >= 0 || right < nums.len() as i32 {
            if left_found != -1 && right_found != -1 {
                break;
            }
            if left >= 0 {
                if nums[left as usize] == target && left_found == -1 {
                    left_found = left;
                }
                left -= 1;
            }
            if right < nums.len() as i32 {
                if nums[right as usize] == target && right_found == -1 {
                    right_found = right;
                }
                right += 1;
            }
        }
        // println!("{} {}", left_found, right_found);
        if left_found != -1 && right_found != -1 {
            let left_start: i32 = i32::abs(start - left_found);
            let right_start: i32 = i32::abs(start - right_found);
            if left_start < right_start {
                return left_start;
            }
            return right_start;
        }
        if left_found != -1 {
            return i32::abs(start - left_found);
        }
        i32::abs(start - right_found)
    }
}
