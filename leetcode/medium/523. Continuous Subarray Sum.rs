use std::collections::HashMap;
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() < 2 {
            return false;
        }

        // inspiration :: https://leetcode.com/problems/continuous-subarray-sum/description/comments/1564916
        // Basically you want to create an array of the accumulated sum, but instead of the sum, you have the sum%k. If you just go through it normally and return on sum%k == 0, then that only accounts for (n) possibilities out of (n^2) possibilities. However, if you find duplicated sum%k values, then that the sub array between those two indexes will actually be the solution.

        // i.e.
        // [4, 1, 2, 3] and 6

        //     if we get the accumulated sum, it looks like this [4, 5, 7, 10]
        //     if we make it accumulated sum % k, it looks like this [4, 5, 1, 4]
        //     notice that there is duplicated 4's. The diffference between these two sums in theory must be a multiple of 6 since we've only been storing the num%k.

        let mut prefix_mod_sum: Vec<i32> = vec![];
        let mut mod_map: HashMap<i32, usize> = HashMap::new();
        let mut temp_sum: i32 = 0;
        for i in 0..nums.len() {
            temp_sum += nums[i];
            prefix_mod_sum.push(temp_sum % k);
            if prefix_mod_sum.len() >= 2 && temp_sum % k == 0 {
                return true;
            }
        }
        // println!("{:?}", prefix_mod_sum);
        for i in 0..prefix_mod_sum.len() {
            if let Some(mod_idx) = mod_map.get(&prefix_mod_sum[i]) {
                if i - *mod_idx > 1 {
                    return true;
                } // cant be consecutive [0,1,1] NO -> [1,0,1] YES or [1,2,3,4,5,1] YES
            } else {
                mod_map.insert(prefix_mod_sum[i], i);
            }
        }
        // println!("{:?}", mod_map);
        false

        // SLOW TOO, STINKY
        // let mut prefix_sum: Vec<i32> = vec![];
        // let mut temp_sum: i32 = 0;
        // for i in 0..nums.len() {
        //     temp_sum += nums[i];
        //     prefix_sum.push(temp_sum);
        //     if prefix_sum.len() >= 2 {
        //         let mut idx: usize = 0;
        //         if prefix_sum[prefix_sum.len()-1] % k == 0 {return true;}
        //         while idx < prefix_sum.len() {
        //             if (prefix_sum.len()-1) - idx < 2 {break;}
        //             if (prefix_sum[prefix_sum.len()-1] - prefix_sum[idx]) % k == 0 {return true;}
        //             idx += 1;
        //         }
        //     }
        // }
        // println!("{:?}", prefix_sum);
        // for i in 0..prefix_sum.len() {
        //     let mut idx: usize = 0;
        //     if i > 0 && prefix_sum[i] % k == 0 {
        //         return true;
        //     }
        //     while idx < i {
        //         if (i-idx) >= 2 && (prefix_sum[i]-prefix_sum[idx]) % k == 0 {
        //             return true;
        //         }
        //         idx += 1;
        //     }
        // }
        // false

        // // SLOW WAY | TLE
        // let mut starting_length: usize = 2;

        // while starting_length <= nums.len() {
        //     let mut sum: i32 = 0;
        //     for i in 0..starting_length as usize {
        //         sum += nums[i];
        //     }
        //     if sum % k == 0 {return true}
        //     let mut idx: usize = starting_length;
        //     while idx < nums.len() {
        //         sum += nums[idx];
        //         sum -= nums[idx-starting_length];
        //         if sum % k == 0 {return true}
        //         idx += 1;
        //     }
        //     // println!("{}", starting_length);
        //     starting_length += 1;
        // }
        // false
    }
}
