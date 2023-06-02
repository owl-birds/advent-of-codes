impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut temp_nums: Vec<i32> = nums.clone();
        temp_nums.sort();
        if (temp_nums[temp_nums.len() - 1] / 2) >= temp_nums[temp_nums.len() - 2] {
            for i in 0..nums.len() {
                if nums[i] == temp_nums[temp_nums.len() - 1] {
                    return i as i32;
                }
            }
        }
        -1
    }
}
