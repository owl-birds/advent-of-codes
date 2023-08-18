impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut diffs: Vec<i32> = vec![];
        let mut same: Vec<i32> = vec![];
        for i in 0..nums.len() {
            if nums[i] != val {
                diffs.push(nums[i]);
                continue;
            }
            same.push(nums[i]);
        }
        let mut idx: usize = 0;
        while idx < diffs.len() {
            nums[idx] = diffs[idx];
            idx += 1;
        }
        for num in same {
            nums[idx] = num;
            idx += 1;
        }

        diffs.len() as i32
    }
}
