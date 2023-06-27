impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        // MEMORY EXCEDED
        let mut temp_sum_nums: Vec<Vec<i32>> = vec![];
        // [ [sum, num1, num2] ]

        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                temp_sum_nums.push(vec![nums1[i], nums2[j]]);
            }
        }
        temp_sum_nums.sort_by(|a, b| (a[0] + a[1]).cmp(&(&b[0] + &b[1])));
        // println!("{:?}", temp_sum_nums);

        let mut result: Vec<Vec<i32>> = vec![];
        // for i in 0..k as usize {
        //     if i >= temp_sum_nums.len() {break}
        //     result.push(vec![temp_sum_nums[i][1], temp_sum_nums[i][2]]);
        // }

        if temp_sum_nums.len() >= k as usize {
            // println!("{:?}", &temp_sum_nums[0..k as usize].to_vec());
            let result = &temp_sum_nums[0..k as usize].to_vec();
            return result.to_vec();
        } else {
            // println!("{:?}", temp_sum_nums);
            return temp_sum_nums;
        }
        // result
        // MEMORY EXCEDED
    }
}
