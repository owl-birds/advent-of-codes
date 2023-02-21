impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        // 
        if nums.len() == 1{return nums[0];}

        let mut left = 0;
        let mut left_num = nums[left];
        let mut count_left = 0;

        let mut right = nums.len() - 1;
        let mut right_num = nums[right];
        let mut count_right = 0;

        while left <= right{
            if left_num == nums[left]{
                count_left += 1;
            }else{
                if count_left == 1{return left_num;}
                count_left = 1;
                left_num = nums[left];
            }
            if right_num == nums[right]{
                count_right += 1;
            }else{
                if count_right == 1{return right_num;}
                count_right = 1;
                right_num = nums[right];
            }
            left += 1;
            right -= 1;
        }
        return left_num; // if only one num, or it arrived in the middle of the array
    }
}
