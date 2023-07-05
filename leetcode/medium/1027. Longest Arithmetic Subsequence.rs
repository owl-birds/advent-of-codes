use std::collections::HashMap;
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        if nums.len() == 2 {
            return 2;
        }
        let mut longest: i32 = 2; // the min lenght is 2 so yeah
                                  // or
                                  // let mut longest: i32 = 0;

        let mut dp: Vec<HashMap<i32, i32>> = vec![]; // storing prev, itereration,, the diff
        for i in 0..nums.len() {
            dp.push(HashMap::new());
            for j in 0..i {
                let diff: i32 = nums[i] - nums[j];
                let get_count = dp[j].get(&diff);
                let mut count_j: i32 = 0;
                match get_count {
                    Some(c_j) => {
                        count_j = *c_j;
                    }
                    None => {
                        dp[i].insert(diff, 2);
                        continue;
                    }
                }
                dp[i].insert(diff, count_j + 1);
                longest = if longest > (count_j + 1) {
                    longest
                } else {
                    count_j + 1
                };
            }
        }
        // println!("{:?}", dp);
        longest

        // // o(n^3) AND OF COURSE ITS SLOW AS HELL, TLE
        // for i in 0..nums.len() {
        //     if (nums.len() - i) <= longest as usize {break}
        //     for j in i+1..nums.len() {
        //         if (nums.len() - j) < longest as usize {break}
        //         let diff: i32 = nums[j] - nums[i];
        //         let mut temp_length = 2;
        //         let mut starting: i32 = 1;
        //         for k in j+1..nums.len() {
        //             // if ((nums.len() - k) + temp_length as usize) < longest as usize {break}
        //             if (nums[i] == 0 && nums[j] == 0 && nums[k] == 0) || (nums[i] == nums[j] && nums[j] == nums[k]) {
        //                 temp_length += 1;
        //                 continue;
        //             }
        //             let next_diff: i32 = nums[k] - nums[i];
        //             if diff != 0 && next_diff % diff == 0 && (next_diff / diff) == (starting+1) {
        //                 temp_length += 1;
        //                 starting += 1;
        //             }
        //         }
        //         // println!("not_max-->{}:{}::{}",nums[i],nums[j],diff);
        //         if temp_length > longest {
        //             println!("{}:{}::{}",nums[i],nums[j],diff);
        //             longest = temp_length;
        //         }
        //     }
        // }
        // longest
    }
}
