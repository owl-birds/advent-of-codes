impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        for num in &nums {
            if num % 2 == 0 {
                result.push(*num);
            }
        }
        for num in &nums {
            if num % 2 != 0 {
                result.push(*num);
            }
        }

        result
    }
}
