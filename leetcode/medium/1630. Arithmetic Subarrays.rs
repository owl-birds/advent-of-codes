impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut result: Vec<bool> = vec![];

        for i in 0..l.len() {
            let mut is_arith: bool = true;
            let mut curr_vec: Vec<i32> = vec![];

            for idx in l[i]..r[i] + 1 {
                curr_vec.push(nums[idx as usize]);
            }
            curr_vec.sort_by(|a, b| a.cmp(&b));
            let diff: i32 = curr_vec[1] - curr_vec[0];
            for idx in 2..curr_vec.len() {
                if (curr_vec[idx] - curr_vec[idx - 1]) != diff {
                    is_arith = false;
                    break;
                }
            }

            result.push(is_arith);
        }

        result
    }
}
