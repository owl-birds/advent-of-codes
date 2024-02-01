impl Solution {
    pub fn min_max_game(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return i32::MIN;
        }
        let mut temp_nums: Vec<i32> = nums.to_vec();
        Self::recurse(&mut temp_nums);
        temp_nums[0]
    }
    pub fn recurse(nums: &mut Vec<i32>) {
        if nums.len() == 1 {
            return;
        }
        let mut temp: Vec<i32> = vec![0; nums.len() / 2];
        for i in 0..temp.len() {
            if i % 2 == 0 {
                temp[i] = if nums[i * 2] > nums[i * 2 + 1] {
                    nums[i * 2 + 1]
                } else {
                    nums[i * 2]
                };
            } else {
                temp[i] = if nums[i * 2] < nums[i * 2 + 1] {
                    nums[i * 2 + 1]
                } else {
                    nums[i * 2]
                };
            }
        }
        *nums = temp;

        Self::recurse(nums);
    }
}
