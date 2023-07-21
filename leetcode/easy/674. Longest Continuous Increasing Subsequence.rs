impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut max: i32 = 1;
        // let mut is_increasing: bool = false;
        let mut idx: usize = 0;

        while idx < nums.len() {
            let mut count: i32 = 1;
            while idx < nums.len() - 1 && nums[idx] < nums[idx + 1] {
                count += 1;
                idx += 1;
            }
            if count == 1 {
                idx += 1;
                continue;
            }
            if count > max {
                max = count
            }
        }

        max
    }
}
