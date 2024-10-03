impl Solution {
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut temp: Vec<i32> = nums.to_vec();

        for i in 0..temp.len() - 1 {
            if temp[i] == temp[i + 1] {
                temp[i] *= 2;
                temp[i + 1] = 0;
            }
        }
        let mut result: Vec<i32> = vec![];
        for i in 0..temp.len() {
            if temp[i] != 0 {
                result.push(temp[i]);
            }
        }
        for i in 0..(temp.len() - result.len()) {
            result.push(0);
        }

        result
    }
}
