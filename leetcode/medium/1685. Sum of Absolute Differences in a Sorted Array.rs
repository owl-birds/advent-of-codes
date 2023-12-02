impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        // if nums.len() == 1 {return vec![0]}
        let mut result: Vec<i32> = vec![];
        let mut prefix: Vec<i32> = vec![nums[0]];
        for i in 1..nums.len() {
            prefix.push(prefix[prefix.len() - 1] + nums[i]);
        }
        // println!("{:?}", prefix);
        for i in 0..nums.len() {
            if i == 0 {
                result.push(
                    (prefix[prefix.len() - 1] - prefix[i])
                        - (nums[i] * (nums.len() - i - 1) as i32),
                );
                continue;
            }
            if i == (nums.len() - 1) {
                result.push((nums[i] * i as i32) - (prefix[i] - nums[i]));
                break;
            }
            let mut before: i32 = (nums[i] * i as i32) - prefix[i - 1];
            let mut after: i32 =
                (prefix[prefix.len() - 1] - prefix[i]) - (nums[i] * (nums.len() - i - 1) as i32);
            result.push(before + after);
        }

        // // SLOW
        // for i in 0..nums.len() {
        //     let mut temp: i32 = 0;
        //     for j in 0..nums.len() {
        //         temp += (i32::abs(nums[i]-nums[j]));
        //     }
        //     result.push(temp);
        // }
        // // SLOW

        result
    }
}
