impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        // inspiration : https://assets.leetcode.com/users/images/6fed1231-e63e-45e6-acc7-a22dbca7664e_1594507497.278275.png
        if nums.len() <= 3 {
            return 0;
        }
        let mut temp_nums: Vec<i32> = nums.to_vec();
        temp_nums.sort_by(|a, b| a.cmp(b));
        // println!("sorted nums: {:?}", temp_nums);
        // so it sorted first, then we just move the window starting from the last 3 numbers to the first three numbers,
        //
        // [0, 1, 2, 3, 4, 5] --> MUST BE SORTED
        // ---> . . . 3 4 5
        // ---> 0 . . . 4 5
        // ---> 0 1 . . . 5
        // ---> 0 1 3 . . .
        // then find the max and min that re not in the window, then max - min

        let mut end: usize = nums.len() - 3;
        let mut start: usize = 0;
        let mut min_diff: i32 = i32::MAX;
        while end <= temp_nums.len() {
            // println!("start:{},end:{}", start, end); // already SORTED
            let diff: i32 = temp_nums[end - 1] - temp_nums[start];
            if min_diff > diff {
                min_diff = diff;
            }
            end += 1;
            start += 1;
        }
        min_diff
    }
}
