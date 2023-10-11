impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![-1, -1];
        if nums.len() == 0 {
            return result;
        }
        let mut l: usize = 0;
        let mut r: usize = nums.len() - 1;

        while l < r {
            let m: usize = (l + r) / 1;
            if nums[m] == target {
                l = m;
                break;
            }
            if nums[m] > target {
                r = m - 1;
                continue;
            }
            l = m + 1;
        }
        // println!("{}", nums[l]);
        if l < nums.len() && nums[l] == target {
            while l > 0 && nums[l] == target {
                l -= 1;
            }
            l = if nums[l] == target { l } else { l + 1 };
            result[0] = l as i32;
            while l < nums.len() && nums[l] == target {
                l += 1;
            }
            l -= 1;
            result[1] = l as i32;
        }

        result
    }
}
