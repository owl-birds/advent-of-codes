impl Solution {
    pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        if nums.len() % 3 != 0 || nums.len() < 3 {
            return vec![];
        }
        let mut result: Vec<Vec<i32>> = vec![];
        let mut temp: Vec<i32> = nums.to_vec();
        temp.sort_by(|a, b| b.cmp(&a));
        let mut idx: usize = 0;
        // println!("{:?}", temp);

        while idx < temp.len() {
            if temp[idx] - temp[idx + 2] > k {
                return vec![];
            }
            let mut temp_vec: Vec<i32> = vec![];
            let mut temp_idx: usize = idx;
            while temp_idx < idx + 3 {
                temp_vec.push(temp[temp_idx]);
                temp_idx += 1;
            }
            idx = temp_idx;
            result.push(temp_vec);
        }

        result
    }
}
