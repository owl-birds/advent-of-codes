use std::collections::HashMap;
impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        // if arr.len() == 1 {return 1}
        // o (n^2) :: TLE
        let mut longest: i32 = 1;
        let mut dp: Vec<HashMap<i32, i32>> = vec![];
        for i in 0..arr.len() {
            dp.push(HashMap::new());
            for j in 0..i {
                let diff: i32 = arr[i] - arr[j];
                if diff != difference {
                    continue;
                }
                let get_count_j = dp[j].get_mut(&diff);
                let mut count_j: i32 = 0;
                match get_count_j {
                    Some(c_j) => count_j = *c_j,
                    None => {
                        dp[i].insert(diff, 2);
                        if (diff == difference) && longest == 1 {
                            longest = 2;
                        }
                        continue;
                    }
                }
                if (diff == difference) && longest < (count_j + 1) {
                    longest = count_j + 1;
                }
                dp[i].insert(diff, count_j + 1);
            }
        }
        // println!("{:?}", dp);
        longest
    }
}
