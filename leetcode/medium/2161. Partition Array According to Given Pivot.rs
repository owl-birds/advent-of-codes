impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut count_pivot: u32 = 0;

        for i in 0..nums.len() {
            if nums[i] == pivot {
                count_pivot += 1;
                continue;
            }
            if nums[i] < pivot {
                result.push(nums[i]);
            }
        }
        while count_pivot >= 1 {
            count_pivot -= 1;
            result.push(pivot);
        }
        for i in 0..nums.len() {
            if nums[i] > pivot {
                result.push(nums[i])
            }
        }

        result
    }
}
