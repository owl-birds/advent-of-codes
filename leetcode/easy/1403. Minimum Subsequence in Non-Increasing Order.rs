impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        // if nums.len() == 1 {return nums.clone();}
        let mut nums_sorted: Vec<i32> = nums.clone();
        nums_sorted.sort();
        let mut temp_sum: i32 = 0;
        for i in 0..nums_sorted.len() {
            temp_sum += nums_sorted[i];
        }
        let mut idx: usize = nums_sorted.len();
        let mut result: Vec<i32> = vec![];
        let mut result_sum: i32 = 0;
        while idx >= 1 {
            idx -= 1;
            result_sum += nums_sorted[idx];
            result.push(nums_sorted[idx]);
            temp_sum -= nums_sorted[idx];
            if temp_sum < result_sum {
                // println!("break: {:?}", result);
                break;
            }
        }

        result
    }
}
