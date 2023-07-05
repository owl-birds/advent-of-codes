impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut one_length: Vec<i32> = vec![];
        // [ 0, 3, 0, 0, 5 ] ::: 3 : the arr_1 length
        // ex: [0,1,1,1,0,0,1,1]
        // :: [0,3,0,0,2]
        let mut idx: usize = 0;

        while idx < nums.len() {
            let mut count_one: i32 = 0;
            while idx < nums.len() && nums[idx] == 1 {
                count_one += 1;
                idx += 1;
            }
            if count_one > 0 {
                one_length.push(count_one);
                continue;
            }
            one_length.push(0);
            idx += 1;
        }
        // println!("{:?}", one_length);
        if one_length.len() == 1 && one_length[0] != 0 {
            return one_length[0] - 1;
        }
        let mut max_length: i32 = 0;
        for i in 0..one_length.len() {
            if i == 0 {
                if one_length[i + 1] > max_length {
                    max_length = one_length[i + 1]
                }
                continue;
            }
            if i == one_length.len() - 1 {
                if one_length[i - 1] > max_length {
                    max_length = one_length[i - 1]
                }
                continue;
            }
            let temp_length: i32 = one_length[i - 1] + one_length[i + 1];
            if temp_length > max_length {
                max_length = temp_length
            }
        }
        max_length
    }
}
