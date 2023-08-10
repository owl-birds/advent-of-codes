impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.len() == 1 {
            return nums[0] == target;
        }
        // let mut sepr_idx: usize = 0;
        // while sepr_idx < nums.len() {
        //     sepr_idx += 1;
        //     if sepr_idx < nums.len() && nums[sepr_idx] < nums[sepr_idx-1] {break}
        // }
        // println!("separate idx: {}", sepr_idx);
        if nums[0] >= nums[nums.len() - 1] {
            // NOT GONNA WORK IF THERE RE EXIST SO MANY DUPLICATES
            // let mut l: usize = 0;
            // let mut r: usize = nums.len()-1;
            // while l < r {
            //     let m : usize = (l+r)/2;
            //     if nums[m] >= nums[0] {
            //         l = m + 1;
            //         continue;
            //     }
            //     r = m;
            // }
            // println!("{}", l);
            // NOT GONNA WORK IF THERE RE EXIST SO MANY DUPLICATES
            let mut sepr_idx: usize = 0;
            while sepr_idx < nums.len() {
                if sepr_idx < nums.len() && nums[sepr_idx] == target {
                    return true;
                }
                if sepr_idx < nums.len() && sepr_idx > 0 && nums[sepr_idx] < nums[sepr_idx - 1] {
                    break;
                }
                sepr_idx += 1;
            }
            // println!("separate idx: {}", sepr_idx);
            if Self::bin_search_bool(&nums, target, sepr_idx + 1, nums.len() - 1) {
                return true;
            }
        } else {
            if Self::bin_search_bool(&nums, target, 0, nums.len() - 1) {
                return true;
            }
        }

        false
    }
    pub fn bin_search_bool(nums: &Vec<i32>, target: i32, left: usize, right: usize) -> bool {
        let mut l: usize = left;
        let mut r: usize = right;

        while l < r {
            let m: usize = (l + r) / 2;
            if nums[m] == target {
                return true;
            }
            if nums[m] > target {
                r = m;
                continue;
            }
            l = m + 1;
        }
        if l < nums.len() && nums[l] == target {
            return true;
        }

        false
    }
}
